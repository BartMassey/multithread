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
    if thread_count < 1 {
        eprintln!("Thread count should be greater than 0.");
        exit(2);
    }

    let len = file.metadata().unwrap().len();
    let reader = BufReader::new(file);
    let data = read_all(reader, len, thread_count);

    let start = Instant::now();
    let gc_count = count_gc(data, thread_count);
    let end = Instant::now();

    println!("GC count = {}", gc_count);

    let ratio = gc_count as f64 / len as f64;
    println!("GC ratio = {}", ratio);

    let duration = end - start;
    println!("Count duration = {:?}", duration);
}

fn count_gc(mut data: Vec<Vec<u8>>, thread_count: u8) -> u128 {
    let mut gc = 0;

    let mut threads = Vec::with_capacity(thread_count as usize);
    // Spawn a thread for each chunk to count the number of G and C
    for _ in 0..thread_count {
        let chunk = data.pop().unwrap();

        threads.push(thread::spawn(move || {
            chunk.iter().filter(|&&b| b == b'C' || b == b'G').count() as u128
        }));
    }

    // When the length of the file cannot be divided by the number of thread,
    // there is a bit of data left to count
    if let Some(chunk) = data.pop() {
        gc += chunk.iter().filter(|&&b| b == b'C' || b == b'G').count() as u128;
    };

    for t in threads {
        gc += t.join().unwrap();
    }

    gc
}

// Transform the file data into a vec of vec, each thread will then consume a vec
fn read_all(mut reader: BufReader<File>, len: u64, thread_count: u8) -> Vec<Vec<u8>> {
    let mut data = Vec::with_capacity(thread_count as usize + 1);
    let thread_len = len / thread_count as u64;

    // Prepare the data for each thread
    for _ in 0..thread_count {
        let mut chunk = Vec::with_capacity(thread_len as usize);
        reader
            .by_ref()
            .take(thread_len)
            .read_to_end(&mut chunk)
            .unwrap();
        data.push(chunk);
    }

    // If there is still data in the file put it in a vec to be pop in last
    if thread_len * thread_count as u64 != len {
        let mut chunk = Vec::with_capacity((len - thread_len * thread_count as u64) as usize);
        reader.read_to_end(&mut chunk).unwrap();
        data.insert(0, chunk);
    }

    data
}
