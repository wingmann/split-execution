use crossbeam;
use rand::Rng;
use std::io::{stdin, stdout, Read, Write};
use std::time::SystemTime;

fn main() {
    execute();
    pause();
}

fn execute() {
    const CAPACITY: usize = 1_000_000;
    const RANGE: (i64, i64) = (0, 10_000);
    const THREADS_COUNT: usize = 8;

    println!("Program is run.\nInitializing vector. Wait...");

    let mut vec = random_vec(CAPACITY, RANGE);
    // _print_values(&vec, "\nRaw list:");
    let start = SystemTime::now();

    parallel_sort(&mut vec, THREADS_COUNT);

    let stop = SystemTime::now();
    // _print_values(&vec, "\nSorted list:");

    let elapsed_time = stop.duration_since(start).unwrap();
    let millis = elapsed_time.as_millis();
    println!(
        "Vector of 1 Million 64-bit integers (0..10K). \
         Sorting time: {} milliseconds. [8 threads]",
        millis
    );
}

// Filling the vector with random numbers.
fn random_vec(capacity: usize, range: (i64, i64)) -> Vec<i64> {
    let mut vec = vec![0; capacity];
    for i in vec.iter_mut() {
        *i = rand::thread_rng().gen_range(range.0..=range.1);
    }
    vec
}

fn parallel_sort(data: &mut [i64], threads_count: usize) {
    const MIDDLE_STEP: usize = 1000;
    let data_length = data.len();

    if data.is_empty() || data.len() == 1 {
        return;
    }

    if data_length <= MIDDLE_STEP {
        data.sort();
    } else {
        let chunks = std::cmp::min(data.len(), threads_count);
        let _ = crossbeam::scope(|scope| {
            for slice in data.chunks_mut(data.len() / chunks) {
                scope.spawn(move |_| slice.sort());
            }
        });
        // Merge result.
        data.sort();
    }
}

// Outputs a column of numbers from a vector. (10xN)
fn _print_values(vec: &Vec<i64>, message: &str) {
    let mut count = 0;
    println!("{}", message);
    for i in vec.iter() {
        if count == 10 {
            count = 0;
            println!();
        }
        print!("{:5} ", i);
        count += 1;
    }
    println!("\n");
}

fn pause() {
    print!("\nPress Enter to continue...");
    stdout().flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let mut vec = Vec::<i64>::new();
        parallel_sort(&mut vec, 8);
    }

    #[test]
    fn one_element() {
        let mut vec = vec![30];
        parallel_sort(&mut vec, 8);
    }
}
