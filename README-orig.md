# Multithread

Multithread is a 'C' and 'G' counting program.

## Build

```sh
cargo build --release
```

## Usage

```sh
./target/release/multithread-rs file number_of_threads
```

The file can be generated with the following command. WARNING, this will
create a 3 GB file at /tmp/acgt.

```sh
./target/release/gen
```

### Example usage

As I have a 12 threads CPU. Replace with how much you have.

```sh
cargo build --release
./target/release/gen
./target/release/multithread-rs /tmp/acgt 12
```