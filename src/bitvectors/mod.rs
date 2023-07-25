//pub mod bitvector;
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
        //let v: Vec<u64> = Vec::new();
        Bitvector {
            data: vec![0; n],
            n,
        }

    }

    pub fn len(&self) -> usize {
        self.n
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
