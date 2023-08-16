use rand::{Rng, thread_rng};
use std::vec::Vec;
use crate::bitvectors::Bitvector;

// fn: build_empty
#[test]
fn build_empty_bitvector_full_of_zeros() {
    let n: usize = 10;

    let bv = Bitvector::build_empty(n);

    for i in 0..n {
        assert_eq!(bv.get(i), 0);
    }
}

// fn: build_empty
#[test]
fn build_empty_bitvector_full_of_zeros_rand_size() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(200..=300);

    let bv = Bitvector::build_empty(n);

    for i in 0..n {
        assert_eq!(bv.get(i), 0);
    }
}

// fn: build_from_vec
#[test]
fn build_from_vec_zeros() {
    let v: Vec<u64> = vec![0,0,0];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(0, bv.get(i));
    }
    assert_eq!(bv.len(), 3*64);
}
//
// fn: build_from_vec
#[test]
fn build_from_vec_ones() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX,u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(1, bv.get(i));
    }
    assert_eq!(bv.len(), 3*64);
}

// fn: build_from_vec
#[test]
fn build_from_vec_small() {
    let v: Vec<u64> = vec![0,3920,234,940,1,4345,0,2,12324];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..v.len() {
        assert_eq!(bv[i], v[i]);
    }
    for i in 0..64 {
        assert_eq!(bv.get(i), 0);
    }
    for i in 64*6..64*7 {
        assert_eq!(bv.get(i), 0);
    }
    assert_eq!(bv.get(4*64), 1);
    assert_eq!(bv.get(7*64+1), 1);
    assert_eq!(bv.len(), v.len()*64);
}

// fn: build_from_vec
#[test]
fn build_from_vec_random1() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(1000..=2000);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..v.len() {
        assert_eq!(bv[i], v[i]);
    }
    assert_eq!(bv.len(), v.len()*64);
}

// fn: build_from_vec
#[test]
fn build_from_vec_random2() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(2000..=3000);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..v.len() {
        assert_eq!(bv[i], v[i]);
    }
    assert_eq!(bv.len(), v.len()*64);
}

// fn: set, get
#[test]
fn set_get_fn_small_bitvector_mutate() {
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

// fn: set, get
#[test]
fn set_get_fn_small_bitvector_mutate_random() {
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

// fn: rank1
#[test]
fn rank1_simple() {
    let a: [u64; 7] = [0,1,0,0,1,1,0];
    let bv = Bitvector::build(&a);

    let result: [u64; 7] = [0,1,1,1,2,3,3];

    for i in 0..7 {
        assert_eq!(
            bv.rank1(i),
            result[i],
            "\n>> Error at index: {},\nrank1(i,bv): {},\ncorrect: {}\n",
            i,
            bv.rank1(i),
            result[i]
        );
    }
}

// fn: rank1
#[test]
fn rank1_zeros() {
    let bv = Bitvector::build_empty(300);

    for i in 0..bv.len() {
        assert_eq!(
            bv.rank1(i),
            0,
            "\n>> Error at index: {},\nrank1(i,bv): {},\ncorrect: {}\n",
            i,
            bv.rank1(i),
            0
        );
    }
}

// fn: rank1
#[test]
fn rank1_ones() {
    let mut bv = Bitvector::build_empty(300);
    for i in 0..bv.len() {
        bv.set(i, 1);
    }

    for i in 0..bv.len() {
        let j = i as u64;
        let rank1 = bv.rank1(i);
        assert_eq!(
            rank1,
            j+1,
            "\n>> Error at index: {},\nrank1(i,bv): {},\ncorrect: {}\n",
            i,
            rank1,
            j+1
        );
    }
}

// fn: rank1
#[test]
fn rank1_random1() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(10..=20);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut result: Vec<u64> = vec![0; n*64];
    let mut ones = 0;
    assert_eq!(bv.len(), result.len());
    for k in 0..bv.len() {
        if bv.get(k) == 1 {
            ones += 1;
        }
        result[k] = ones;
    }

    for k in 0..bv.len() {
        assert_eq!(
            bv.rank1(k),
            result[k],
            "\n>> Error at index: {},\nrank1(i,bv): {},\ncorrect: {}\n",
            k,
            bv.rank1(k),
            result[k]
        );
    }
}

// fn: rank1
#[test]
fn rank1_random2() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(50..=60);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut result: Vec<u64> = vec![0; n*64];
    let mut ones = 0;
    assert_eq!(bv.len(), result.len());
    for k in 0..bv.len() {
        if bv.get(k) == 1 {
            ones += 1;
        }
        result[k] = ones;
    }

    for k in 0..bv.len() {
        assert_eq!(
            bv.rank1(k),
            result[k],
            "\n>> Error at index: {},\nrank1(i,bv): {},\ncorrect: {}\n",
            k,
            bv.rank1(k),
            result[k]
        );
    }
}
