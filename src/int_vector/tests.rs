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
