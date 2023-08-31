use std::vec::Vec;
use crate::bitvectors::{Bitvector, Bit};

#[cfg(test)]
mod tests;

/// Succint rank support for bitvector, that allows O(1) time rank query for the bitvector.
/// The structure consist of two level precomputed rank results.
/// First block level stores rank results for indicies `[b1, 2*b1, ... , (|bv|/b1)*b1]` from the bitvector `bv`.
/// The value `b1` is computed by `b1=(log_2(|bv|)^2`.
///
/// The second level computes relative rank values between two blocks in the first block level.
/// Such that `block_level2[i] = rank1(bv, i*b2) - block_level1[i*b2/b1]`, where `b2=log_2(|bv|)`.
pub struct RankSupport {
    bv: Bitvector,
    block_level1: Vec<u64>,
    block_level2: Vec<u64>,
    b1: usize,
    b2: usize,
}

impl RankSupport {

    pub fn new(bv: Bitvector) -> RankSupport {
        let b1 = bv.len().ilog2().pow(2) as usize;
        let b1 = if b1 > bv.len() {0} else {b1}; // this happens only with low
                                                 // values
        let b2 = bv.len().ilog2() as usize;
        //println!("b1:{}, b2:{}", b1, b2);

        let b1_n = if b1 == 0 {1} else {(bv.len()+b1-1)/b1}; // ceil(bv.len()/b1)
        let b2_n = if b2 == 0 {1} else {(bv.len()+b2-1)/b2}; // ceil(bv.len()/b2)

        let mut v1 = vec![0; b1_n];
        let mut v2 = vec![0; b2_n];

        // first block_level2 values inside first value in block_level1
        // if b2=0, then there are not block in level 2
        if b2 > 0 {
            let k = if b1 <= bv.len() {b1/b2} else {(bv.len()%b1)+b2-1/b2};
            for j in 1..k {
                let start = (j-1)*b2;
                let stop = start+b2-1;
                let b2_i = j;
                v2[b2_i] = v2[b2_i-1] + bv.scan_blocks(start, stop, Bit::ONE, u64::MAX).0;
            }
        }

        for i in 1..b1_n {
            v1[i] = v1[i-1] + bv.scan_blocks((i-1)*b1, i*b1-1, Bit::ONE, u64::MAX).0;


            let k = if i*b1 + b1 <= bv.len() {b1/b2} else {((bv.len()%b1)+b2-1)/b2};
            for j in 1..k {
                let start = i*b1 + (j-1)*b2;
                let stop = start+b2-1;
                let b2_i = i*b2+j;
                //println!("j:{}, i:{}, k:{}", j,i,k);
                v2[b2_i] = v2[b2_i-1] + bv.scan_blocks(start, stop, Bit::ONE, u64::MAX).0;
            }
        }

        RankSupport {
            bv,
            block_level1: v1,
            block_level2: v2,
            b1,
            b2,
        }
    }

    pub fn get_bv(&self) -> &Bitvector {
        &self.bv
    }

    pub fn get_block_level1(&self) -> &Vec<u64> {
        &self.block_level1
    }

    pub fn get_block_level2(&self) -> &Vec<u64> {
        &self.block_level2
    }
    }

}
