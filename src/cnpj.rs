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

//! # CNPJ
//!
//! This module provides utility for constructing and manipulating CNPJ, as well as validating CNPJs. If
//! a CNPJ was successfully constructed with [`Cnpj::new`] or [`Cnpj::parse_str`] it means that the CNPJ
//! is valid.
//!
//!
//!
use crate::append::ArrayAppend;
use crate::cnpj::CnpjCreationError::CouldNotConvertCnpjToDigits;
use crate::{ONLY_NUMBERS, Cnpj};
use crate::NOT_NUMBERS;
use crate::convert_to_u8;
use crate::join_to_string;
use regex::Regex;
use std::convert::TryInto;
use std::fmt;
use std::fmt::Formatter;
#[cfg(feature = "rand")]
use {
    rand::distributions::{Distribution, Standard, Uniform},
    rand::Rng,
};

lazy_static! {
    static ref WELL_FORMATTED_CNPJ: Regex = Regex::new(r"\d{2}\.\d{3}\.\d{3}/\d{4}-\d{2}").unwrap();
}

/// Formats Cnpj in the well known format:
/// 00.000.000/0000-00
/// # Example
///
/// ```
/// use validbr::Cnpj;
///
/// let cnpj = Cnpj::parse_str("80.906.404/0001-88").expect("Invalid cnpj.");
/// assert_eq!(format!("{}", cnpj), "80.906.404/0001-88")
/// ```
impl fmt::Display for Cnpj {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let f3 = join_to_string!(&self.digits[..2]);
        let m3 = join_to_string!(&self.digits[2..5]);
        let e3 = join_to_string!(&self.digits[5..8]);

        let branch = join_to_string!(self.branch_digits);
        let verifier = join_to_string!(self.verifier_digits);

        write!(f, "{}.{}.{}/{}-{}", f3, m3, e3, branch, verifier)
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum CnpjCreationError {
    /// When provided Cnpj digits could not be validated against their verifier digits, in other
    /// words, when provided Cnpj is not valid.
    InvalidCnpjDigits,
    /// When provided Cnpj string is not a valid Cnpj format.
    ///
    /// Supported Cpf formats are:
    /// - 00.000.000/0000-00
    /// - 00000000000000
    InvalidCnpjStringFormat,
    /// When type conversion failure occurs.
    CouldNotConvertCnpjToDigits,
    /// When provided Cnpj string is too short.
    ShortCnpjString,
    /// When provided numbers for digits (cnpj digits, branch digits or validation digits)
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

impl Cnpj {
    /// Creates a new Cnpj if the provided `[digits]`, `[branch_digits]` and `[verifier_digits]` are valid.
    ///
    /// # Example
    /// ```
    /// use validbr::Cnpj;
    ///
    /// let cnpj = Cnpj::new([5, 3, 8, 7, 1, 1, 4, 3], [0, 0, 0, 1], [3, 5]); // Valid CPF
    /// assert!(cnpj.is_ok());
    /// ```
    ///
    /// ```
    /// use validbr::Cnpj;
    /// use validbr::cnpj::CnpjCreationError;
    ///
    /// let cnpj = Cnpj::new([8, 0, 9, 0, 6, 4, 0, 4], [0, 0, 0, 3], [8, 8]); // Invalid CPF
    /// assert_eq!(cnpj, Err(CnpjCreationError::InvalidCnpjDigits));
    /// ```
    pub fn new(
        digits: [u8; 8],
        branch_digits: [u8; 4],
        verifier_digits: [u8; 2],
    ) -> Result<Cnpj, CnpjCreationError> {
        let digits_is_valid = digits.iter().all(|i| *i <= 9);
        let branch_digits_is_valid = branch_digits.iter().all(|i| *i <= 9);
        let verifier_digits_is_valid = verifier_digits.iter().all(|i| *i <= 9);

        if !digits_is_valid || !branch_digits_is_valid || !verifier_digits_is_valid {
            return Err(CnpjCreationError::DigitsOutOfBounds)
        }

        let (first_verifier_digit, second_verifier_digit) =
            calculate_verifier_digits(digits, branch_digits);

        if first_verifier_digit != verifier_digits[0]
            || second_verifier_digit != verifier_digits[1] {
            Err(CnpjCreationError::InvalidCnpjDigits)
        } else {
            Ok(Cnpj {
                digits,
                branch_digits,
                verifier_digits,
            })
        }
    }

    /// Parses a Cnpj String to a [`Cnpj`].
    ///
    /// Supported Cnpj formats are:
    ///
    /// - 000.000.000-00
    /// - 00000000000
    ///
    /// # Examples
    ///
    /// ```
    /// use validbr::Cnpj;
    /// let cnpj = Cnpj::parse_str("53.871.143/0001-35");
    /// assert!(cnpj.is_ok());
    /// assert_eq!(cnpj, Ok(Cnpj { digits: [5, 3, 8, 7, 1, 1, 4, 3], branch_digits: [0, 0, 0, 1], verifier_digits: [3, 5]}));
    /// ```
    ///
    /// ```
    /// use validbr::Cnpj;
    /// let cnpj = Cnpj::parse_str("53871143000135");
    /// assert!(cnpj.is_ok());
    /// assert_eq!(cnpj, Ok(Cnpj { digits: [5, 3, 8, 7, 1, 1, 4, 3], branch_digits: [0, 0, 0, 1], verifier_digits: [3, 5]}));
    /// ```
    pub fn parse_str(cnpj: &str) -> Result<Cnpj, CnpjCreationError> {
        let only_numbers = ONLY_NUMBERS.is_match(cnpj);
        if only_numbers && cnpj.len() != 14 {
            return Err(CnpjCreationError::ShortCnpjString);
        }

        return if (ONLY_NUMBERS.is_match(cnpj) && cnpj.len() == 14) || (WELL_FORMATTED_CNPJ.is_match(cnpj))
        {
            let cnpj_only_with_numbers = NOT_NUMBERS.replace_all(cnpj, "");

            let digits_vec: Option<Vec<u8>> =
                convert_to_u8!(cnpj_only_with_numbers.chars().take(8)).collect();
            let branch_digits_vec: Option<Vec<u8>> =
                convert_to_u8!(cnpj_only_with_numbers.chars().skip(8).take(4)).collect();
            let validators_vec: Option<Vec<u8>> =
                convert_to_u8!(cnpj_only_with_numbers.chars().skip(12)).collect();

            let digits_array: Option<[u8; 8]> = digits_vec.and_then(|v| v.try_into().ok());
            let branch_digits_array: Option<[u8; 4]> =
                branch_digits_vec.and_then(|v| v.try_into().ok());
            let validators_array: Option<[u8; 2]> = validators_vec.and_then(|v| v.try_into().ok());

            if let Some(digits) = digits_array {
                if let Some(validators) = validators_array {
                    if let Some(branch_digits) = branch_digits_array {
                        Cnpj::new(digits, branch_digits, validators)
                    } else {
                        Err(CouldNotConvertCnpjToDigits)
                    }
                } else {
                    Err(CouldNotConvertCnpjToDigits)
                }
            } else {
                Err(CouldNotConvertCnpjToDigits)
            }
        } else {
            Err(CnpjCreationError::InvalidCnpjStringFormat)
        };
    }
}

/// Calculates the verifier digit given input `[cnpj_digits]`.
///
/// This function does not care about the amount of digits provided to it, but the correct amount
/// of digits to be provided to this function is either 12 CNPJ digits with branch digits or 13 values
/// (12 CNPJ digits with branch digits and first verifier digit). When provided with 12 CPF digits, the function calculates
/// the first verifier digits, when provided with 13 values (12 CNPJ digits and first verifier digit),
/// the function calculates the second verifier digits.
///
/// # Example
///
/// ```
/// use validbr::cnpj::calculate_verifier_digit;
/// use validbr::cnpj::DigitCalculationError::WrongAmountOfDigits;
///
/// assert_eq!(calculate_verifier_digit([5, 3, 8, 7, 1, 1, 4, 3, 0, 0, 0, 1]), 3);
/// assert_eq!(calculate_verifier_digit([5, 3, 8, 7, 1, 1, 4, 3, 0, 0, 0, 1, 3]), 5);
///
/// assert_eq!(calculate_verifier_digit([3, 4, 8, 5, 4, 6, 7, 8, 0, 0, 0, 1]), 5);
/// assert_eq!(calculate_verifier_digit([3, 4, 8, 5, 4, 6, 7, 8, 0, 0, 0, 1, 5]), 3);
///
/// assert_eq!(calculate_verifier_digit([3, 1, 2, 3, 8, 8, 2, 6, 0, 0, 0, 1]), 1);
/// assert_eq!(calculate_verifier_digit([3, 1, 2, 3, 8, 8, 2, 6, 0, 0, 0, 1, 1]), 7);
/// ```
///
pub fn calculate_verifier_digit<const S: usize>(cnpj_digits: [u8; S]) -> u8 {
    let mul_digits: Vec<u8> = get_multiplier_values(S);

    let digits_sum: u16 = cnpj_digits
        .iter()
        .enumerate()
        .map(|(pos, digit)| (*digit as u16) * (mul_digits[pos] as u16))
        .sum();

    let pre_verifier_digit = (digits_sum % 11) as u8;

    if pre_verifier_digit < 2 {
        0
    } else {
        11 - pre_verifier_digit
    }
}

/// Calculates the multiplier values for CNPJ verifier digit calculation given the `[amount]`
/// of digits.
///
/// The multiplier values must always end in 9, cycling from 9 to 2, and when reach 2, starts in 9 again
/// until 2.
///
/// # Example
///
/// ```
/// use validbr::cnpj::get_multiplier_values;
///
/// assert_eq!(get_multiplier_values(12), vec![5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
/// assert_eq!(get_multiplier_values(13), vec![6, 5, 4, 3, 2, 9, 8, 7, 6, 5, 4, 3, 2]);
/// ```
///
pub fn get_multiplier_values(amount: usize) -> Vec<u8> {
    let range = 2u8..=9u8;
    let range_rev_cycle = range.cycle();
    range_rev_cycle
        .take(amount)
        .collect::<Vec<u8>>()
        .into_iter()
        .rev()
        .collect()
}

/// Calculate both first and second verifier digits, given the `[digits]` input.
///
/// # Example
///
/// ```
/// use validbr::Cnpj;
/// use validbr::cnpj::calculate_verifier_digits;
/// use validbr::cnpj::DigitCalculationError::WrongAmountOfDigits;
///
/// assert_eq!(calculate_verifier_digits([2, 7, 1, 4, 8, 7, 3, 4], [0, 0, 0, 1]), (7, 9));
/// assert_eq!(calculate_verifier_digits([1, 2, 3, 4, 5, 6, 7, 8], [9, 0, 1, 2]), (3, 0));
/// ```
pub fn calculate_verifier_digits(digits: [u8; 8], branch_digits: [u8; 4]) -> VerifierDigits {
    let cnpj_digits: [u8; 12] = digits.append_array::<4>(branch_digits);
    let first_digit = calculate_verifier_digit::<12>(cnpj_digits);

    let digits_with_first_verifier: [u8; 13] = cnpj_digits.append(first_digit);
    let second_digit = calculate_verifier_digit::<13>(digits_with_first_verifier);

    (first_digit, second_digit)
}

/// ## Random CNPJ Example
///
/// ```
/// use validbr::Cnpj;
/// use validbr::cnpj::*;
/// use rand::Rng;
/// let mut rng = rand::thread_rng();
/// let cnpj: Cnpj = rng.gen();
///
/// let verifier = validbr::cnpj::calculate_verifier_digits(cnpj.digits, cnpj.branch_digits);
/// assert_eq!(verifier.0, cnpj.verifier_digits[0]);
/// assert_eq!(verifier.1, cnpj.verifier_digits[1]);
/// ```
#[cfg(feature = "rand")]
impl Distribution<Cnpj> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cnpj {
        let uniform_int = Uniform::from(0u8..=9u8);
        let digits: Vec<u8> = rng.sample_iter(uniform_int)
            .take(8)
            .collect();

        let branch_digits: Vec<u8> = rng.sample_iter(uniform_int)
            .take(4)
            .collect();

        let digits_array: [u8; 8] = digits.try_into()
            .expect("Conversion of Vec with 8 elements MUST be possible at this point.");

        let branch_digits_array: [u8; 4] = branch_digits.try_into()
            .expect("Conversion of Vec with 4 elements MUST be possible at this point.");

        let (first, second) = calculate_verifier_digits(digits_array, branch_digits_array);

        Cnpj::new(digits_array, branch_digits_array, [first, second])
            .expect("Generated Cnpj MUST be valid at this point")
    }
}

/// Struct object used to generate random [`Cnpj`] based in provided [`Branch::0`] instead of
/// generating a random branch digit.
#[cfg_attr(feature = "rand", derive(Debug, Eq, PartialEq, Hash, Clone))]
pub struct Branch([u8; 4]);

#[cfg(feature = "rand")]
impl fmt::Display for Branch {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", join_to_string!(self.0))
    }
}

/// When branch number is not in the range of `0..=9999`.
#[cfg_attr(feature = "rand", derive(Debug, Eq, PartialEq, Hash, Clone))]
pub struct InvalidBranchNumber;

#[cfg(feature = "rand")]
impl Branch {
    /// Creates the [`Branch`] object representing the first
    /// company Cnpj number registered in the Brazilian Governmental Organizations responsible
    /// for the Cnpj registration.
    ///
    /// The first company branch is represented by the number `0001`.
    pub fn first() -> Branch {
        Branch([0, 0, 0, 1])
    }

    /// Creates a branch from provided `branch_digits`.
    ///
    /// Every number in the `branch_digits` array must be in the range of `0..=9`,
    /// otherwise this method will fail.
    pub fn new(branch_digits: [u8; 4]) -> Result<Branch, InvalidBranchNumber> {
        for b in &branch_digits {
            if *b > 9 {
                return Err(InvalidBranchNumber)
            }
        }

        Ok(Branch(branch_digits))
    }

    /// Creates a Branch object from provided `branch_number`.
    ///
    /// The `branch_number` must be in the range of `0..=9999`, otherwise this method will fail.
    pub fn from_u8(branch_number: u16) -> Result<Branch, InvalidBranchNumber> {
        if branch_number > 9999 {
            return Err(InvalidBranchNumber)
        }

        let first_digit = (branch_number / 1000) as u8;
        let second_digit = ((branch_number % 1000) / 100) as u8;
        let third_digit = (((branch_number % 1000) % 100) / 10) as u8;
        let fourth_digit = (((branch_number % 1000) % 100) % 10) as u8;

        Branch::new([first_digit, second_digit, third_digit, fourth_digit])
    }
}

/// ## Random CNPJ with specific branch example
///
/// ```
/// use validbr::Cnpj;
/// use validbr::cnpj::*;
/// use rand::Rng;
/// let mut rng = rand::thread_rng();
/// let branch = Branch::from_u8(0004).unwrap();
/// let cnpj: Cnpj = rng.sample(branch);
/// assert_eq!(cnpj.branch_digits, [0, 0, 0, 4])
/// ```
#[cfg(feature = "rand")]
impl Distribution<Cnpj> for Branch {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Cnpj {
        let uniform_int = Uniform::from(0u8..=9u8);
        let digits: Vec<u8> = rng.sample_iter(uniform_int)
            .take(8)
            .collect();

        let digits_array: [u8; 8] = digits.try_into()
            .expect("Conversion of Vec with 8 elements MUST be possible at this point.");

        let (first, second) = calculate_verifier_digits(digits_array, self.0);

        Cnpj::new(digits_array, self.0, [first, second])
            .expect("Generated Cnpj MUST be valid at this point")
    }
}
