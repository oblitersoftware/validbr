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

/// Converts every [`i32`] in iterator `$iter` to a [`u8`] value.
#[macro_export]
macro_rules! convert_to_u8 {
    ($iter:expr) => {
        $iter.map(|i| {
            let u: Option<u8> = i.to_digit(10).and_then(|i| i.try_into().ok());
            u
        })
    };
}

/// Converts each item of `$expr` to [`String`] and then join all them into a single [`String`]
/// without any separator.
#[macro_export]
macro_rules! join_to_string {
    ($exp:expr) => {
        $exp.iter().map(|d| d.to_string()).collect::<String>()
    };
}
