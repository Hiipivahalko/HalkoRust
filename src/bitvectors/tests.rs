use rand::{Rng, thread_rng};
//use rand::seq::index;
use crate::bitvectors::Bitvector;

#[test]
fn empty_bitvector_full_of_zeros() {
    let n: usize = 10;

    let bv = Bitvector::build_empty(n);

    for i in 0..n {
        assert_eq!(bv[i], 0);
    }
}

#[test]
fn empty_bitvector_full_of_zeros_rand_size() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(50..=100);//rng.gen_range(50..100);

    let bv = Bitvector::build_empty(n);

    for i in 0..n {
        assert_eq!(bv[i], 0);
    }
}

#[test]
fn small_bitvector_mutate() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    let arr = [1, 5, 7];
    for i in 0..arr.len() {
        bv[arr[i]] = 1;
    }

    for i in 0..n {
        if arr.contains(&i) {
            assert_eq!(bv[i], 1);
        } else {
            assert_eq!(bv[i], 0);
        }

    }
}

#[test]
fn small_bitvector_mutate_random() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(50..=100);//rng.gen_range(50..100);
    let mut bv = Bitvector::build_empty(n);

    //let mut i_vec = index::sample(&rng, n, 5);
    let mut arr: [usize; 5] = [0, 0, 0, 0, 0];
    for i in 0..5 {
        arr[i] = rng.gen_range(0..n);
    }

    for i in arr {
        bv[i] = 1;
    }

    for i in 0..n {
        if arr.contains(&i) {
            assert_eq!(bv[i], 1);
        } else {
            assert_eq!(bv[i], 0);
        }

    }
}

#[test]
fn all_ones() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    for i in 0..n {
        bv[i] = 1;
    }

    for i in 0..n {
        assert_eq!(bv[i], 1);
    }
}

// first change all 0s -> 1
// then change all 1s -> 0
#[test]
fn all_ones_first_then_back_zeros() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    for i in 0..n {
        bv[i] = 1;
    }
    for i in 0..n {
        bv[i] = 0;
    }

    for i in 0..n {
        assert_eq!(bv[i], 0);
    }
}
