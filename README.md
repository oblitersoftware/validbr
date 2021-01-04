![Doc](https://docs.rs/validbr/badge.svg)
![Crate](https://img.shields.io/crates/v/validbr.svg)
# validbr

Providing data structures and utilities for Brazilian Registries. Validbr is currently capable of validating CPF and CNPJ, but is planned to support:

- RG (structure only)
- CNH (validation and structure)
- CEP (database)
- State and City (database)

## Validation

validbr is capable of validating some Brazilian Registries types in regards of the number of these documents being valid, not in regards of them being registered in Brazilian Organizations. Currently there is no easy way of checking these values against Brazilian Organizations without paid services.

## Databases

validbr will be frequently updated to keep CEP, State and City databases updated. We may add neighbourhood database in the future, initially they will not be supported because the huge amount of them.

Examples:

# CPF
```rust
use validbr::Cpf;
let cpf = Cpf::parse_str("123.456.789-09");
assert_eq!(cpf, Ok(Cpf { digits: [1, 2, 3, 4, 5, 6, 7, 8, 9], verifier_digits: [0, 9]}));
```

# CNPJ

```rust
use validbr::Cnpj;
let cpf = Cnpj::parse_str("12.345.678/0001-95");
assert_eq!(cpf, Ok(Cnpj { digits: [1, 2, 3, 4, 5, 6, 7, 8], branch_digits: [0, 0, 0, 1], verifier_digits: [9, 5]}));
```
