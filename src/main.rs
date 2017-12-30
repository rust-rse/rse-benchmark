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
    let mut shards = make_random_shards!(per_shard, data_shards + parity_shards);
    //let mut shards = make_blank_shards(per_shard, data_shards + parity_shards);
    let r = ReedSolomon::with_pparam(data_shards, parity_shards, pparam);

    let start = time::precise_time_ns();
    for _ in 0..iterations {
        r.encode_shards(&mut shards).unwrap();
        //assert!(r.verify_shards(&shards).unwrap());
    }
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    let byte_count = (iterations * per_shard * data_shards) as f64;
    println!("encode :");
    println!("    shards           : {} / {}", data_shards, parity_shards);
    println!("    shard length     : {}", per_shard);
    println!("    bytes per encode : {}", pparam.bytes_per_encode);
    println!("    time taken       : {}", time_taken);
    println!("    byte count       : {}", byte_count);
    println!("    MB/s             : {}", byte_count / 1_000_000.0 / time_taken);
}

fn benchmark_encode_inplace(iterations    : usize,
                            //data_shards   : usize,
                            //parity_shards : usize,
                            //per_shard     : usize,
                            pparam        : ParallelParam) {
    const DATA_SHARDS   : usize = 5;
    const PARITY_SHARDS : usize = 2;
    const PER_SHARD     : usize = 1_000_000;
    //let mut shards = make_random_shards!(per_shard, data_shards + parity_shards);
    //let mut shards = make_blank_shards(per_shard, data_shards + parity_shards);
    let mut slices : [[u8; PER_SHARD]; DATA_SHARDS + PARITY_SHARDS] =
        [[0; PER_SHARD]; DATA_SHARDS + PARITY_SHARDS];
    let r = ReedSolomon::with_pparam(DATA_SHARDS, PARITY_SHARDS, pparam);

    let mut slices_ref : Vec<&mut [u8]> =
        Vec::with_capacity(DATA_SHARDS + PARITY_SHARDS);
    for slice in slices.iter_mut() {
        slices_ref.push(slice);
    }

    let start = time::precise_time_ns();
    for _ in 0..iterations {
        r.encode(&mut slices_ref).unwrap();
        //assert!(r.verify_shards(&shards).unwrap());
    }
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    let byte_count = (iterations * PER_SHARD * DATA_SHARDS) as f64;
    println!("encode inplace :");
    println!("    shards           : {} / {}", DATA_SHARDS, PARITY_SHARDS);
    println!("    shard length     : {}", PER_SHARD);
    println!("    bytes per encode : {}", pparam.bytes_per_encode);
    println!("    time taken       : {}", time_taken);
    println!("    byte count       : {}", byte_count);
    println!("    MB/s             : {}", byte_count / 1_000_000.0 / time_taken);
}

fn benchmark_verify(iterations    : usize,
                    data_shards   : usize,
                    parity_shards : usize,
                    per_shard     : usize,
                    pparam        : ParallelParam) {
    let mut shards = make_random_shards!(per_shard, data_shards + parity_shards);
    //let mut shards = make_blank_shards(per_shard, data_shards + parity_shards);
    let r = ReedSolomon::with_pparam(data_shards, parity_shards, pparam);

    r.encode_shards(&mut shards).unwrap();

    let start = time::precise_time_ns();
    for _ in 0..iterations {
        r.verify_shards(&shards).unwrap();
    }
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    let byte_count = (iterations * per_shard * (data_shards + parity_shards)) as f64;
    println!("verify :");
    println!("    shards           : {} / {}", data_shards, parity_shards);
    println!("    shard length     : {}", per_shard);
    println!("    bytes per encode : {}", pparam.bytes_per_encode);
    println!("    time taken       : {}", time_taken);
    println!("    byte count       : {}", byte_count);
    println!("    MB/s             : {}", byte_count / 1_000_000.0 / time_taken);
}

fn benchmark_reconstruct(iterations    : usize,
                         data_shards   : usize,
                         parity_shards : usize,
                         per_shard     : usize,
                         pparam        : ParallelParam) {
    let mut shards = make_random_shards!(per_shard, data_shards + parity_shards);
    //let mut shards = make_blank_shards(per_shard, data_shards + parity_shards);
    let r = ReedSolomon::with_pparam(data_shards, parity_shards, pparam);

    r.encode_shards(&mut shards).unwrap();

    let mut shards = shards_into_option_shards(shards);

    let start = time::precise_time_ns();
    for _ in 0..iterations {
        shards[0] = None;
        r.reconstruct_shards(&mut shards).unwrap();
    }
    let end   = time::precise_time_ns();
    let time_taken = (end - start) as f64 / 1_000_000_000.0;
    let byte_count = (iterations * per_shard * 1) as f64;
    println!("reconstruct :");
    println!("    shards           : {} / {}", data_shards, parity_shards);
    println!("    shard length     : {}", per_shard);
    println!("    bytes per encode : {}", pparam.bytes_per_encode);
    println!("    time taken       : {}", time_taken);
    println!("    byte count       : {}", byte_count);
    println!("    MB/s             : {}", byte_count / 1_000_000.0 / time_taken);
}

fn main() {
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(1024));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(2048));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(4096));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(16384));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(32768));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(65536));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(10485760));
    /*println!("=====");
    benchmark_encode_inplace(500, ParallelParam::new(1024));
    benchmark_encode_inplace(500, ParallelParam::new(2048));
    benchmark_encode_inplace(500, ParallelParam::new(4096));
    benchmark_encode_inplace(500, ParallelParam::new(8192));
    benchmark_encode_inplace(500, ParallelParam::new(16384));
    benchmark_encode_inplace(500, ParallelParam::new(32768));
    benchmark_encode_inplace(500, ParallelParam::new(65536));
    benchmark_encode_inplace(500, ParallelParam::new(10485760));*/
    println!("=====");
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(1024));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(2048));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(4096));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(16384));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(32768));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(65536));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(10485760));
    println!("=====");
    benchmark_encode(500, 10, 1, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 3, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(8192));
    println!("=====");
    benchmark_encode(500, 3, 1, 496, ParallelParam::new(1024));
    benchmark_encode(500, 5, 1, 496, ParallelParam::new(1024));
    benchmark_encode(500, 7, 1, 496, ParallelParam::new(1024));
    benchmark_encode(500, 9, 1, 496, ParallelParam::new(1024));
    println!("=====");
    benchmark_encode(500, 10, 1, 496, ParallelParam::new(1024));
    benchmark_encode(500, 10, 2, 496, ParallelParam::new(1024));
    benchmark_encode(500, 10, 3, 496, ParallelParam::new(1024));
    benchmark_encode(500, 10, 4, 496, ParallelParam::new(1024));
    println!("=====");
    benchmark_encode(500, 10, 1, 4096, ParallelParam::new(1024));
    benchmark_encode(500, 10, 2, 4096, ParallelParam::new(1024));
    benchmark_encode(500, 10, 3, 4096, ParallelParam::new(1024));
    benchmark_encode(500, 10, 4, 4096, ParallelParam::new(1024));
    println!("=====");
    benchmark_encode(500, 10, 1, 4096, ParallelParam::new(8192));
    benchmark_encode(500, 10, 2, 4096, ParallelParam::new(8192));
    benchmark_encode(500, 10, 3, 4096, ParallelParam::new(8192));
    benchmark_encode(500, 10, 4, 4096, ParallelParam::new(8192));
    println!("=====");
    benchmark_encode(500, 3, 2, 496, ParallelParam::new(1024));
    benchmark_encode(500, 5, 2, 496, ParallelParam::new(1024));
    benchmark_encode(500, 7, 2, 496, ParallelParam::new(1024));
    benchmark_encode(500, 9, 2, 496, ParallelParam::new(1024));
    println!("=====");
    benchmark_encode(500, 3,     1, 496, ParallelParam::new(8192));
    benchmark_encode(500, 5,     1, 496, ParallelParam::new(8192));
    benchmark_encode(500, 10,     1, 496, ParallelParam::new(8192));
    println!("=====");
    benchmark_encode(500, 10, 1, 496, ParallelParam::new(500));
    benchmark_encode(500, 3, 2, 496, ParallelParam::new(500));
    benchmark_encode(500, 10, 3, 496, ParallelParam::new(500));
    println!("=====");
    benchmark_encode(500, 10, 2, 10_000, ParallelParam::new(8192));
    benchmark_encode(500, 100, 20, 10_000, ParallelParam::new(8192));
    benchmark_encode(500, 17, 3, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(50, 10, 4, 16_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 5, 2, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 4, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 50, 20, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(50, 17, 3, 16_000_000, ParallelParam::new(8192));
    println!("=====");
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(1024));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(4096));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(8192));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(16384));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(32768));
    benchmark_encode(500, 10, 2, 1_000_000, ParallelParam::new(65536));
}
