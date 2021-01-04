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

//! # CPF
//!
//! This module provides utility for constructing and manipulating CPF, as well as validating CPFs. If
//! a CPF was successfully constructed with [`Cpf::new`] or [`Cpf::parse_str`] it means that the CPF
//! is valid.
use crate::append::ArrayAppend;
use crate::{ONLY_NUMBERS, Cpf};
use crate::NOT_NUMBERS;
use crate::convert_to_u8;
use crate::join_to_string;
use crate::cpf::CpfCreationError::CouldNotConvertCpfToDigits;
use regex::Regex;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Formatter;
#[cfg(feature = "rand")]
use {
    rand::distributions::{Distribution, Standard,  Uniform},
    rand::Rng,
};

lazy_static! {
    static ref WELL_FORMATTED_CPF: Regex = Regex::new(r"\d{3}\.\d{3}\.\d{3}-\d{2}").unwrap();
}

/// Formats Cpf in the well known format:
/// 000.000.000-00
/// # Example
///
/// ```
/// use validbr::Cpf;
/// let cpf = Cpf::parse_str("887.614.320-32").expect("Invalid cpf.");
/// assert_eq!(format!("{}", cpf), "887.614.320-32")
/// ```
impl fmt::Display for Cpf {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let f3 = join_to_string!(&self.digits[..3]);
        let m3 = join_to_string!(&self.digits[3..6]);
        let e3 = join_to_string!(&self.digits[6..9]);

        let verifier = join_to_string!(self.verifier_digits);

        write!(f, "{}.{}.{}-{}", f3, m3, e3, verifier)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CpfCreationError {
    /// When provided Cpf digits could not be validated against their verifier digits, in other
    /// words, when provided Cpf is not valid.
    InvalidCpfDigits,
    /// When provided Cpf string is not a valid Cpf format.
    ///
    /// Supported Cpf formats are:
    /// - 000.000.000-00
    /// - 00000000000
    InvalidCpfStringFormat,
    /// When type conversion failure occurs.
    CouldNotConvertCpfToDigits,
    /// When provided Cpf string is too short.
    ShortCpfString,
    /// When provided numbers for digits (cpf digits or validation digits)
    /// are out of bounds, in other words, they are not respecting the range of `0..=9`.
    /// All numbers in the digits array must respect the range `0..=9`.
    DigitsOutOfBounds,
}

#[derive(Debug, Eq, PartialEq)]
pub enum DigitCalculationError {
    /// When the amount of digits provided for calculation does
    WrongAmountOfDigits(usize),
}

type VerifierDigits = (u8, u8);

impl Cpf {
    /// Creates a new Cpf if the provided `[digits]` and `[verifier_digits]` are valid.
    ///
    /// # Example
    /// ```
    /// use validbr::Cpf;
    ///
    /// let cpf = Cpf::new([3, 1, 0, 1, 2, 6, 6, 5, 0], [5, 4]); // Valid CPF
    /// assert!(cpf.is_ok());
    /// ```
    ///
    /// ```
    /// use validbr::Cpf;
    /// use validbr::cpf::CpfCreationError;
    ///
    /// let cpf = Cpf::new([1, 2, 3, 4, 5, 6, 7, 8, 9], [1, 0]); // Invalid CPF
    /// assert_eq!(cpf, Err(CpfCreationError::InvalidCpfDigits));
    /// ```
    pub fn new(digits: [u8; 9], verifier_digits: [u8; 2]) -> Result<Cpf, CpfCreationError> {
        let digits_is_valid = digits.iter().all(|i| *i <= 9);
        let verifier_digits_is_valid = verifier_digits.iter().all(|i| *i <= 9);

        if !digits_is_valid || !verifier_digits_is_valid {
            return Err(CpfCreationError::DigitsOutOfBounds)
        }

        let (first_verifier_digit, second_verifier_digit) = calculate_verifier_digits(digits);

        if first_verifier_digit != verifier_digits[0]
            || second_verifier_digit != verifier_digits[1] {
            Err(CpfCreationError::InvalidCpfDigits)
        } else {
            Ok(Cpf {
                digits,
                verifier_digits,
            })
        }
    }

    /// Parses a Cpf String to a [`Cpf`].
    ///
    /// Supported Cpf formats are:
    ///
    /// - 000.000.000-00
    /// - 00000000000
    ///
    /// # Examples
    ///
    /// ```
    /// use validbr::Cpf;
    /// let cpf = Cpf::parse_str("261.442.230-45");
    /// assert!(cpf.is_ok());
    /// assert_eq!(cpf, Ok(Cpf { digits: [2, 6, 1, 4, 4, 2, 2, 3, 0], verifier_digits: [4, 5]}));
    /// ```
    ///
    /// ```
    /// use validbr::Cpf;
    /// let cpf = Cpf::parse_str("26144223045");
    /// assert!(cpf.is_ok());
    /// assert_eq!(cpf, Ok(Cpf { digits: [2, 6, 1, 4, 4, 2, 2, 3, 0], verifier_digits: [4, 5]}));
    /// ```
    ///
    /// ```
    /// use validbr::Cpf;
    /// let cpf = Cpf::parse_str("12345678909");
    /// assert!(cpf.is_ok());
    /// assert_eq!(cpf, Ok(Cpf { digits: [1, 2, 3, 4, 5, 6, 7, 8, 9], verifier_digits: [0, 9]}));
    /// ```
    pub fn parse_str(cpf: &str) -> Result<Cpf, CpfCreationError> {
        let only_numbers = ONLY_NUMBERS.is_match(cpf);
        if only_numbers && cpf.len() != 11 {
            return Err(CpfCreationError::ShortCpfString);
        }

        if (ONLY_NUMBERS.is_match(cpf) && cpf.len() == 11) || (WELL_FORMATTED_CPF.is_match(cpf)) {
            let cpf_only_with_numbers = NOT_NUMBERS.replace_all(cpf, "");

            let digits_vec: Option<Vec<u8>> =
                convert_to_u8!(cpf_only_with_numbers.chars().take(9)).collect();
            let validators_vec: Option<Vec<u8>> =
                convert_to_u8!(cpf_only_with_numbers.chars().skip(9)).collect();

            let digits_array: Option<[u8; 9]> = digits_vec.and_then(|v| v.try_into().ok());
            let validators_array: Option<[u8; 2]> = validators_vec.and_then(|v| v.try_into().ok());

            if let Some(digits) = digits_array {
                if let Some(validators) = validators_array {
                    Cpf::new(digits, validators)
                } else {
                    Err(CouldNotConvertCpfToDigits)
                }
            } else {
                Err(CouldNotConvertCpfToDigits)
            }
        } else {
            Err(CpfCreationError::InvalidCpfStringFormat)
        }
    }
}

/// Calculates the verifier digit given input `cpf_digits`.
///
/// This function does not care about the amount of digits provided to it, but the correct amount
/// of digits to be provided to this function is either 9 CPF digits or 10 values
/// (9 CPF digits and first verifier digit). When provided with 9 CPF digits, the function calculates
/// the first verifier digits, when provided with 10 values (9 CPF digits and first verifier digit),
/// the function calculates the second verifier digits.
///
/// # Example
///
/// ```
/// use validbr::cpf::calculate_verifier_digit;
/// use validbr::cpf::DigitCalculationError::WrongAmountOfDigits;
///
/// assert_eq!(calculate_verifier_digit([4, 1, 4, 9, 0, 4, 2, 5, 7]), 8);
/// assert_eq!(calculate_verifier_digit([4, 1, 4, 9, 0, 4, 2, 5, 7, 8]), 0);
/// assert_eq!(calculate_verifier_digit([4, 1, 4, 9, 0, 4, 2, 5, 7, 8, 10]), 2);
/// assert_eq!(calculate_verifier_digit([4, 1, 4, 9, 0, 4, 2, 5]), 7);
/// assert_eq!(calculate_verifier_digit([4, 1, 4, 9, 0, 4, 2]), 8);
/// ```
///
pub fn calculate_verifier_digit<const S: usize>(cpf_digits: [u8; S]) -> u8 {
    let modulo_num = S + 1;
    let digits_sum: u16 = cpf_digits
        .iter()
        .enumerate()
        .map(|(pos, digit)| (*digit as u16) * ((modulo_num - pos) as u16))
        .sum();

    let pre_verifier = ((digits_sum * 10) % 11) as u8;
    if pre_verifier == 10 {
        0
    } else {
        pre_verifier
    }
}

/// Calculate both first and second verifier digits, given the `digits` input.
///
/// # Example
///
/// ```
/// use validbr::Cpf;
/// use validbr::cpf::DigitCalculationError::WrongAmountOfDigits;
/// use validbr::cpf::calculate_verifier_digits;
///
/// assert_eq!(calculate_verifier_digits([4, 1, 4, 9, 0, 4, 2, 5, 7]), (8, 0));
/// assert_eq!(calculate_verifier_digits([1, 2, 3, 4, 5, 6, 7, 8, 9]), (0, 9));
/// ```
pub fn calculate_verifier_digits(digits: [u8; 9]) -> VerifierDigits {
    let first_digit = calculate_verifier_digit::<9>(digits);

    let digits_with_first_verifier: [u8; 10] = digits.append(first_digit);
    let second_digit = calculate_verifier_digit::<10>(digits_with_first_verifier);

    (first_digit, second_digit)
}

/// ## Random CPF Example
///
/// ```
/// use validbr::Cpf;
/// use validbr::cpf::*;
/// use rand::Rng;
/// let mut rng = rand::thread_rng();
/// let cpf: Cpf = rng.gen();
/// let verifier = validbr::cpf::calculate_verifier_digits(cpf.digits);
///
/// assert_eq!(verifier.0, cpf.verifier_digits[0]);
/// assert_eq!(verifier.1, cpf.verifier_digits[1]);
/// ```
#[cfg(feature = "rand")]
impl Distribution<Cpf> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cpf {
        let uniform_int = Uniform::from(0u8..=9u8);
        let digits: Vec<u8> = rng.sample_iter(uniform_int)
            .take(9)
            .collect();

        let digits_array: [u8; 9] = digits.try_into()
            .expect("Conversion of Vec with 9 elements MUST be possible at this point.");

        let (first, second) = calculate_verifier_digits(digits_array);

        Cpf::new(digits_array, [first, second])
            .expect("Generated Cpf MUST be valid at this point")
    }
}