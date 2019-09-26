use trits_module_preview::*;

use std::convert::TryInto;

fn main() {
    let trit: Trit = '-'.try_into().expect("error creating trit");
    println!("01: {}", trit);

    let tryte: Tryte = 'A'.try_into().expect("error crating tryte");
    println!("02: {}", tryte);

    let trits: T1B1 = "1-0".try_into().expect("error creating trit sequence");
    println!("03: {}", trits);

    let trytes: T3B1 = "ABC".try_into().expect("error creating tryte sequence");
    println!("04: {}", trytes);

    let trits = trytes.to_t1b1();
    println!("05: {}", trits);

    let trytes = trits.to_t3b1().expect("error creating T3B1 encoded trits");
    println!("06: {}", trytes);

    let bytes9t2b = trytes.to_t9b2().expect("error creating T9B2 encoded trits");
    println!("07: {:?}", bytes9t2b);

    let trits = bytes9t2b.to_t1b1();
    println!("08: {}", trits);

    let trytes: T3B1 = "ABCDE".try_into().expect("error creating tryte sequence");
    println!("09: {}", trytes);

    let bytes5t1b = trytes.to_t5b1().expect("error creating T5B1 encoded trits");
    println!("10: {:?}", bytes5t1b);

    let trits = bytes5t1b.to_t1b1();
    println!("11: {}", trits);

    let trytes: T3B1 = trits.to_t3b1().expect("error creating T3B1 encoded trits");
    println!("12: {}", trytes);

    // ======================================================================
    // Create `Trits` instance with different internal encodings but same API
    // ======================================================================

    let mut trits: Trits<T1B1> = Trits::new();
    trits.add_trits("10-").unwrap();
    trits.add_trytes("ABC").unwrap();
    println!("13: {}", trits.len());

    let mut trits: Trits<T3B1> = Trits::new();
    trits.add_trits("10-").unwrap();
    trits.add_trytes("ABC").unwrap();
    println!("14: {}", trits.len());

    let mut trits: Trits<T5B1> = Trits::new();
    trits.add_trits("10-1-10-1-").unwrap();
    trits.add_trytes("ABCDE").unwrap();
    println!("15: {}", trits.len());

    let mut trits: Trits<T9B2> = Trits::new();
    trits.add_trits("10--110-0").unwrap();
    trits.add_trytes("ABC").unwrap();
    println!("16: {}", trits.len());
}
