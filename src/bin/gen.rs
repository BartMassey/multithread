use rand::{
    distributions::{Distribution, Standard},
    prelude::SmallRng,
    Rng, SeedableRng,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

enum ACGT {
    A,
    C,
    G,
    T,
}

impl ACGT {
    fn get(&self) -> u8 {
        match self {
            ACGT::A => b'A',
            ACGT::C => b'C',
            ACGT::G => b'G',
            ACGT::T => b'T',
        }
    }
}

impl Distribution<ACGT> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ACGT {
        match rng.gen_range(0, 4) {
            0 => ACGT::A,
            1 => ACGT::C,
            2 => ACGT::G,
            3 => ACGT::T,
            _ => unreachable!(),
        }
    }
}

const THREE_GB: usize = 3_221_225_000;

fn main() {
    let mut writer = {
        let file = File::create("/tmp/acgt").unwrap();
        BufWriter::new(file)
    };

    let rng = SmallRng::from_entropy();

    for acgt in rng.sample_iter::<ACGT, _>(Standard).take(THREE_GB) {
        writer.write_all(&[acgt.get()]).unwrap();
    }
}
