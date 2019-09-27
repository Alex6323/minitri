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

    let trits: T1B1 = trytes.into();
    println!("05: {}", trits);

    let trytes: T3B1 = trits.into();
    println!("06: {}", trytes);

    let bytes9t2b: T9B2 = trytes.into();
    println!("07: {:?}", bytes9t2b);

    let trits: T1B1 = bytes9t2b.into();
    println!("08: {}", trits);

    let trytes: T3B1 = "ABCDE".try_into().expect("error creating tryte sequence");
    println!("09: {}", trytes);

    let bytes5t1b: T5B1 = trytes.into();
    println!("10: {:?}", bytes5t1b);

    let trits: T1B1 = bytes5t1b.into();
    println!("11: {}", trits);

    let trytes: T3B1 = trits.into();
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
