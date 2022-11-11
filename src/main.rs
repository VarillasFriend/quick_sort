use quick_sort::*;
use rand::Rng;
use rand::distributions::Uniform;

fn main() {
    let range = Uniform::from(0..100_000_000);
    let values: Vec<u32> = rand::thread_rng().sample_iter(&range).take(100_000_000).collect();

    let val = sort(&values);
}
