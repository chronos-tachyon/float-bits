# float-bits
Floats stored as raw bits, making them hashable and totally ordered.

The types in this crate represent IEEE 754 binary floating point numbers,
using unsigned integers to store the raw bits of the floats.  Some of these
types represent float formats not supported by Rust, or only if specific Rust
features are available and enabled.  A limited suite of operations are
available that operate directly on the raw bit representation, bypassing the
need for Rust support.
