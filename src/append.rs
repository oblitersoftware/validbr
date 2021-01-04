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
use std::convert::TryInto;

pub trait ArrayAppend<T, const S: usize> {
    fn append(self, element: T) -> [T; S + 1];
    fn append_array<const N: usize>(self, array: [T; N]) -> [T; S + N];
}

impl<T, const S: usize> ArrayAppend<T, S> for [T; S]
where
    T: Clone,
{
    fn append(self, element: T) -> [T; S + 1] {
        let vec: Vec<T> = self
            .to_vec()
            .iter()
            .map(|i| i.clone())
            .chain(std::iter::once(element))
            .collect();

        let n_array: Result<[T; S + 1], _> = vec.try_into();
        if let Ok(n_array) = n_array {
            n_array
        } else {
            panic!();
        }
    }

    fn append_array<const N: usize>(self, array: [T; N]) -> [T; S + N] {
        let vec: Vec<T> = self
            .to_vec()
            .iter()
            .map(|i| i.clone())
            .chain(array.iter().map(|i| i.clone()))
            .collect();

        let n_array: Result<[T; S + N], _> = vec.try_into();
        if let Ok(n_array) = n_array {
            n_array
        } else {
            panic!();
        }
    }
}
