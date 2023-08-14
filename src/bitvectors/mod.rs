use std::vec::Vec;
use std::ops::{Index, IndexMut};

#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct Bitvector {
    data: Vec<u64>,
    n: usize,
}

impl Bitvector {
    pub fn build(arr: &[u64]) -> Bitvector {
        Bitvector {
            data: arr.to_vec(),
            n: arr.len(),
        }
    }

    pub fn build_empty(n: usize) -> Bitvector {
        Bitvector {
            data: vec![0; (n/64) + 1],
            n,
        }

    }

    pub fn build(arr: &[u64]) -> Bitvector {
        let mut bv = Bitvector::build_empty(arr.len());
        for (i, v) in arr.iter().enumerate() {
            bv.set(i, *v);
            println!("v: {}, bv[{}]: {}", *v, i, bv.get(i));
        }

        bv
    }
    pub fn len(&self) -> usize {
        self.n
    }

    pub fn get(&self, i: usize) -> u64 {
        let one: u64 = 1;
        (&self.data[i/64] >> (i%64)) & one
    }

    pub fn set(&mut self, i: usize, val: u32) {
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
