**RIG: A bigWig reader in Rust**

Just a WIP for fun.

```rust
let mut bw = BigWig::from_path("ENCFF457TKX.bigWig").open().unwrap();
println!("{}", bw.read_magic_number().unwrap());
println!("{}", bw.is_bigwig());
// 2291137574
// true
```
