use b64_bench::bench;
use std::time::Instant;

fn main() {
    let start = Instant::now();
    let mut len = 0_usize;

    for _ in 0..100 {
        len += bench(10000, 1000);
    }

    println!("{:?}, time: {:?}", len, Instant::now() - start);
}
