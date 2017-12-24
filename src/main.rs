extern crate rand;
extern crate reed_solomon_erasure;
extern crate time;

//use self::rand::{thread_rng, Rng};
//use std::rc::Rc;

use reed_solomon_erasure::*;

macro_rules! make_random_shards {
    ($per_shard:expr, $size:expr) => {{
        let mut shards = Vec::with_capacity(13);
        for _ in 0..$size {
            shards.push(make_blank_shard($per_shard));
        }

        for s in shards.iter_mut() {
            fill_random(s);
        }

        shards
    }}
}

fn fill_random(arr : &mut Shard) {
    for a in arr.iter_mut() {
        *a = rand::random::<u8>();
    }
}

fn benchmark_encode(iterations    : usize,
                    data_shards   : usize,
                    parity_shards : usize,
                    per_shard     : usize,
                    pparam        : ParallelParam) {
    //let mut shards = make_random_shards!(per_shard, data_shards + parity_shards);
    let mut shards = make_blank_shards(per_shard, data_shards + parity_shards);
    let r = ReedSolomon::with_pparam(data_shards, parity_shards, pparam);

    let start = time::precise_time_ns();
    for _ in 0..iterations {
        r.encode_shards(&mut shards).unwrap();
    }
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    let byte_count = (iterations * per_shard * data_shards) as f64;
    println!("time taken : {}", time_taken);
    println!("byte count : {}", byte_count);
    println!("MB/s : {}", byte_count / 1_000_000.0 / time_taken);
}

fn main() {
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(1024,  10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(2048,  10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(4096,  10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(8192,  10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(16384, 10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(32768, 10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(65536, 10));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(10485760, 10));
    println!("=====");

    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(1024,  10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(2048,  10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(4096,  10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(8192,  10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(16384, 10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(32768, 10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(65536, 10));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(10485760, 10));
    println!("=====");
    benchmark_encode(500, 100, 20, 496, ParallelParam::new(200,  10));
}
