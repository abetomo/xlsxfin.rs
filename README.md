# xlsxfin.rs

```rust
extern crate xlsxfin;

fn main() {
    println!("{}", xlsxfin::pmt(0.08 / 12.0, 10, 1_000_000, 0, false));
}
```
