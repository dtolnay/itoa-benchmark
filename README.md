# itoa Benchmark

This benchmark evaluates the performance of Rust libraries for rendering
fixed-precision integer primitives to ASCII base-10 string. 

## Procedure

**Input data:** The benchmark generates random u32, u64, and u128 values with an
equal number of values at every possible combination of type and length. For
example the number of 5-character long u32 values (10000&ndash;99999) will be
equal to the number of 3-character long u64 values (100&ndash;999).

**Measurement:** For each library, for each data type, for each length group, we
perform multiple passes over the input data and take the duration of the fastest
pass.

Build and run the benchmark yourself using `cargo run --release`.

## Results

The following results are measured on a 2025 AMD Ryzen Threadripper 9975WX using
Rust 1.92.0.

![performance](https://raw.githubusercontent.com/dtolnay/itoa-benchmark/master/performance.png)
