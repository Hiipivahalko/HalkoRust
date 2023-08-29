use std::vec::Vec;
use rand::{Rng, thread_rng};

use crate::rank_support::RankSupport;
use crate::bitvectors::Bitvector;


// fn: new
#[test]
fn new_bv64_ones() {

    let v = vec![u64::MAX; 1];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![36];
    let res_level2: Vec<u64> = vec![
                                    6,12,18,24,30,
                                    6,12,18,24,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv128_ones() {

    let v = vec![u64::MAX; 2];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![49,98];
    let res_level2: Vec<u64> = vec![
                                    7,14,21,28,35,42,
                                    7,14,21,28,35,42,
                                    7,14,21,28,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv192_ones() {

    let v = vec![u64::MAX; 3];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![49,98,147];
    let res_level2: Vec<u64> = vec![
                                    7,14,21,28,35,42,
                                    7,14,21,28,35,42,
                                    7,14,21,28,35,42,
                                    7,14,21,28,35,42
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv256_ones() {

    let v = vec![u64::MAX; 4];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![64, 128, 192, 256];
    let res_level2: Vec<u64> = vec![8, 16, 24, 32, 40, 48, 56,
                                    8, 16, 24, 32, 40, 48, 56,
                                    8, 16, 24, 32, 40, 48, 56,
                                    8, 16, 24, 32, 40, 48, 56];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv64_zeros() {

    let v = vec![0; 1];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![0];
    let res_level2: Vec<u64> = vec![
                                    0,0,0,0,0,
                                    0,0,0,0,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv128_zeros() {

    let v = vec![0; 2];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![0,0];
    let res_level2: Vec<u64> = vec![
                                    0,0,0,0,0,0,
                                    0,0,0,0,0,0,
                                    0,0,0,0,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv192_zeros() {

    let v = vec![0; 3];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![0,0,0];
    let res_level2: Vec<u64> = vec![
                                    0,0,0,0,0,0,
                                    0,0,0,0,0,0,
                                    0,0,0,0,0,0,
                                    0,0,0,0,0,0,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv256_zeros() {

    let v = vec![0; 4];

    let rs = RankSupport::new(Bitvector::build_from_vec(&v));

    let res_level1: Vec<u64> = vec![0,0,0,0];
    let res_level2: Vec<u64> = vec![
                                    0,0,0,0,0,0,0,
                                    0,0,0,0,0,0,0,
                                    0,0,0,0,0,0,0,
                                    0,0,0,0,0,0,0,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv64_mixed() {

    let mut a = [0; 64];
    let b1_size = 36;
    for i in 0..b1_size { a[i] = 1; }
    //b2_size = 6;

    let rs = RankSupport::new(Bitvector::build(&a));

    let res_level1: Vec<u64> = vec![36];
    let res_level2: Vec<u64> = vec![
                                    6,12,18,24,30,
                                    0,0,0,0,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv128_mixed() {

    let mut a = [0; 128];
    let b1_size = 49;
    for i in 0..b1_size { a[i] = 1; }
    for i in 2*b1_size..a.len() { a[i] = 1; }
    //b2_size = 6;

    let rs = RankSupport::new(Bitvector::build(&a));

    let res_level1: Vec<u64> = vec![49,49];
    let res_level2: Vec<u64> = vec![
                                    7,14,21,28,35,42,
                                    0,0,0,0,0,0,
                                    7,14,21,28,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv192_mixed() {

    let mut a = [0; 192];
    let b1_size = 49;
    for i in 0..b1_size { a[i] = 1; }
    for i in 2*b1_size..3*b1_size { a[i] = 1; }
    //b2_size = 6;

    let rs = RankSupport::new(Bitvector::build(&a));

    let res_level1: Vec<u64> = vec![49,49,98];
    let res_level2: Vec<u64> = vec![
                                    7,14,21,28,35,42,
                                    0,0,0,0,0,0,
                                    7,14,21,28,35,42,
                                    0,0,0,0,0,0,
                                ];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_bv256_mixed() {

    let mut a = [0; 256];
    let b1_size = 64;
    for i in 0..b1_size { a[i] = 1; }
    for i in 2*b1_size..3*b1_size { a[i] = 1; }

    let rs = RankSupport::new(Bitvector::build(&a));

    let res_level1: Vec<u64> = vec![64,64,128,128];
    let res_level2: Vec<u64> = vec![8, 16, 24, 32, 40, 48, 56,
                                    0,0,0,0,0,0,0,
                                    8, 16, 24, 32, 40, 48, 56,
                                    0,0,0,0,0,0,0];

    assert_eq!(*rs.get_block_level1(), res_level1);
    assert_eq!(*rs.get_block_level2(), res_level2);
}

// fn: new
#[test]
fn new_build_all_sizes_from_1_to_100() {
    for i in 1..100 {
        let v = vec![1; i];
        let _rs = RankSupport::new(Bitvector::build_from_vec2(&v));
    }
}

// fn: new
#[test]
fn new_build_random() {
    let mut rng = thread_rng();
    for i in 0..5 {
        let n = rng.gen_range(200..300);
        let v: Vec<u32> = (0..n).map(|_| rng.gen_range(0..=1)).collect();

        let _rs = RankSupport::new(Bitvector::build_from_vec2(&v));
    }
}
