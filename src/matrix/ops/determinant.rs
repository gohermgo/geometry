use crate::{
    matrix::{
        ops::{cofactor::ArrayCofactor, determinant_2, ConstIndex},
        AsArray,
    },
    Cofactor, Matr2, Matr3, Matr4,
};

#[const_trait]
pub trait Determinant {
    fn determinant(&self) -> f32;
    fn is_invertible(&self) -> bool {
        self.determinant() != 0.
    }
}
impl const Determinant for [f32; 4] {
    #[inline]
    fn determinant(&self) -> f32 {
        (self[0] * self[3]) - (self[1] * self[2])
    }
}
impl Determinant for Matr2 {
    #[inline]
    fn determinant(&self) -> f32 {
        (self.const_index((0, 0)) * self.const_index((1, 1)))
            - (self.const_index((0, 1)) * self.const_index((1, 0)))
    }
}

const fn array_submatrix<const N: usize>(
    in_array: [f32; N * N],
    omitted_row: usize,
    omitted_col: usize,
) -> [f32; { N - 1 } * { N - 1 }] {
    // Make square array
    let mut array = [0.0; { N - 1 } * { N - 1 }];

    // Make index counter, so as not to overindex, and write correctly
    let mut array_index = 0;

    // Iterate over the `Self` matrix
    let mut row = 0;
    'rows: loop {
        if row >= N {
            break 'rows;
        }
        if row == omitted_row {
            row += 1;
            continue 'rows;
        }
        let row_offset = row * N;
        let mut col = 0;
        'cols: loop {
            if col >= N {
                break 'cols;
            }
            if col == omitted_col {
                col += 1;
                continue 'cols;
            }
            let src = row_offset + col;
            array[array_index] = *in_array.const_index(src);
            array_index += 1;
            col += 1;
        }

        row += 1;
    }

    array
}
#[inline]
const fn array_minor<const N: usize>(array: [f32; N * N], row: usize, col: usize) -> f32
where
    [f32; (N - 1) * (N - 1)]: ~const Determinant,
{
    array_submatrix::<N>(array, row, col).determinant()
}
#[inline]
const fn array_cofactor<const N: usize>(array: [f32; N * N], row: usize, col: usize) -> f32
where
    [f32; (N - 1) * (N - 1)]: ~const Determinant,
{
    if (row + col) % 2 != 0 {
        -array_minor(array, row, col)
    } else {
        array_minor(array, row, col)
    }
}
// #[inline]
// const fn array_determinant<const N: usize>(array: [f32; N * N]) -> f32
// where
//     [f32; (N - 1) * (N - 1)]: ~const Determinant,
// {
//     let mut tally = 0.0;
//     let row = 0;
//     let mut col = 0;
//     while col < 3 {
//         let src = 3 * col;
//         tally += array[src] * array_cofactor(array, row, col);
//         col += 1;
//     }
//     tally
// }
// impl const Determinant for [f32; 9] {
//     #[inline]
//     fn determinant(&self) -> f32 {
//         let mut tally = 0.0;
//         let row = 0;
//         let mut col = 0;
//         while col < 3 {
//             let src = 3 * col;
//             tally += self[src] * self.array_cofactor(row, col);
//             col += 1;
//         }
//         tally
//     }
// }
impl Determinant for Matr3 {
    #[inline]
    fn determinant(&self) -> f32 {
        let mut tally = 0.0;
        let row = 0;
        let mut col = 0;
        while col < 3 {
            tally += self[(row, col)] * self.cofactor(row, col);
            col += 1;
        }
        tally
    }
}
// impl const Determinant for [f32; 16] {
//     #[inline]
//     fn determinant(&self) -> f32 {
//         array_determinant::<4>(*self)
//     }
// }
impl Determinant for Matr4 {
    #[inline]
    fn determinant(&self) -> f32 {
        let mut tally = 0.0;
        let row = 0;
        for col in 0..4 {
            tally += self[(row, col)] * self.cofactor(row, col);
        }
        tally
    }
}

#[cfg(test)]
use crate::Matrix;

#[cfg(test)]
mod tests {
    use super::*;
    mod mat2 {}
    mod mat3 {
        use super::{Cofactor, Determinant, Matr3, Matrix};
        #[test]
        fn calc_det() {
            let a = Matr3::new([1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
            assert_eq!(a.cofactor(0, 0), 56.0);
            assert_eq!(a.cofactor(0, 1), 12.0);
            assert_eq!(a.cofactor(0, 2), -46.0);
            assert_eq!(a.determinant(), -196.0);
        }
    }

    mod mat4 {}
}
