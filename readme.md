# Excel Column ID

Convert to Excel Column name from number

[crates-url]: https://crates.io/crates/excel_column_id


## Usage

To use `excel_column_id`, first add this to your `Cargo.toml`:

```toml
[dependencies]
excel_column_id = "0.1"
```

Next, add this to your crate:

```rust
use excel_column_id::ColumnId;

let column_a = ColumnId::from(1);
let column_z = ColumnId::from(26);
```

## License

This project is licensed under the [MIT license](LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in `bytes` by you, shall be licensed as MIT, without any additional
terms or conditions.

