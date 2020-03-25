//! Encoding, where 8 trits are stored in two bytes.
//!
//! Advantages:
//!     * Better memory-efficiency than T3B1, and same as T4B1.
//!     * If the `hi` part is stored in byte 1, and the `lo` part in byte 2 at the same
//!         relative position, then this form of vectorization allows high-level SIMD,
//!         and therefore for the best ternary processing performance.
//! Disadvantages:
//!     * There are still encodings with better memory efficiency (see T5B1 and T9B2).
