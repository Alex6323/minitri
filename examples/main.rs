use minitri::{BalancedTrit, BalancedTryte, Encoding, TritVec, T1B1, T3B1, T5B1};

fn main() {
    // Create single trits and trytes from their char repr.
    let trit: BalancedTrit = '-'.into();
    let tryte: BalancedTryte = 'A'.into();

    // Create trit sequences from their str repr.
    let t1b1: T1B1 = "1-0".into();

    // Convert between encodings (1)
    let t3b1: T3B1 = "ABC".into();
    let t1b1: T1B1 = t3b1.into();
    let t3b1: T3B1 = t1b1.into();
    //let t9b2: T9B2 = t3b1.into();
    //let t1b1: T1B1 = t9b2.into();

    // Convert between encodings (2)
    let t3b1: T3B1 = "ABCDE".into();
    let t5b1: T5B1 = t3b1.into();
    let t1b1: T1B1 = t5b1.into();
    let t3b1: T3B1 = t1b1.into();

    // Create trit vectors that use a particular ternary encoding
    let mut vec: TritVec<T1B1> = TritVec::new();
    vec.push("10-");
    vec.push("ABC");

    let mut vec: TritVec<T3B1> = TritVec::new();
    vec.push("10-");
    vec.push("ABC");

    let mut vec: TritVec<T5B1> = TritVec::new();
    vec.push("10-1-10-1-");
    vec.push("ABCDE");

    //let mut vec: TritVec<T9B2> = TritVec::new();
    //vec.push("10--110-0");
    //vec.push("ABC");
}
