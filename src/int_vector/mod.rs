use std::fmt;
use std::vec::Vec;

#[cfg(test)]
mod tests;

/// Compact integer vector structure. Stores each value in the vector using `l` bits.
///
/// ```
/// use halko_rust::int_vector::IntVector;
///
/// let mut iv = IntVector::new(4, 8);
///
/// iv.set(1,1);
/// iv.set(2,2);
/// iv.set(3,3);
///
/// assert_eq!(iv.len(), 4);
/// assert_eq!(iv.get(0), 0);
/// assert_eq!(iv.get(1), 1);
/// assert_eq!(iv.get(2), 2);
/// assert_eq!(iv.get(3), 3);
/// ```
///
#[derive(Debug, Clone, PartialEq)]
pub struct IntVector {
    l: usize, // size of max value in bits
    data: Vec<u64>,
    n: usize,
}

impl IntVector {
    /// Create new `IntVector` of size `n`, where each value is at most `2^l-1`
    ///
    /// ```
    /// use halko_rust::int_vector::IntVector;
    ///
    /// // [0,0,0,0]
    /// // data -> [0000000000000000000000000000000000000000000000000000|000|000|000|000]
    /// let iv = IntVector::new(4, 3);
    /// assert_eq!(iv.get_data().len(), 1);
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

    /// Set new value `new_val` into the `i`-th value in the `IntVector`
    ///
    /// ```
    /// use halko_rust::int_vector::IntVector;
    ///
    /// let mut iv = IntVector::new(5, 8);
    ///
    /// assert_eq!(iv.get(0), 0);
    /// iv.set(0, 12);
    /// assert_eq!(iv.get(0), 12);
    ///
    /// iv.set(2, 1);
    /// iv.set(3, 0);
    /// iv.set(4, 2);
    ///
    /// assert_eq!(iv.get(2), 1);
    /// assert_eq!(iv.get(3), 0);
    /// assert_eq!(iv.get(4), 2);
    /// ```
    pub fn set(&mut self, i: usize, new_val: u64) {
        if new_val > 0 && new_val.ilog2() as usize > self.l {
            panic!("[IntVector::set], Error setting new value into IntVector, new value is too large. \
                   new value: {}, largest valid value: {}.",
                   new_val, self.l.pow(2)-1);
        }

        if i >= self.n {
            panic!("[IntVector::set], Index out of bounds, i:{}, IntVector length:{}",
                   i, self.n);
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

    /// Returns the `i`-th value in the `IntVector`
    ///
    /// ```
    /// use halko_rust::int_vector::IntVector;
    ///
    /// let mut iv = IntVector::new(5, 8);
    ///
    /// assert_eq!(iv.get(0), 0);
    /// iv.set(0, 12);
    /// assert_eq!(iv.get(0), 12);
    ///
    /// iv.set(2, 1);
    /// iv.set(3, 0);
    /// iv.set(4, 2);
    ///
    /// assert_eq!(iv.get(2), 1);
    /// assert_eq!(iv.get(3), 0);
    /// assert_eq!(iv.get(4), 2);
    /// ```
    pub fn get(&self, i: usize) -> u64 {

        if i >= self.n {
            panic!("[IntVector::set], Index out of bounds, i:{}, IntVector length:{}",
                   i, self.n);
        }

        let k = (i*self.l)/64;

        if self.l == 64 {
            return self.data[k];
        }

        let loc_i = (i*self.l)%64;

        // value completely inside the block
        if loc_i + self.l <= 64 {
            return (self.data[k] >> loc_i) & !(u64::MAX << self.l);
        }

        let right_part = self.data[k] >> loc_i;
        let left_part = !(u64::MAX << ((loc_i+self.l)%64)) & self.data[k+1];
        (left_part << (64-loc_i)) | right_part
    }

    /// Returns the length of the IntVector.
    ///
    /// ```
    /// use halko_rust::int_vector::IntVector;
    ///
    /// let iv = IntVector::new(5, 8);
    ///
    /// assert_eq!(iv.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns reference to the raw data of IntVector.
    ///
    /// ```
    /// use halko_rust::int_vector::IntVector;
    ///
    /// let iv = IntVector::new(4, 32);
    ///
    /// let data = iv.get_data();
    /// assert_eq!(data[0], 0);
    /// assert_eq!(data[1], 0);
    /// assert_eq!(data.len(), 2);
    ///
    /// let mut iv2 = IntVector::new(4, 32);
    /// iv2.set(0, 2_u64.pow(32)-1);
    /// iv2.set(1, 2_u64.pow(32)-1);
    /// iv2.set(2, 2_u64.pow(32)-1);
    /// iv2.set(3, 2_u64.pow(32)-1);
    ///
    /// let data2 = iv2.get_data();
    ///
    /// assert_eq!(data2[0], u64::MAX);
    /// assert_eq!(data2[1], u64::MAX);
    ///
    /// ```
    pub fn get_data(&self) -> &Vec<u64> {
        &self.data
    }
}

impl fmt::Display for IntVector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut res = "[".to_string();
        for i in 0..self.n {
            res.push_str(self.get(i).to_string().as_str());
            res.push(',');
        }
        res.push(']');
        write!(f, "{}", res)
    }
}
