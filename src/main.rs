use num_bigint::BigInt;
use num_traits::Num;
mod prime_gen;

fn main() {
    let n_str = "9999999999999999999999999999999999999999999999999999999999999999999999999999999";
    let n = BigInt::from_str_radix(n_str, 10);
    let plist = prime_gen::prime_list(&n.unwrap());
    for i in 0..plist.len() {
        println!("{:?}", plist.get(i).unwrap());
    }
}
