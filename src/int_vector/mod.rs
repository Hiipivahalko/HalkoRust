use std::vec::Vec;

#[cfg(test)]
mod tests;

/// Compact integer vector structure. Stores each value in the vector using `l` bits.
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

    pub fn set(&mut self, i: usize, new_val: u64) {
        if new_val > 0 && new_val.ilog2() as usize > self.l {
            panic!("[IntVector], Error setting new value into IntVector, new value is too large. \
                   new value: {}, largest valid value: {}.",
                   new_val, self.l.pow(2)-1);
        }

        let k = (i*self.l)/64;

        if self.l == 64 {
            self.data[k] = new_val;
            return;
        }

        let loc_i = (i*self.l)%64;

        // value completely inside the block
        if loc_i + self.l <= 64 {
            let left_neg = if self.l + loc_i < 64 {u64::MAX << (self.l + loc_i)} else {0};
            let right_neg = !(u64::MAX << loc_i);
            let neg = left_neg | right_neg;
            // clean prev bits
            self.data[k] = self.data[k] & neg;

            // set new value
            self.data[k] = self.data[k] | (new_val << loc_i);
        } else {
            // clean data in firts block
            self.data[k] = self.data[k] & (u64::MAX >> (64-loc_i));
            // set data in first block
            self.data[k] = self.data[k] | (new_val << loc_i);

            // clean data in second block
            self.data[k+1] = self.data[k+1] & (u64::MAX << ((loc_i+self.l)%64));
            // set data in second block
            self.data[k+1] = self.data[k+1] | (new_val >> (64-loc_i));
        }

    }
}
