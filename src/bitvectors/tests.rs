use rand::{Rng, thread_rng};
//use rand::seq::index;
use crate::bitvectors::Bitvector;

#[test]
fn empty_bitvector_full_of_zeros() {
    let n: usize = 10;

    let bv = Bitvector::build_empty(n);

    for i in 0..n {
        assert_eq!(bv.get(i), 0);
    }
}

#[test]
fn empty_bitvector_full_of_zeros_rand_size() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(200..=300);

    let bv = Bitvector::build_empty(n);

    for i in 0..n {
        assert_eq!(bv.get(i), 0);
    }
}

#[test]
fn small_bitvector_mutate() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    let arr = [1, 5, 7];
    for i in 0..arr.len() {
        bv.set(arr[i], 1);
    }

    for i in 0..n {
        if arr.contains(&i) {
            assert_eq!(bv.get(i), 1);
        } else {
            assert_eq!(bv.get(i), 0);
        }

    }
}

#[test]
fn small_bitvector_mutate_random() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(200..=300);
    let mut bv = Bitvector::build_empty(n);

    //let mut i_vec = index::sample(&rng, n, 5);
    let mut arr: [usize; 5] = [0, 0, 0, 0, 0];
    for i in 0..5 {
        arr[i] = rng.gen_range(0..n);
    }

    for i in arr {
        bv.set(i,1);
    }

    for i in 0..n {
        if arr.contains(&i) {
            assert_eq!(bv.get(i), 1);
        } else {
            assert_eq!(bv.get(i), 0);
        }

    }
}

#[test]
fn all_ones() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    for i in 0..n {
        bv.set(i,1);
    }

    for i in 0..n {
        assert_eq!(bv.get(i), 1);
    }
}

// first change all 0s -> 1
// then change all 1s -> 0
#[test]
fn all_ones_first_then_back_zeros() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    for i in 0..n {
        bv.set(i,1);
    }
    for i in 0..n {
        bv.set(i, 0);
    }

    for i in 0..n {
        assert_eq!(
            bv.get(i),
            0,
            "\n>> Error at index: {}, value: {}\n",
            i,
            bv.get(i)
        );
    }
}

#[test]
fn bitvector_lenght() {
    let n: usize = 10;
    let bv = Bitvector::build_empty(n);

    assert_eq!(bv.len(), n);
}

#[test]
fn build_bitvector_from_array1() {
    let a: [u64; 7] = [0,1,0,0,1,1,0];
    let bv = Bitvector::build(&a);

    for i in 0..a.len() {
        assert_eq!(
            bv.get(i),
            a[i],
            "\n>> Error at index: {},\nbv: {},\na: {}\n",
            i,
            bv.get(i),
            a[i]
        );
    }
}

#[test]
fn build_bitvector_from_array2() {
    let a: [u64; 5] = [1,1,1,1,1];
    let bv = Bitvector::build(&a);

    for i in 0..a.len() {
        assert_eq!(
            bv.get(i),
            a[i],
            "\n>> Error at index: {},\nbv: {},\na: {}\n",
            i,
            bv.get(i),
            a[i]
        );
    }
}

#[test]
fn build_bitvector_from_array_random() {
    let mut rng = thread_rng();

    const N: usize = 300;
    let mut a: [u64; N] = [0; N];
    for _i in 0..75 {
        let x: usize = rng.gen_range(0..N);
        a[x] = 1;
    }
    let bv = Bitvector::build(&a);

    for i in 0..a.len() {
        assert_eq!(
            bv.get(i),
            a[i],
            "\n>> Error at index: {},\nbv: {},\na: {}\n",
            i,
            bv.get(i),
            a[i]
        );
    }
}
