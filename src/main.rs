use halko_rust::bitvectors::Bitvector;

fn main() {
    println!("Hello, world!");
    let a: [u64; 7] = [0,1,0,0,1,1,0];
    let mut bv = Bitvector::build(&a);
    println!("{}", bv[0]);
    println!("{}", bv[1]);
    bv[0] = 1;
    println!("{}", bv[0]);
}
