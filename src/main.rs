use halko_rust::bitvectors::Bitvector;

fn main() {
    println!("Hello, world!");
    let a: [u64; 7] = [0,1,0,0,1,1,0];
    let mut bv = Bitvector::build(&a);
    println!("{}", bv.get(0));
    println!("{}", bv.get(1));
    bv.set(0,1);
    println!("{}", bv.get(0));
}
