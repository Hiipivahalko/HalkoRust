use std::vec::Vec;
use crate::bitvectors::{Bitvector, Bit};

#[cfg(test)]
mod tests;

pub struct RankSupport {
    bv: Bitvector,
    _block_level1: Vec<u64>,
    _block_level2: Vec<u64>,
}

impl RankSupport {
    //const DATA_BLOCK_L: usize = 64;

    pub fn new(bv: Bitvector) -> RankSupport {
        let b1_size = bv.len().ilog2().pow(2) as usize;
        let b1_size = if b1_size > bv.len() {0} else {b1_size}; // this happens only with low
                                                                // values
        let b2_size = bv.len().ilog2() as usize;

        println!("b1_size:{}, b2_size:{}", b1_size, b2_size);
        let b1 = if b1_size == 0 {0} else {bv.len()/b1_size};
        let b2 = if b2_size == 0 {0} else {bv.len()/b2_size};

        println!("b1:{}, b2:{}, bv_len:{}", b1, b2, bv.len());


        let mut v1 = vec![0; b1];
        let mut v2 = vec![0; b2];

        if b1_size > 0 {

        for (i, ran_val) in (0..=bv.len()-b1_size).step_by(b1_size).enumerate() {
            // block_level1
            if i == 0 {
                v1[i] = bv.scan_blocks(ran_val, ran_val+b1_size-1, Bit::ONE, u64::MAX).0;
            } else {
                v1[i] = v1[i-1] + bv.scan_blocks(ran_val, ran_val+b1_size-1, Bit::ONE, u64::MAX).0;
            }

            // block_level2
            v2[i*b2_size] = bv.scan_blocks(ran_val, ran_val+b2_size-1, Bit::ONE, u64::MAX).0;

            for j in 1..b2_size {
                let block_start = ran_val + (j-1)*b2_size;
                let block_stop = ran_val + j*b2_size -1;
                let k: usize = i*b2_size+j;
                v2[k] = v2[k-1] + bv.scan_blocks(block_start, block_stop, Bit::ONE, u64::MAX).0;
            }
        }
        }

        // compute last blocks for block_level2
        if b1*b1_size + b2_size <= bv.len() && v2.len() > 0 {
            let start_i = v2.len()-b2%b2_size;
            v2[start_i] = bv.scan_blocks(start_i*b2_size, (start_i+1)*b2_size-1, Bit::ONE, u64::MAX).0;

            for j in 1..b2%b2_size {
                let block_start = start_i*b2_size + j*b2_size;
                let block_stop = block_start + b2_size -1;
                v2[start_i+j] = v2[start_i+j-1] + bv.scan_blocks(block_start, block_stop, Bit::ONE, u64::MAX).0;
            }
        }

        let v2: Vec<u64> = (0..v2.len()).filter(|i| (i+1)%b2_size != 0).map(|i| v2[i]).collect();

        //println!("v1:{:?}", v1);
        //println!("v2:{:?}", v2);

        RankSupport {
            bv,
            _block_level1: v1,
            _block_level2: v2,
        }
    }

    pub fn get_bv(&self) -> &Bitvector {
        &self.bv
    }

    pub fn get_block_level1(&self) -> &Vec<u64> {
        &self._block_level1
    }

    pub fn get_block_level2(&self) -> &Vec<u64> {
        &self._block_level2
    }

}
