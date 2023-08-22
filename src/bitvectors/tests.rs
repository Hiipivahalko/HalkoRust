use std::panic;
use std::vec::Vec;
use rand::{Rng, thread_rng};

use crate::bitvectors::{Bitvector, Bit};

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

// fn: build
#[test]
fn build_bitvector_from_array1() {
    let a: [u32; 7] = [0,1,0,0,1,1,0];
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

// fn: build
#[test]
fn build_bitvector_from_array2() {
    let a: [u32; 5] = [1,1,1,1,1];
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

// fn: build
#[test]
fn build_bitvector_from_array_random() {
    let mut rng = thread_rng();

    const N: usize = 300;
    let mut a: [u32; N] = [0; N];
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

// fn: set, get
#[test]
fn set_get_fn_small_bitvector_mutate() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    let arr = [1, 5, 7];
    for i in 0..arr.len() {
        bv.set(arr[i], Bit::ONE);
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
        bv.set(i, Bit::ONE);
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
fn all_ones() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    for i in 0..n {
        bv.set(i, Bit::ONE);
    }

    for i in 0..n {
        assert_eq!(bv.get(i), 1);
    }
}

// fn: set, get
//
// first change all 0s -> 1
// then change all 1s -> 0
#[test]
fn all_ones_first_then_back_zeros() {
    let n: usize = 10;
    let mut bv = Bitvector::build_empty(n);

    for i in 0..n {
        bv.set(i, Bit::ONE);
    }
    for i in 0..n {
        bv.set(i, Bit::ZERO);
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

// fn: len
#[test]
fn bitvector_lenght() {
    let n: usize = 10;
    let bv = Bitvector::build_empty(n);

    assert_eq!(bv.len(), n);
}

// fn: rank1
#[test]
fn rank1_simple() {
    let a: [u32; 7] = [0,1,0,0,1,1,0];
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
        bv.set(i, Bit::ONE);
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

// fn: rank0
#[test]
fn rank0_simple() {
    let a: [u32; 7] = [0,1,0,0,1,1,0];
    let bv = Bitvector::build(&a);

    let result: [u64; 7] = [1,1,2,3,3,3,4];

    for i in 0..7 {
        assert_eq!(
            bv.rank0(i),
            result[i],
            "\n>> Error at index: {},\nrank0(i,bv): {},\ncorrect: {}\n",
            i,
            bv.rank0(i),
            result[i]
        );
    }
}

// fn: rank0
#[test]
fn rank0_zeros() {
    let bv = Bitvector::build_empty(300);

    for i in 0..bv.len() {
        let j = i as u64;
        assert_eq!(
            bv.rank0(i),
            j+1,
            "\n>> Error at index: {},\nrank0(i,bv): {},\ncorrect: {}\n",
            i,
            bv.rank0(i),
            j+1
        );
    }
}

// fn: rank0
#[test]
fn rank0_ones() {
    let mut bv = Bitvector::build_empty(300);
    for i in 0..bv.len() {
        bv.set(i, Bit::ONE);
    }

    for i in 0..bv.len() {
        let rank0 = bv.rank0(i);
        assert_eq!(
            rank0,
            0,
            "\n>> Error at index: {},\nrank0(i,bv): {},\ncorrect: {}\n",
            i,
            rank0,
            0
        );
    }
}

// fn: rank0
#[test]
fn rank0_random1() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(10..=20);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut result: Vec<u64> = vec![0; n*64];
    let mut zeros = 0;
    assert_eq!(bv.len(), result.len());
    for k in 0..bv.len() {
        if bv.get(k) == 0 {
            zeros += 1;
        }
        result[k] = zeros;
    }

    for k in 0..bv.len() {
        assert_eq!(
            bv.rank0(k),
            result[k],
            "\n>> Error at index: {},\nrank0(i,bv): {},\ncorrect: {}\n",
            k,
            bv.rank0(k),
            result[k]
        );
    }
}

// fn: rank0
#[test]
fn rank0_random2() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(50..=60);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut result: Vec<u64> = vec![0; n*64];
    let mut zeros = 0;
    assert_eq!(bv.len(), result.len());
    for k in 0..bv.len() {
        if bv.get(k) == 0 {
            zeros += 1;
        }
        result[k] = zeros;
    }

    for k in 0..bv.len() {
        assert_eq!(
            bv.rank0(k),
            result[k],
            "\n>> Error at index: {},\nrank0(i,bv): {},\ncorrect: {}\n",
            k,
            bv.rank0(k),
            result[k]
        );
    }
}

// fn: select1
#[test]
fn select1_simple() {
    let a: [u32; 7] = [0,1,0,0,1,1,0];
    let bv = Bitvector::build(&a);

    let result: [usize; 4] = [usize::MAX, 1, 4, 5];

    for i in 1..result.len() {
        let select1 = bv.select1(i);
        assert_eq!(
            select1,
            result[i],
            "\n>> Error at {}-th 1bit,\nselect1({},bv): {},\ncorrect: {}\n",
            i,i,
            select1,
            result[i]
        );
    }
}

// fn: select1
#[test]
#[should_panic]
fn select1_zeros() {
    let a: [u32; 7] = [0,0,0,0,0,0,0];
    let bv = Bitvector::build(&a);

    let _result = bv.select1(2);
}

// fn: select1
#[test]
#[should_panic]
fn select1_zeroth_1bit() {
    let a: [u32; 7] = [0,0,0,0,0,0,0];
    let bv = Bitvector::build(&a);

    let _result = bv.select1(0);
}

// fn: select1
#[test]
#[should_panic]
fn select1_over_nth_1bit() {
    let a: [u32; 7] = [0,0,0,0,0,0,0];
    let bv = Bitvector::build(&a);

    let _result = bv.select1(8);
}


// fn: select1
#[test]
fn select1_bitvector_all_ones() {
    let v = vec![u64::MAX, u64::MAX, u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 1..=bv.len() {
        let select1 = bv.select1(i);
        assert_eq!(
            select1,
            i-1,
            "\n>> Error at {}-th 1bit,\nselect1({},bv): {},\ncorrect: {}\n",
            i,i,
            select1,
            i-1
        );
    }
}

// fn: select1
#[test]
fn select1_small() {
    let v = vec![0, u64::MAX,0, 2, 0];
    let bv = Bitvector::build_from_vec(&v);

    for i in 1..=64 {
        let select1 = bv.select1(i);
        let result = i-1+64;
        assert_eq!(
            select1,
            result,
            "\n>> Error at {}-th 1bit,\nselect1({},bv): {},\ncorrect: {}\n",
            i,i,
            select1,
            result
        );
    }

    // value 2 set this bit on in the input vector
    assert_eq!(bv.select1(65), (3*64)-1+2);

    // checking there are no more 1s
    let no_more_ones = panic::catch_unwind(|| {
        bv.select1(66)
    });
    assert!(no_more_ones.is_err());
}

// fn: select1
#[test]
fn select1_random1() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(30..=40);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..=u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut results: Vec<usize> = vec![0];

    // finding solutions
    for i in 0..bv.len() {
        if bv.get(i) == 1 {
            results.push(i);
        }
    }


    for i in 1..results.len() {
        assert_eq!(bv.select1(i), results[i]);
    }
}

// fn: select1
#[test]
fn select1_random2() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(50..=60);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..=u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut results: Vec<usize> = vec![0];

    // finding solutions
    for i in 0..bv.len() {
        if bv.get(i) == 1 {
            results.push(i);
        }
    }


    for i in 1..results.len() {
        assert_eq!(bv.select1(i), results[i]);
    }
}

// fn: select0
#[test]
fn select0_simple() {
    let a: [u32; 7] = [0,1,0,0,1,1,0];
    let bv = Bitvector::build(&a);

    let result: [usize; 5] = [usize::MAX, 0, 2, 3, 6];

    for i in 1..result.len() {
        let select1 = bv.select0(i);
        assert_eq!(
            select1,
            result[i],
            "\n>> Error at {}-th 0bit,\nselect0({},bv): {},\ncorrect: {}\n",
            i,i,
            select1,
            result[i]
        );
    }
}

// fn: select0
#[test]
#[should_panic]
fn select0_ones() {
    let a: [u32; 7] = [1,1,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let _result = bv.select0(2);
}

// fn: select0
#[test]
#[should_panic]
fn select0_zeroth_0bit() {
    let a: [u32; 7] = [1,1,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let _result = bv.select0(0);
}

// fn: select0
#[test]
#[should_panic]
fn select0_over_nth_1bit() {
    let a: [u32; 7] = [1,1,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let _result = bv.select0(8);
}


// fn: select0
#[test]
fn select0_bitvector_all_zeros() {
    let v = vec![0, 0, 0];
    let bv = Bitvector::build_from_vec(&v);

    for i in 1..=bv.len() {
        let select1 = bv.select0(i);
        assert_eq!(
            select1,
            i-1,
            "\n>> Error at {}-th 0bit,\nselect0({},bv): {},\ncorrect: {}\n",
            i,i,
            select1,
            i-1
        );
    }
}

// fn: select0
#[test]
fn select0_small() {
    let v = vec![u64::MAX, 0, u64::MAX, 0xFFFFFFFFFFFFFFFD, u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 1..=64 {
        let select0 = bv.select0(i);
        let result = i-1+64;
        assert_eq!(
            select0,
            result,
            "\n>> Error at {}-th 0bit,\nselect0({},bv): {},\ncorrect: {}\n",
            i,i,
            select0,
            result
        );
    }

    // value 0xFFFFFFFFFFFFFFFD set this bit on in the input vector
    assert_eq!(bv.select0(65), (3*64)-1+2);

    // checking there are no more 1s
    let no_more_ones = panic::catch_unwind(|| {
        bv.select0(66)
    });
    assert!(no_more_ones.is_err());
}

// fn: select0
#[test]
fn select0_random1() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(30..=40);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..=u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut results: Vec<usize> = vec![0];

    // finding solutions
    for i in 0..bv.len() {
        if bv.get(i) == 0 {
            results.push(i);
        }
    }


    for i in 1..results.len() {
        assert_eq!(bv.select0(i), results[i]);
    }
}

// fn: select0
#[test]
fn select0_random2() {
    let mut rng = thread_rng();
    let n: usize = rng.gen_range(50..=60);
    let v: Vec<u64> = (0..n).map(|_| rng.gen_range(0..=u64::MAX)).collect();
    let bv = Bitvector::build_from_vec(&v);

    let mut results: Vec<usize> = vec![0];

    // finding solutions
    for i in 0..bv.len() {
        if bv.get(i) == 0 {
            results.push(i);
        }
    }


    for i in 1..results.len() {
        assert_eq!(bv.select0(i), results[i]);
    }
}

// fn: scan_bits
#[test]
fn scan_bits_zeros() {
    let v: Vec<u64> = vec![0,0];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_bits(0, i, Bit::ZERO, u64::MAX),
            (i as u64 + 1, i),
            "Error scanning zeros, range[0,{}]", i
        );
        assert_eq!(
            bv.scan_bits(0, i, Bit::ONE, u64::MAX),
            (0, i),
            "Error scanning ones"
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_ones() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(bv.scan_bits(0, i, Bit::ZERO, u64::MAX), (0, i));
        assert_eq!(bv.scan_bits(0, i, Bit::ONE, u64::MAX), (i as u64 + 1, i));
    }
}

// fn: scan_bits
#[test]
fn scan_bits_range_one() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(bv.scan_bits(i, i, Bit::ZERO, u64::MAX), (0, i));
        assert_eq!(bv.scan_bits(i, i, Bit::ONE, u64::MAX), (1, i));
    }
}

// fn: scan_bits
#[test]
fn scan_bits_small1() {
    let a: [u32; 10] = [0,0,0,1,0,1,1,0,1,1];
    let bv = Bitvector::build(&a);

    let res1 = vec![0,0,0,1,1,2,3,3,4,5];
    let res0 = vec![1,2,3,3,4,4,4,5,5,5];

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_bits(0, i, Bit::ZERO, u64::MAX),
            (res0[i], i),
            "Error scanning zeros, range[0,{}].",
            i
        );
        assert_eq!(
            bv.scan_bits(0, i, Bit::ONE, u64::MAX),
            (res1[i], i),
            "Error scanning ones, range[0,{}].",
            i
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_small2() {
    let a: [u32; 10] = [0,0,0,0,0,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let res0 = vec![1,2,3,4,5,5,5,5,5,5];
    let res1 = vec![0,0,0,0,0,1,2,3,4,5];

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_bits(0, i, Bit::ZERO, u64::MAX),
            (res0[i], i),
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_bits(0, i, Bit::ONE, u64::MAX),
            (res1[i], i),
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_with_limit() {
    let a: [u32; 10] = [0,0,0,0,0,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let limit = 3;
    let res0 = vec![(1,0),(2,1),(3,2),(3,2),(3,2),(3,2),(3,2),(3,2),(3,2),(3,2)];
    let res1 = vec![(0,0),(0,1),(0,2),(0,3),(0,4),(1,5),(2,6),(3,7),(3,7),(3,7)];

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_bits(0, i, Bit::ZERO, limit),
            res0[i],
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_bits(0, i, Bit::ONE, limit),
            res1[i],
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_small_range_with_limit() {
    let a: [u32; 10] = [1,1,0,1,0,1,1,1,0,1];
    let bv = Bitvector::build(&a);

    // [11|01011101|]
    assert_eq!(bv.scan_bits(2, bv.len()-1, Bit::ONE, 2), (2,5));
    // [110|10111|01]
    assert_eq!(bv.scan_bits(3, 7, Bit::ZERO, 1), (1,4));
    // [1|1010111|01]
    assert_eq!(bv.scan_bits(1, 7, Bit::ONE, 3), (3,5));
}

// fn: scan_bits
#[test]
fn scan_bits_random_without_limit() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N+1] = [0; N+1];
    let mut res0: [(u64, usize); N+1] = [(0,0); N+1];
    let mut res1: [(u64, usize); N+1] = [(0,0); N+1];

    for i in 1..a.len() {
        a[i] = rng.gen_range(0..=1);
        res0[i] = (res0[i-1].0 + if a[i] == 0 {1} else {0}, i-1);
        res1[i] = (res1[i-1].0 + if a[i] == 0 {0} else {1}, i-1);
    }
    let bv = Bitvector::build(&a[1..]);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_bits(0, i, Bit::ZERO, u64::MAX),
            res0[i+1],
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_bits(0, i, Bit::ONE, u64::MAX),
            res1[i+1],
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_random_without_limit_start_random() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N] = [0; N];
    let mut res0: [(u64, usize); N] = [(0,0); N];
    let mut res1: [(u64, usize); N] = [(0,0); N];

    for i in 0..a.len() {
        a[i] = rng.gen_range(0..=1);
    }
    let bv = Bitvector::build(&a);

    let start = rng.gen_range(50..=N/2)-1;
    if a[start] == 1 {
        res0[start] = (0,start);
        res1[start] = (1,start);
    } else {
        res0[start] = (1,start);
        res1[start] = (0,start);
    }
    for i in start+1..bv.len() {
        res0[i] = (res0[i-1].0 + if a[i] == 0 {1} else {0}, i);
        res1[i] = (res1[i-1].0 + if a[i] == 0 {0} else {1}, i);
    }
    for i in start..bv.len() {
        assert_eq!(
            bv.scan_bits(start, i, Bit::ZERO, u64::MAX),
            res0[i],
            "Error scanning zeros, start:{}, range[{},{}].",
            start, start, i
        );
        assert_eq!(
            bv.scan_bits(start, i, Bit::ONE, u64::MAX),
            res1[i],
            "Error scanning ones, start:{}, range[{},{}].",
            start, start, i
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_random_with_limit() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N] = [0; N];
    let mut res0: [(u64, usize); N] = [(0,0); N];
    let mut res1: [(u64, usize); N] = [(0,0); N];

    let limit = rng.gen_range(20..=40);

    let mut limit0 = false;
    let mut limit0_val = (0,0);
    let mut limit1 = false;
    let mut limit1_val = (0,0);

    a[0] = rng.gen_range(0..=1);
    if a[0] == 1 {
        res0[0] = (0,0);
        res1[0] = (1,0);
    } else {
        res0[0] = (1,0);
        res1[0] = (0,0);
    }

    for i in 1..a.len() {
        a[i] = rng.gen_range(0..=1);
        res0[i] = if !limit0 {
                (res0[i-1].0 + if a[i] == 0 {1} else {0}, i)
            } else {
                limit0_val
            };
        res1[i] = if !limit1 {
                (res1[i-1].0 + if a[i] == 0 {0} else {1}, i)
            } else {
                limit1_val
            };

        if !limit0 && res0[i].0 == limit {
            limit0 = true;
            limit0_val = res0[i];
        }
        if !limit1 && res1[i].0 == limit {
            limit1 = true;
            limit1_val = res1[i];
        }

    }
    let bv = Bitvector::build(&a);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_bits(0, i, Bit::ZERO, limit),
            res0[i],
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_bits(0, i, Bit::ONE, limit),
            res1[i],
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_bits
#[test]
fn scan_bits_random_with_limit_start_random() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N] = [0; N];
    let mut res0: [(u64, usize); N] = [(0,0); N];
    let mut res1: [(u64, usize); N] = [(0,0); N];

    for i in 0..a.len() {
        a[i] = rng.gen_range(0..=1);
    }
    let bv = Bitvector::build(&a);

    let limit = rng.gen_range(20..=40);
    let mut limit0 = false;
    let mut limit0_val = (0,0);
    let mut limit1 = false;
    let mut limit1_val = (0,0);

    let start = rng.gen_range(50..=N/2)-1;

    if a[start] == 1 {
        res0[start] = (0,start);
        res1[start] = (1,start);
    } else {
        res0[start] = (1,start);
        res1[start] = (0,start);
    }

    for i in start+1..a.len() {
        res0[i] = if !limit0 {
                (res0[i-1].0 + if a[i] == 0 {1} else {0}, i)
            } else {
                limit0_val
            };
        res1[i] = if !limit1 {
                (res1[i-1].0 + if a[i] == 0 {0} else {1}, i)
            } else {
                limit1_val
            };

        if !limit0 && res0[i].0 == limit {
            limit0 = true;
            limit0_val = res0[i];
        }
        if !limit1 && res1[i].0 == limit {
            limit1 = true;
            limit1_val = res1[i];
        }
    }
    for i in start..bv.len() {
        assert_eq!(
            bv.scan_bits(start, i, Bit::ZERO, limit),
            res0[i],
            "Error scanning zeros, i:{}, start:{}",
            i, start
        );
        assert_eq!(
            bv.scan_bits(start, i, Bit::ONE, limit),
            res1[i],
            "Error scanning ones, i:{}, start:{}",
            i, start
        );
    }
}


// fn: scan_blocks
#[test]
fn scan_blocks_zeros() {
    let v: Vec<u64> = vec![0,0];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, u64::MAX),
            (i as u64+1, i),
            "Error scanning zeros,\nrange: [0,{}].\n",
            i,
       );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, u64::MAX),
            (0, i),
            "Error scanning ones, range: [0,{}]",
            i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_ones() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, u64::MAX),
            (0, i),
            "Error scanning zeros,\nrange: [0,{}].\n",
            i,
       );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, u64::MAX),
            (i as u64+1, i),
            "Error scanning ones, range: [0,{}]",
            i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_range_one() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX];
    let bv = Bitvector::build_from_vec(&v);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(i, i, Bit::ZERO, u64::MAX),
            (0, i),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i,i
        );
        assert_eq!(
            bv.scan_blocks(i, i, Bit::ONE, u64::MAX),
            (1, i),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_range_7() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX,u64::MAX,u64::MAX];
    let v2: Vec<u64> = vec![0,0,0,0];
    let bv_ones = Bitvector::build_from_vec(&v);
    let bv_zeros = Bitvector::build_from_vec(&v2);

    let range = 7-1;
    for i in 0..bv_ones.len()-1-range {
        assert_eq!(
            bv_ones.scan_blocks(i, i+range, Bit::ZERO, u64::MAX),
            (0, i+range),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i, i+range
        );
        assert_eq!(
            bv_ones.scan_blocks(i, i+range, Bit::ONE, u64::MAX),
            (1 + range as u64, i+range),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i+range
        );
    }

    for i in 0..bv_zeros.len()-1-range {
        assert_eq!(
            bv_zeros.scan_blocks(i, i+range, Bit::ZERO, u64::MAX),
            (1 + range as u64, i+range),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i, i+range
        );
        assert_eq!(
            bv_zeros.scan_blocks(i, i+range, Bit::ONE, u64::MAX),
            (0, i+range),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i+range
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_range_32() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX,u64::MAX,u64::MAX];
    let v2: Vec<u64> = vec![0,0,0,0];
    let bv_ones = Bitvector::build_from_vec(&v);
    let bv_zeros = Bitvector::build_from_vec(&v2);

    let range = 32-1;
    for i in 0..bv_ones.len()-1-range {
        assert_eq!(
            bv_ones.scan_blocks(i, i+range, Bit::ZERO, u64::MAX),
            (0, i+range),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i, i+range
        );
        assert_eq!(
            bv_ones.scan_blocks(i, i+range, Bit::ONE, u64::MAX),
            (1 + range as u64, i+range),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i+range
        );
    }

    for i in 0..bv_zeros.len()-1-range {
        assert_eq!(
            bv_zeros.scan_blocks(i, i+range, Bit::ZERO, u64::MAX),
            (1 + range as u64, i+range),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i, i+range
        );
        assert_eq!(
            bv_zeros.scan_blocks(i, i+range, Bit::ONE, u64::MAX),
            (0, i+range),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i+range
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_range_64() {
    let v: Vec<u64> = vec![u64::MAX,u64::MAX,u64::MAX,u64::MAX];
    let v2: Vec<u64> = vec![0,0,0,0];
    let bv_ones = Bitvector::build_from_vec(&v);
    let bv_zeros = Bitvector::build_from_vec(&v2);

    let range = 64-1;
    for i in 0..bv_ones.len()-1-range {
        assert_eq!(
            bv_ones.scan_blocks(i, i+range, Bit::ZERO, u64::MAX),
            (0, i+range),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i, i+range
        );
        assert_eq!(
            bv_ones.scan_blocks(i, i+range, Bit::ONE, u64::MAX),
            (1 + range as u64, i+range),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i+range
        );
    }

    for i in 0..bv_zeros.len()-1-range {
        assert_eq!(
            bv_zeros.scan_blocks(i, i+range, Bit::ZERO, u64::MAX),
            (1 + range as u64, i+range),
            "Error scanning zeros,\nrange: [{},{}].\n",
            i, i+range
        );
        assert_eq!(
            bv_zeros.scan_blocks(i, i+range, Bit::ONE, u64::MAX),
            (0, i+range),
            "Error scanning ones,\nrange: [{},{}].\n",
            i,i+range
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_small1() {
    let a: [u32; 10] = [0,0,0,1,0,1,1,0,1,1];
    let bv = Bitvector::build(&a);

    let res1 = vec![0,0,0,1,1,2,3,3,4,5];
    let res0 = vec![1,2,3,3,4,4,4,5,5,5];

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, u64::MAX),
            (res0[i], i),
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, u64::MAX),
            (res1[i], i),
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_small2() {
    let a: [u32; 10] = [0,0,0,0,0,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let res0 = vec![1,2,3,4,5,5,5,5,5,5];
    let res1 = vec![0,0,0,0,0,1,2,3,4,5];

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, u64::MAX),
            (res0[i], i),
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, u64::MAX),
            (res1[i], i),
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_with_limit_small() {
    let a: [u32; 10] = [0,0,0,0,0,1,1,1,1,1];
    let bv = Bitvector::build(&a);

    let limit = 3;
    let res0 = vec![(1,0),(2,1),(3,2),(3,2),(3,2),(3,2),(3,2),(3,2),(3,2),(3,2)];
    let res1 = vec![(0,0),(0,1),(0,2),(0,3),(0,4),(1,5),(2,6),(3,7),(3,7),(3,7)];

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, limit),
            res0[i],
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, limit),
            res1[i],
            "Error scanning ones, i:{}",
            i
        );
    }
}
//
// fn: scan_blocks
#[test]
fn scan_blocks_with_limit_small2() {
    let v = vec![0, 81, 0];
    let bv = Bitvector::build_from_vec(&v);

    println!("{}",bv[1]);
    let limit = 3;

    for i in 64+7..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, limit),
            (3, 2),
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, limit),
            (3, 64+7-1),
            "Error scanning ones, range[0,{}].",
            i
        );
    }
}


// fn: scan_blocks
#[test]
fn scan_blocks_small_range_with_limit() {
    let mut rng = thread_rng();
    let v = vec![7, u64::MAX, 0];
    let bv = Bitvector::build_from_vec(&v);

    for i in 3..64 {
        let start = rng.gen_range(5..60);
        assert_eq!(bv.scan_blocks(start, i+64, Bit::ONE, 3),
            (3, 64+3-1)
        )
    }
    for i in 0..64 {
        assert_eq!(bv.scan_blocks(0, i+64*2, Bit::ZERO, 3),
            (3, 5)
        )
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_random_without_limit() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N+1] = [0; N+1];
    let mut res0: [(u64, usize); N+1] = [(0,0); N+1];
    let mut res1: [(u64, usize); N+1] = [(0,0); N+1];

    for i in 1..a.len() {
        a[i] = rng.gen_range(0..=1);
        res0[i] = (res0[i-1].0 + if a[i] == 0 {1} else {0}, i-1);
        res1[i] = (res1[i-1].0 + if a[i] == 0 {0} else {1}, i-1);
    }
    let bv = Bitvector::build(&a[1..]);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, u64::MAX),
            res0[i+1],
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, u64::MAX),
            res1[i+1],
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_random_without_limit_start_random() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N] = [0; N];
    let mut res0: [(u64, usize); N] = [(0,0); N];
    let mut res1: [(u64, usize); N] = [(0,0); N];

    for i in 0..a.len() {
        a[i] = rng.gen_range(0..=1);
    }
    let bv = Bitvector::build(&a);

    let start = rng.gen_range(50..=N/2)-1;
    if a[start] == 1 {
        res0[start] = (0,start);
        res1[start] = (1,start);
    } else {
        res0[start] = (1,start);
        res1[start] = (0,start);
    }
    for i in start+1..bv.len() {
        res0[i] = (res0[i-1].0 + if a[i] == 0 {1} else {0}, i);
        res1[i] = (res1[i-1].0 + if a[i] == 0 {0} else {1}, i);
    }
    for i in start..bv.len() {
        assert_eq!(
            bv.scan_blocks(start, i, Bit::ZERO, u64::MAX),
            res0[i],
            "Error scanning zeros, start:{}, range[{},{}].",
            start, start, i
        );
        assert_eq!(
            bv.scan_blocks(start, i, Bit::ONE, u64::MAX),
            res1[i],
            "Error scanning ones, start:{}, range[{},{}].",
            start, start, i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_random_with_limit() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N] = [0; N];
    let mut res0: [(u64, usize); N] = [(0,0); N];
    let mut res1: [(u64, usize); N] = [(0,0); N];

    let limit = rng.gen_range(20..=40);

    let mut limit0 = false;
    let mut limit0_val = (0,0);
    let mut limit1 = false;
    let mut limit1_val = (0,0);

    a[0] = rng.gen_range(0..=1);
    if a[0] == 1 {
        res0[0] = (0,0);
        res1[0] = (1,0);
    } else {
        res0[0] = (1,0);
        res1[0] = (0,0);
    }

    for i in 1..a.len() {
        a[i] = rng.gen_range(0..=1);
        res0[i] = if !limit0 {
                (res0[i-1].0 + if a[i] == 0 {1} else {0}, i)
            } else {
                limit0_val
            };
        res1[i] = if !limit1 {
                (res1[i-1].0 + if a[i] == 0 {0} else {1}, i)
            } else {
                limit1_val
            };

        if !limit0 && res0[i].0 == limit {
            limit0 = true;
            limit0_val = res0[i];
        }
        if !limit1 && res1[i].0 == limit {
            limit1 = true;
            limit1_val = res1[i];
        }

    }
    let bv = Bitvector::build(&a);

    for i in 0..bv.len() {
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ZERO, limit),
            res0[i],
            "Error scanning zeros, i:{}",
            i
        );
        assert_eq!(
            bv.scan_blocks(0, i, Bit::ONE, limit),
            res1[i],
            "Error scanning ones, i:{}",
            i
        );
    }
}

// fn: scan_blocks
#[test]
fn scan_blocks_random_with_limit_start_random() {
    let mut rng = thread_rng();
    const N: usize = 321;
    let mut a: [u32; N] = [0; N];
    let mut res0: [(u64, usize); N] = [(0,0); N];
    let mut res1: [(u64, usize); N] = [(0,0); N];

    for i in 0..a.len() {
        a[i] = rng.gen_range(0..=1);
    }
    let bv = Bitvector::build(&a);

    let limit = rng.gen_range(20..=40);
    let mut limit0 = false;
    let mut limit0_val = (0,0);
    let mut limit1 = false;
    let mut limit1_val = (0,0);

    let start = rng.gen_range(50..=N/2)-1;

    if a[start] == 1 {
        res0[start] = (0,start);
        res1[start] = (1,start);
    } else {
        res0[start] = (1,start);
        res1[start] = (0,start);
    }

    for i in start+1..a.len() {
        res0[i] = if !limit0 {
                (res0[i-1].0 + if a[i] == 0 {1} else {0}, i)
            } else {
                limit0_val
            };
        res1[i] = if !limit1 {
                (res1[i-1].0 + if a[i] == 0 {0} else {1}, i)
            } else {
                limit1_val
            };

        if !limit0 && res0[i].0 == limit {
            limit0 = true;
            limit0_val = res0[i];
        }
        if !limit1 && res1[i].0 == limit {
            limit1 = true;
            limit1_val = res1[i];
        }
    }
    for i in start..bv.len() {
        assert_eq!(
            bv.scan_blocks(start, i, Bit::ZERO, limit),
            res0[i],
            "Error scanning zeros, i:{}, start:{}",
            i, start
        );
        assert_eq!(
            bv.scan_blocks(start, i, Bit::ONE, limit),
            res1[i],
            "Error scanning ones, i:{}, start:{}",
            i, start
        );
    }
}
