# Trit-Encodings

An utterly simple trinary/ternary crate.

# Intent

This crate is for testing ternary stuff quickly. It is not intended for production use.

# Properties
    * no dependencies
    * all important byte encodings required in IOTA
    * minimalistic
    * convenient to use
    * simply panics on conversion errors
    * no unsafe code

# Usage

```Rust
// Create single trits and trytes from their char repr.
let trit: BalancedTrit = '-'.into();
let tryte: BalancedTryte = 'A'.into();

// Create trit sequences from their str repr.
let t1b1: T1B1 = "1-0".into();

// Convert between encodings (1)
let t3b1: T3B1 = "ABC".into();
let t1b1: T1B1 = t3b1.into();
let t3b1: T3B1 = t1b1.into();

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
```