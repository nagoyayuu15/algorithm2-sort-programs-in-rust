
mod combsort;
mod heapsort;
mod mergesort;
mod quicksort;
mod shellsort;

use rand::{distributions::Uniform, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 100);
    let v: Vec<u32> = (0..100).map(|_| rng.sample(&range)).collect();

    let mut targ = v.clone();
    heapsort::heapsort(&mut targ);
    println!("{:?}", targ);
    let mut targ = v.clone();
    combsort::combsort(&mut targ);
    println!("{:?}", targ);
    let mut targ = v.clone();
    mergesort::mergesort(&mut targ);
    println!("{:?}", targ);
    let mut targ = v.clone();
    quicksort::quicksort(&mut targ);
    println!("{:?}", targ);
    let mut targ = v.clone();
    shellsort::shellsort(&mut targ);
    println!("{:?}", targ);
}
