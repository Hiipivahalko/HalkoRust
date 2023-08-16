use std::vec::Vec;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

/// Simple bitvector implementation.
/// Use struct's build-functions to building/initializing Bitvector
///
/// # Example
///
/// ```
/// use halko_rust::bitvectors::Bitvector;
///
/// // [0,0,0,0,0]
/// let mut bv = Bitvector::build_empty(5);
///
/// for i in 0..5 { assert_eq!(0, bv.get(i)); }
///
/// bv.set(0, 1); // bv = [1,0,0,0,0];
/// assert_eq!(bv.get(0), 1);
///
/// bv.set(3, 1); // bv = [1,0,0,1,0];
/// assert_eq!(bv.get(3), 1);
/// ```
///
/// * Bitvector can be builded from the array containing 0s and 1s.
/// ```
/// use halko_rust::bitvectors::Bitvector;
///
/// let a: [u64; 7] = [0,1,0,0,1,1,0];
/// let bv = Bitvector::build(&a);
///
/// for i in 0..a.len() { assert_eq!(a[i], bv.get(i)); }
/// ```
#[derive(Debug)]
pub struct Bitvector {
    data: Vec<u64>,
    n: usize,
}

impl Bitvector {
    /// Builds bitvector of length `n` containing only 0s.
    ///
    /// ```
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// // [0,0,0,0,0]
    /// let mut bv = Bitvector::build_empty(5);
    ///
    /// for i in 0..5 { assert_eq!(0, bv.get(i)); }
    /// ```
    pub fn build_empty(n: usize) -> Bitvector {
        Bitvector {
            data: vec![0; (n/64) + 1],
            n,
        }
    }

    /// Builds bitvector from input array containing only 0s and 1s.
    ///
    /// ```
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let a: [u64; 7] = [0,1,0,0,1,1,0];
    /// let bv = Bitvector::build(&a);
    ///
    /// for i in 0..a.len() { assert_eq!(a[i], bv.get(i)); }
    /// ```
    pub fn build(arr: &[u64]) -> Bitvector {
        let mut bv = Bitvector::build_empty(arr.len());
        for (i, v) in arr.iter().enumerate() {
            bv.set(i, *v);
        }

        bv
    }

    /// Builds bitvector from vector containing positive integers.
    /// Copies only input vector values into data variable and length
    /// of the result bitvector is `v.len()`*64.
    ///
    /// ```rust
    /// use halko_rust::bitvectors::Bitvector;
    /// use std::vec::Vec;
    ///
    /// let v: Vec<u64> = vec![0,2];
    /// let bv = Bitvector::build_from_vec(&v);
    ///
    /// // bv
    /// // 00000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// // 01000000 00000000 00000000 00000000 00000000 00000000 00000000 00000000
    /// ```
    pub fn build_from_vec(v: &Vec<u64>) -> Bitvector {
        Bitvector {
            data: v.to_vec(),
            n: v.len()*64,
        }
    }


    /// Return length of the bitvector (number of bits).
    ///
    /// ```rust
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let bv1 = Bitvector::build_empty(5);
    /// assert_eq!(bv1.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns bit value in the i-th bit.
    pub fn get(&self, i: usize) -> u64 {
        let one: u64 = 1;
        (&self.data[i/64] >> (i%64)) & one
    }

    /// Sets or unsets the i-th bit in the bitvector.
    ///
    /// ```
    /// use halko_rust::bitvectors::Bitvector;
    /// let mut bv = Bitvector::build_empty(5); // [0,0,0,0,0]
    ///
    /// bv.set(1, 1); // [0,1,0,0,0]
    /// bv.set(1, 0); // [0,0,0,0,0]
    /// ```
    pub fn set(&mut self, i: usize, val: u64) {
        let one: u64 = 1;
        if val == 0 {
            self.data[i/64] &= !(one << (i%64));
        } else {
           self.data[i/64] |= one << (i%64);
        }
    }

    /// Returns numbers of 1s in the bitvector in range [0,i].
    ///
    /// ```rust
    /// use std::panic;
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let a: [u64; 7] = [0,1,0,0,1,1,0];
    /// let bv = Bitvector::build(&a);
    ///
    /// assert_eq!(bv.rank1(0), 0);
    /// assert_eq!(bv.rank1(1), 1);
    /// assert_eq!(bv.rank1(2), 1);
    /// assert_eq!(bv.rank1(6), 3);
    ///
    /// let panic_result = panic::catch_unwind(|| {
    ///     bv.rank1(7)
    /// });
    /// assert!(panic_result.is_err());
    /// ```
    pub fn rank1(&self, i: usize) -> u64 {
        if i >= self.n {
            panic!("Rank query out of the bitvector range -> i:{}, length of bitvector:{}", i, self.n);
        }

        let ones_in_block = (self.data[i/64] << (64 - (i%64)-1)).count_ones() as u64;
        // ones in range [0,i-1]
        let m_i = self.data[0..(i/64)].iter().fold(0, |acc, x| (acc + x.count_ones() as u64));

        m_i + ones_in_block
    }
}

impl Index<usize> for Bitvector {
    type Output = u64;

    /// ```rust
    /// use halko_rust::bitvectors::Bitvector;
    /// use std::vec::Vec;
    ///
    /// let v: Vec<u64> = vec![0,2];
    /// let bv = Bitvector::build_from_vec(&v);
    ///
    /// assert_eq!(bv[0], v[0]);
    /// assert_eq!(bv[1], v[1]);
    /// ```
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Bitvector {
    /// ```rust
    /// use halko_rust::bitvectors::Bitvector;
    /// use std::vec::Vec;
    ///
    /// let v: Vec<u64> = vec![0,2];
    /// let mut bv = Bitvector::build_from_vec(&v);
    ///
    /// for i in 0..64 { assert_eq!(bv.get(i), 0) };
    ///
    /// bv[0] = u64::MAX;
    /// for i in 0..64 { assert_eq!(bv.get(i), 1) };
    /// ```
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}
