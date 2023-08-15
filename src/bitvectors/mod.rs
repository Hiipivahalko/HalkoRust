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
    pub fn build_empty(n: usize) -> Bitvector {
        Bitvector {
            data: vec![0; (n/64) + 1],
            n,
        }

    }

    /// Builds bitvector from input array containing only 0s and 1s.
    pub fn build(arr: &[u64]) -> Bitvector {
        let mut bv = Bitvector::build_empty(arr.len());
        for (i, v) in arr.iter().enumerate() {
            bv.set(i, *v);
            println!("v: {}, bv[{}]: {}", *v, i, bv.get(i));
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
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns bit value in the i-th bit.
    pub fn get(&self, i: usize) -> u64 {
        let one: u64 = 1;
        (&self.data[i/64] >> (i%64)) & one
    }

    /// Sets or unsets the i-th bit in the bitvector.
    pub fn set(&mut self, i: usize, val: u64) {
        let one: u64 = 1;
        if val == 0 {
            self.data[i/64] &= !(one << (i%64));
        } else {
           self.data[i/64] |= one << (i%64);
        }
    }

}

impl Index<usize> for Bitvector {
    type Output = u64;

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl IndexMut<usize> for Bitvector {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}
