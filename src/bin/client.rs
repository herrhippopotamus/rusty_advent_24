// use crate::day_1::day_1;
use aoc::day_1::day_1;
use std::time::Instant;

fn main() {
    let runs = 1000;
    let mut runtime = std::time::Duration::new(0, 0);
    for run in 0..runs {
        let start = Instant::now();
        let (d10, d11) = day_1().unwrap();
        let elapsed = start.elapsed();
        runtime += elapsed;

        println!(
            "run[{} / {}]: day 1 - part 1: {}, part 2: {} - took: {:?}",
            run, runs, d10, d11, elapsed
        );
    }
    println!("Average time for {} runs: {:?}", runs, runtime / runs);
}
