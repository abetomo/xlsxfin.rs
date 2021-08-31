# xlsxfin.rs

Excel Financial functions.

## Functions

[docs.rs/xlsxfin](https://docs.rs/xlsxfin/)

* cumipmt
    * [office/cumipmt-function](https://support.microsoft.com/en-us/office/cumipmt-function-61067bb0-9016-427d-b95b-1a752af0e606)
* fv
    * [office/fv-function](https://support.microsoft.com/en-us/office/fv-function-2eef9f44-a084-4c61-bdd8-4fe4bb1b71b3)
* ipmt
    * [office/ipmt-function](https://support.microsoft.com/en-us/office/ipmt-function-5cce0ad6-8402-4a41-8d29-61a0b054cb6f)
* pmt
    * [office/pmt-function](https://support.microsoft.com/en-us/office/pmt-function-0214da64-9a63-4996-bc20-214433fa6441)
* ppmt
    * [office/ppmt-functiooffice/ppmt-functionn](https://support.microsoft.com/en-us/office/ppmt-function-c370d9e3-7749-4ca4-beea-b06c6ac95e1b)

## Example

```rust
extern crate xlsxfin;

fn main() {
    println!("{}", xlsxfin::pmt(0.08 / 12.0, 10, 1_000_000, 0, false));
}
```
