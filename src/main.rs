use std::time::Instant;

use b64_bench::bench;

fn main() {
    let now = Instant::now();
    let mut len = 0_usize;

    for _ in 0..100 {
        len += bench(10000, 1000);
    }

    println!("{:?}, time: {:?}", len, Instant::now() - now);
}
