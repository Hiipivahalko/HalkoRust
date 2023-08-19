use std::vec::Vec;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

pub enum Bit {
    ZERO,
    ONE,
}

impl Bit {
    pub fn value(&self) -> u32 {
        match *self {
           Bit::ZERO => 0,
           Bit::ONE => 1,
        }
    }
}

/// Simple bitvector implementation.
/// Use struct's build-functions to building/initializing Bitvector
///
/// # Example
///
/// ```
/// use halko_rust::bitvectors::{Bitvector, Bit};
///
/// // [0,0,0,0,0]
/// let mut bv = Bitvector::build_empty(5);
///
/// for i in 0..5 { assert_eq!(0, bv.get(i)); }
///
/// bv.set(0, Bit::ONE); // bv = [1,0,0,0,0];
/// assert_eq!(bv.get(0), 1);
///
/// bv.set(3, Bit::ONE); // bv = [1,0,0,1,0];
/// assert_eq!(bv.get(3), 1);
/// ```
///
/// * Bitvector can be builded from the array containing 0s and 1s.
/// ```
/// use halko_rust::bitvectors::Bitvector;
///
/// let a: [u32; 7] = [0,1,0,0,1,1,0];
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
    /// let a: [u32; 7] = [0,1,0,0,1,1,0];
    /// let bv = Bitvector::build(&a);
    ///
    /// for i in 0..a.len() { assert_eq!(a[i], bv.get(i)); }
    /// ```
    pub fn build(arr: &[u32]) -> Bitvector {
        let mut bv = Bitvector::build_empty(arr.len());
        for (i, v) in arr.iter().enumerate() {
            match v {
                0 => bv.set(i, Bit::ZERO),
                _ => bv.set(i, Bit::ONE)

            }
        }

        bv
    }

    /// Builds bitvector from vector containing positive integers.
    /// Copies only input vector values into data variable and length
    /// of the result bitvector is `v.len()`*64.
    ///
    /// ```
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
    /// ```
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let bv1 = Bitvector::build_empty(5);
    /// assert_eq!(bv1.len(), 5);
    /// ```
    pub fn len(&self) -> usize {
        self.n
    }

    /// Returns bit value in the i-th bit.
    pub fn get(&self, i: usize) -> u32 {
        const I: u32 = 1;
        (&self.data[i/64] >> (i%64)) as u32 & I
    }

    /// Sets or unsets the i-th bit in the bitvector.
    ///
    /// ```
    /// use halko_rust::bitvectors::{Bitvector, Bit};
    /// let mut bv = Bitvector::build_empty(5); // [0,0,0,0,0]
    ///
    /// bv.set(1, Bit::ONE); // [0,1,0,0,0]
    /// bv.set(1, Bit::ZERO); // [0,0,0,0,0]
    /// ```
    pub fn set(&mut self, i: usize, val: Bit) {
        const I: u64 = 1;
        match val {
            Bit::ZERO => self.data[i/64] &= !(I << (i%64)),
            Bit::ONE => self.data[i/64] |= I << (i%64),
        }
    }

    /// Returns numbers of 1s in the bitvector in range `[0,i]`.
    ///
    /// ```
    /// use std::panic;
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let a: [u32; 7] = [0,1,0,0,1,1,0];
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
        let m_i = self.data[0..(i/64)].iter().fold(0, |acc, &x| (acc + x.count_ones() as u64));

        m_i + ones_in_block
    }

    /// Returns numbers of 0s in the bitvector in range `[0,i]`.
    ///
    /// ```
    /// use std::panic;
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let a: [u32; 7] = [0,1,0,0,1,1,0];
    /// let bv = Bitvector::build(&a);
    ///
    /// assert_eq!(bv.rank0(0), 1);
    /// assert_eq!(bv.rank0(1), 1);
    /// assert_eq!(bv.rank0(2), 2);
    /// assert_eq!(bv.rank0(6), 4);
    ///
    /// let panic_result = panic::catch_unwind(|| {
    ///     bv.rank1(7)
    /// });
    /// assert!(panic_result.is_err());
    /// ```
    pub fn rank0(&self, i: usize) -> u64 {
        (i as u64+1) - self.rank1(i)
    }

    /// Returns index of `i`-th 1bit in the bitvector.
    /// Function panics if `i>m`, where `m` is number of ones in the bitvector.
    ///
    /// ```
    /// use std::panic;
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let a: [u32; 7] = [0,1,0,0,1,1,0];
    /// let bv = Bitvector::build(&a);
    ///
    /// assert_eq!(bv.select1(1), 1);
    /// assert_eq!(bv.select1(2), 4);
    /// assert_eq!(bv.select1(3), 5);
    ///
    /// let panic_result = panic::catch_unwind(|| {
    ///     bv.select1(4)
    /// });
    /// assert!(panic_result.is_err());
    /// ```
    pub fn select1(&self, i: usize) -> usize {
        if i > self.n {
            panic!("Select query out of the bitvector range -> i:{}, length of bitvector:{}", i, self.n);
        }

        if i == 0 {
            panic!("Input value i must be greater than 0 (zero). There is not 0th 1bit in the bitvector");
        }

        let mut indicies_before_select_i: usize = 0;
        // idea: loop first blocks until limit found and then final block
        for (k, x) in self.data.iter().enumerate() {
            let ones_in_x = x.count_ones() as usize;
            if indicies_before_select_i + ones_in_x >= i {
                for j in 0..64 { // loop final block in self.data
                    // this applies only for the last block in self.data
                    // when last block contains redudant bits (n%64 != 0)
                    if j + (k*64) >= self.n {
                        break;
                    }

                    if self.get(j+(k*64)) == 1 {
                        indicies_before_select_i += 1;
                    }

                    if indicies_before_select_i == i {
                        return k*64 + j;
                    }
                }
                panic!("Something went wrong when computing the select value, select1 function have error!");
            }
            indicies_before_select_i += ones_in_x;
        }

        panic!(">> Error, bitvector do not have {}th bit ->
               numbers of 1s in the bitvector is less than {}", i,i,);
    }

    /// Returns index of `i`-th 0bit in the bitvector.
    /// Function panics if `i>m`, where `m` is number of zeros in the bitvector.
    ///
    /// ```
    /// use std::panic;
    /// use halko_rust::bitvectors::Bitvector;
    ///
    /// let a: [u32; 7] = [0,1,0,0,1,1,0];
    /// let bv = Bitvector::build(&a);
    ///
    /// assert_eq!(bv.select0(1), 0);
    /// assert_eq!(bv.select0(2), 2);
    /// assert_eq!(bv.select0(3), 3);
    /// assert_eq!(bv.select0(4), 6);
    ///
    /// let panic_result = panic::catch_unwind(|| {
    ///     bv.select0(5)
    /// });
    /// assert!(panic_result.is_err());
    /// ```
    pub fn select0(&self, i: usize) -> usize {
        if i > self.n {
            panic!("Select query out of the bitvector range -> i:{}, length of bitvector:{}", i, self.n);
        }

        if i == 0 {
            panic!("Input value i must be greater than 0 (zero). There is not 0th 0bit in the bitvector");
        }

        let mut indicies_before_select_i: usize = 0;
        // idea: loop first blocks until limit found and then final block
        for (k, x) in self.data.iter().enumerate() {
            let zeros_in_x = x.count_zeros() as usize;
            if indicies_before_select_i + zeros_in_x >= i {
                for j in 0..64 { // loop the next block after limit in self.data
                    // this applies only for the last block in self.data
                    // when last block contains redudant bits (n%64 != 0)
                    if j + (k*64) >= self.n {
                        break;
                    }

                    if self.get(j+(k*64)) == 0 {
                        indicies_before_select_i += 1;
                    }

                    if indicies_before_select_i == i {
                        return k*64 + j;
                    }
                }
                panic!("Something went wrong when computing the select value, select1 function have error!");
            }
            indicies_before_select_i += zeros_in_x;
        }

        panic!(">> Error, bitvector do not have {}th bit ->
               numbers of 0s in the bitvector is less than {}", i,i,);
    }
}

impl Index<usize> for Bitvector {
    type Output = u64;

    /// ```
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
    /// ```
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
