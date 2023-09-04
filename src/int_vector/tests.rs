use crate::int_vector::IntVector;

// fn: new
#[test]
fn new_with_one_val_l_size_from_1_to_64() {
    for i in 1..=64 {
        let iv = IntVector::new(1, i);
        assert_eq!(iv.data.len(), 1);
        assert_eq!(iv.l, i);
    }
}

// fn: new
#[test]
fn new_with_two_val_l_size_from_1_to_64() {
    for i in 1..=32 {
        let iv = IntVector::new(2, i);
        assert_eq!(iv.data.len(), 1);
        assert_eq!(iv.l, i);
    }
    for i in 33..=64 {
        let iv = IntVector::new(2, i);
        assert_eq!(iv.data.len(), 2);
        assert_eq!(iv.l, i);
    }
}

// fn: new
#[test]
fn new_with_three_val_l_size_from_1_to_64() {
    for i in 1..=21 {
        let iv = IntVector::new(3, i);
        assert_eq!(iv.data.len(), 1);
        assert_eq!(iv.l, i);
    }
    for i in 22..=42 {
        let iv = IntVector::new(3, i);
        assert_eq!(iv.data.len(), 2);
        assert_eq!(iv.l, i);
    }
    for i in 43..=64 {
        let iv = IntVector::new(3, i);
        assert_eq!(iv.data.len(), 3);
        assert_eq!(iv.l, i);
    }
}

// fn: new
#[test]
#[should_panic]
fn new_with_too_large_l_value() {
    let _iv = IntVector::new(10, 65);
}

// fn: new
#[test]
#[should_panic]
fn new_with_too_small_l_value() {
    let _iv = IntVector::new(10, 0);
}

// fn: set
#[test]
fn set_l_64_to_max_and_then_to_zero() {
    let mut iv = IntVector::new(2,64);

    iv.set(0, u64::MAX);
    assert_eq!(iv.data[0], u64::MAX);
    assert_eq!(iv.data[1], 0);

    iv.set(1, u64::MAX);
    assert_eq!(iv.data[0], u64::MAX);
    assert_eq!(iv.data[1], u64::MAX);

    iv.set(0, 0);
    assert_eq!(iv.data[0], 0);
    assert_eq!(iv.data[1], u64::MAX);

    iv.set(1, 0);
    assert_eq!(iv.data[0], 0);
    assert_eq!(iv.data[0], 0);
}

// fn: set
#[test]
fn set_l_4_to_max_all_values_inside_same_block() {
    let mut iv = IntVector::new(32, 4); // data.len == 2

    for i in 0..16 {
        iv.set(i, 15);
        let res = u64::MAX >> (64-((i+1)*4));

        assert_eq!(iv.data[0], res,
                   "Error when i:{}", i
                   );
    }
    for i in 0..16 {
        iv.set(i+16, 15);
        let res = u64::MAX >> (64-((i+1)*4));

        assert_eq!(iv.data[1], res,
                   "Error when i:{}", i+16
                   );
    }

    for i in 0..16-1 {
        iv.set(i, 0);
        let res = u64::MAX << ((i+1)*4);

        assert_eq!(iv.data[0], res,
                   "Error when i:{}", i
                   );
    }
    iv.set(15, 0);
    assert_eq!(iv.data[0], 0);

    for i in 0..16-1 {
        iv.set(i+16, 0);
        let res = u64::MAX << ((i+1)*4);

        assert_eq!(iv.data[1], res,
                   "Error when i:{}", i+16
                   );
    }
    iv.set(31, 0);
    assert_eq!(iv.data[1], 0);
}

// fn: set
#[test]
fn set_l_5_to_max_all_values_overlap_blocks() {
    let mut iv = IntVector::new(39, 5); // data.len == 3

    // set block values to max -> set bits on (1) in every block.
    for i in 0..12 {
        iv.set(i, 31);
        let res = u64::MAX >> (64-((i+1)*5));

        assert_eq!(iv.data[0], res,
                   "Error when i:{}", i
                   );
    }
    iv.set(12, 31);
    assert_eq!(iv.data[0], u64::MAX);
    assert_eq!(iv.data[1], 1);

    for i in 0..12 {
        iv.set(i+13, 31);
        let res = u64::MAX >> (64-((i+1)*5)-1);

        assert_eq!(iv.data[1], res,
                   "Error when i:{}", i+13
                   );
    }
    iv.set(25, 31);
    assert_eq!(iv.data[1], u64::MAX);
    assert_eq!(iv.data[2], 3);

    for i in 0..12 {
        iv.set(i+26, 31);
        let res = u64::MAX >> (64-((i+1)*5)-2);

        assert_eq!(iv.data[2], res,
                   "Error when i:{}", i+26
                   );
    }
    iv.set(38, 31);
    assert_eq!(iv.data[2], u64::MAX);
    assert_eq!(iv.data[3], 7);

    // set block values to min -> set bits off (0) in every block.
    for i in 0..12 {
        iv.set(i, 0);
        let res = u64::MAX << ((i+1)*5);

        assert_eq!(iv.data[0], res,
                   "Error when i:{}", i
                   );
    }
    iv.set(12, 0);
    assert_eq!(iv.data[0], 0);
    //println!("{}", !1_u64);
    assert_eq!(iv.data[1], !1_u64);

    for i in 0..12 {
        iv.set(i+13, 0);
        let res = u64::MAX << (((i+1)*5)+1);

        assert_eq!(iv.data[1], res,
                   "Error when i:{}", i+13
                   );
    }
    iv.set(25, 0);
    assert_eq!(iv.data[1], 0);
    assert_eq!(iv.data[2], !3_u64);

    for i in 0..12 {
        iv.set(i+26, 0);
        let res = u64::MAX << (((i+1)*5)+2);

        assert_eq!(iv.data[2], res,
                   "Error when i:{}", i+26
                   );
    }
    iv.set(38, 0);
    assert_eq!(iv.data[2], 0);
    assert_eq!(iv.data[3], 0);
}

// fn: set
#[test]
#[should_panic]
fn set_try_to_set_too_large_value() {

    let mut iv = IntVector::new(39, 5); // data.len == 3

    iv.set(0, 64);
}
