use std::{
    env::args,
    fs::File,
    io::{BufReader, Read},
    process::exit,
    thread,
    time::Instant,
};

fn main() {
    let mut args = args().skip(1);
    if args.len() < 2 {
        eprintln!("Not enough arguments.");
        exit(1);
    }

    let file = File::open(args.next().unwrap()).unwrap();
    let thread_count = args.next().unwrap().parse().unwrap();
    let len = file.metadata().unwrap().len() as usize;
    let reader = BufReader::new(file);
    let (start, end, gc_count) =
        if thread_count == 0 {
            let mut data = read_all(reader, len, 1);
            assert_eq!(data.len(), 1);
            let start = Instant::now();
            let gc_count = count_gc_seq(data.pop().unwrap());
            let end = Instant::now();
            (start, end, gc_count)
        } else {
            let data = read_all(reader, len, thread_count);
            let start = Instant::now();
            let gc_count = count_gc(data, thread_count);
            let end = Instant::now();
            (start, end, gc_count)
        };

    println!("GC count = {}", gc_count);

    let ratio = gc_count as f64 / len as f64;
    println!("GC ratio = {} ({}/{})", ratio, gc_count, len);

    let duration = end - start;
    println!("Count duration = {:?}", duration);
}

fn count_gc(mut data: Vec<Vec<u8>>, thread_count: usize) -> u128 {
    let mut gc = 0;

    let ndata = data.len();
    assert_eq!(thread_count, ndata);
    let mut threads = Vec::with_capacity(ndata);
    // Spawn a thread for each chunk to count the number of G and C
    for _ in 0..thread_count {
        let chunk = data.pop().unwrap();

        threads.push(thread::spawn(move || {
            chunk.iter().filter(|&&b| b == b'C' || b == b'G').count() as u128
        }));
    }

    for t in threads {
        gc += t.join().unwrap();
    }

    gc
}

#[inline(never)]
fn count_gc_seq(data: Vec<u8>) -> u128 {
    data.iter().filter(|&&b| b == b'C' || b == b'G').count() as u128
}

// Transform the file data into a vec of vec, each thread will then consume a vec
fn read_all(mut reader: BufReader<File>, len: usize, thread_count: usize) -> Vec<Vec<u8>> {
    let mut data = Vec::with_capacity(thread_count);
    let thread_len = (len as f64 / thread_count as f64).ceil() as usize;

    // Prepare the data for each thread
    for _ in 0..thread_count {
        let mut chunk = Vec::with_capacity(thread_len);
        reader
            .by_ref()
            .take(thread_len as u64)
            .read_to_end(&mut chunk)
            .unwrap();
        data.push(chunk);
    }

    data
}
