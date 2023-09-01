use std::vec::Vec;

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct IntVector {
    l: usize, // size of max value in bits
    data: Vec<u64>,
    n: usize,
}

impl IntVector {
    /// Create new `IntVector` of size `n`, where each value is at most `2^l-1`
    ///
    /// ```
    /// use create::int_vector::IntVector
    ///
    /// // [0,0,0,0]
    /// // data -> [0000000000000000000000000000000000000000000000000000|000|000|000|000]
    /// let iv = IntVector::new(4, 3);
    /// assert_eq!(iv.data.len(), 1);
    /// ```
    pub fn new(n: usize, l: usize) -> IntVector {
        if l > 64 || l == 0 {
            panic!("[IntVector], Error creating new IntVector. Length of values in bits is invalid. Given length:{} ,\
                   length should be in range [1,64]", l);
        }
        let data: Vec<u64> = vec![0; ((n*l)+64-1)/64];

        IntVector {
            l,
            data,
            n,
        }
    }
}
