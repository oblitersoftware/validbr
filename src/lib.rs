//     validbr - Brazilian registry validator, provides structures for representing CPF, CNPJ, RG, CNH, CEP and Credit Card Number!
//
//         The MIT License (MIT)
//
//      Copyright (c) Obliter Software (https://github.com/oblitersoftware/)
//      Copyright (c) contributors
//
//      Permission is hereby granted, free of charge, to any person obtaining a copy
//      of this software and associated documentation files (the "Software"), to deal
//      in the Software without restriction, including without limitation the rights
//      to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
//      copies of the Software, and to permit persons to whom the Software is
//      furnished to do so, subject to the following conditions:
//
//      The above copyright notice and this permission notice shall be included in
//      all copies or substantial portions of the Software.
//
//      THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
//      IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
//      FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
//      AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//      LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
//      OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
//      THE SOFTWARE.

//! Representation of Brazilian registries: CPF, CNPJ, RG, CNH, CEP and Credit Card Number.
//!
//! validbr also provides validation for CPJ, CNPJ, CNH and Credit Card Number and a builtin database
//! of brazilian CEP, Cities and States.
//!
//! # Cpf
//!
//! Consist in 9 digits separated in partitions of 3 with `.` and two verifier digits separated by a `-` prefix,
//! for example: `123.456.789-09`.
//!
//! ## Example of usage of CPF struct
//!
//! ```
//! use validbr::Cpf;
//! let cpf = Cpf::parse_str("123.456.789-09");
//! assert_eq!(cpf, Ok(Cpf { digits: [1, 2, 3, 4, 5, 6, 7, 8, 9], verifier_digits: [0, 9]}));
//! ```
//!
//! ## Supported formats
//!
//! [`Cpf::parse_str`] only supports following formats:
//! - `###.###.###-##` (Commonly represented CPF)
//! - `###########` (Only digits CPF).
//!
//! # CNPJ
//!
//! Consists in eight numbers separated by a `.` in partitions for 3 (except for the first two digits
//! which are separated from the last two groups), a company branch number digit composed of four digits,
//! separated by a prefix `/` and two last verifier digits prefixed with `-`, example: `12.345.678/0001-95`.
//!
//! ## Example of usage of CNPJ struct
//!
//! ```
//! use validbr::Cnpj;
//! let cpf = Cnpj::parse_str("12.345.678/0001-95");
//! assert_eq!(cpf, Ok(Cnpj { digits: [1, 2, 3, 4, 5, 6, 7, 8], branch_digits: [0, 0, 0, 1], verifier_digits: [9, 5]}));
//! ```
//!
//! ## Supported formats
//!
//! [`Cnpj::parse_str`] only supports following formats:
//! - `##.###.###/####-##` (Commonly represented CNPJ)
//! - `##############` (Only digits CNPJ).
//!
//! # Features
//!
//! ## [Serde](https://crates.io/crates/serde) support
//!
//! validbr supports [serde](https://crates.io/crates/serde) serialization, which must be enabled with feature flag, for example:
//!
//! ```toml
//! [dependencies]
//! validbr = { version = "0.1", features = ["serde"] }
//! ```
//!
//! ## [rand](https://crates.io/crates/rand) support
//!
//! validbr also supports randomly generated CPF and CNPJ through [rand](https://crates.io/crates/serde) crate,
//! which must be enabled with feature flag, for example:
//!
//! ```toml
//! [dependencies]
//! validbr = { version = "0.1", features = ["rand"] }
//! ```
//!
//! ## Enable all
//!
//! You could enable all features using `complete` flag:
//! ```toml
//! [dependencies]
//! validbr = { version = "0.1", features = ["complete"] }
//! ```
#![feature(doc_cfg)]
#![feature(const_evaluatable_checked, const_generics, const_panic)]
#![allow(incomplete_features)]

#[macro_use]
extern crate lazy_static;

use regex::Regex;

#[macro_use] pub(crate) mod macros;

/// Array append utilities.
pub mod append;
/// Cnpj utility functions
pub mod cnpj;
/// Cpf utility functions
pub mod cpf;

#[cfg(feature = "serde")]
use {
    serde::Serialize,
    serde::Deserialize
};


lazy_static! {
    pub(crate) static ref NOT_NUMBERS: Regex = Regex::new(r"[^0-9]+").unwrap();
    pub(crate) static ref ONLY_NUMBERS: Regex = Regex::new(r"^[0-9]+$").unwrap();
}


/// CPF consists of nine digits and two verifier digits.
///
/// The algorithm to calculate the first verifier digit is:
///
/// ```
/// let digits = [0u16; 9];
/// let checker_digits = [0u16; 2];
///
/// let sum_of_mul = ((digits[8] * 10) + (digits[7] * 9) + (digits[6] * 8) + (digits[5] * 7) + (digits[4] * 6) +
/// (digits[3] * 5) + (digits[2] * 4) + (digits[1] * 3) + (digits[0] * 2)) as u16;
/// let pre_first_digit = ((sum_of_mul * 10) % 11) as u8;
/// let first_digit = if pre_first_digit == 10 {
///     0
/// } else {
///     pre_first_digit
/// };
/// ```
///
/// And the algorithm to calculate the second verifier digit is:
///
/// ```
/// let digits = [0u16; 9];
/// let checker_digits = [0u16; 2];
///
/// let sum_of_mul = ((digits[8] * 11) + (digits[7] * 10) + (digits[6] * 9) + (digits[5] * 8) + (digits[4] * 7) +
/// (digits[3] * 6) + (digits[2] * 5) + (digits[1] * 4) + (digits[0] * 3) + (checker_digits[0] * 2)) as u16;
/// let pre_second_digit = ((sum_of_mul * 10) % 11) as u8;
/// let second_digit = if pre_second_digit == 10 {
///     0
/// } else {
///     pre_second_digit
/// };
/// ```
///
///
/// These numbers could be obtained through `[calculate_verifier_digits]`.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cpf {
    /// First 9 digits of CPF.
    pub digits: [u8; 9],
    /// Last 2 digits of CPF (the verifier digits).
    pub verifier_digits: [u8; 2],
}

/// CNPJ consists of eight based digits, four digits for the branch (the number of the registered
/// company) and two verifier digits.
///
/// The algorithm to calculate the first verifier digit is:
///
/// ```
/// let digits = [0u16; 8];
/// let branch_num_digits = [0u16; 4];
/// let checker_digits = [0u16; 2];
///
/// let sum_of_mul = ((digits[0] * 5) + (digits[1] * 4) + (digits[2] * 3) + (digits[3] * 2) + (digits[4] * 9) +
/// (digits[5] * 8) + (digits[6] * 7) + (digits[7] * 6) + (branch_num_digits[0] * 5) + (branch_num_digits[1] * 4) +
/// (branch_num_digits[2] * 3) + (branch_num_digits[3] * 2)) as u16;
/// let pre_first_digit = (sum_of_mul % 11) as u8;
/// let first_digit = if pre_first_digit  < 2 {
///     0
/// } else {
///     11 - pre_first_digit
/// };
///
/// // And the algorithm to calculate the second verifier digit is:
///
/// let sum_of_mul_2 = ((digits[0] * 5) + (digits[1] * 4) + (digits[2] * 3) + (digits[3] * 2) + (digits[4] * 9) +
/// (digits[5] * 8) + (digits[6] * 7) + (digits[7] * 6) + (branch_num_digits[0] * 5) + (branch_num_digits[1] * 4) +
/// (branch_num_digits[2] * 3) + (branch_num_digits[3] * 2)) as u16;
/// let pre_second_digit = (sum_of_mul_2 % 11) as u8;
/// let second_digit = if pre_second_digit  < 2 {
///     0
/// } else {
///     11 - pre_second_digit
/// };
/// ```
///
///
/// These numbers could be obtained through `[calculate_verifier_digits]`.
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Cnpj {
    /// First 8 digits of Cnpj.
    pub digits: [u8; 8],
    /// Four digits of branch.
    pub branch_digits: [u8; 4],
    /// Last 2 digits of CPF (the verifier digits).
    pub verifier_digits: [u8; 2],
}

#[cfg(test)]
mod tests {

    #[cfg(feature = "rand")]
    #[test]
    fn random_cpf() {
        use rand::Rng;
        use crate::Cpf;

        let mut rng = rand::thread_rng();
        let cpf: Cpf = rng.gen();
        let verifier = crate::cpf::calculate_verifier_digits(cpf.digits);
        assert_eq!(verifier.0, cpf.verifier_digits[0]);
        assert_eq!(verifier.1, cpf.verifier_digits[1]);
    }

    #[cfg(feature = "rand")]
    #[test]
    fn random_cnpj() {
        use rand::Rng;
        use crate::Cnpj;

        let mut rng = rand::thread_rng();
        let cnpj: Cnpj = rng.gen();
        let verifier = crate::cnpj::calculate_verifier_digits(cnpj.digits, cnpj.branch_digits);
        assert_eq!(verifier.0, cnpj.verifier_digits[0]);
        assert_eq!(verifier.1, cnpj.verifier_digits[1]);
    }

    #[cfg(feature = "rand")]
    #[test]
    fn random_cnpj_with_specific_branch() {
        use rand::Rng;
        use crate::Cnpj;
        use crate::cnpj::Branch;

        let mut rng = rand::thread_rng();
        let branch = Branch::from_u8(0012).unwrap();
        let cnpj: Cnpj = rng.sample(branch);

        assert_eq!([0, 0, 1, 2], cnpj.branch_digits);

        let verifier = crate::cnpj::calculate_verifier_digits(cnpj.digits, cnpj.branch_digits);
        assert_eq!(verifier.0, cnpj.verifier_digits[0]);
        assert_eq!(verifier.1, cnpj.verifier_digits[1]);
    }
}
