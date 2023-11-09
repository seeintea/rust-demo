use std::convert::AsRef;
use std::fmt::Debug;

fn multiply(x: i8, y: i8) -> i8 {
    x * y
}

pub fn subject_1() {
    // let x: i8 = 15;
    // let y: i16 = 1000;
    // error
    // println!("{x} * {y} = {}", multiply(x.into(), y.try_into().unwrap()));

    let x: i8 = 15;
    let y: i16 = 5;
    println!("{x} * {y} = {}", multiply(x.into(), y.try_into().unwrap()));
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut transpose_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for n in 0..matrix.len() {
        for i in 0..matrix[n].len() {
            transpose_matrix[i][n] = matrix[n][i]
        }
    }
    transpose_matrix
}

// fn pretty_print(matrix: &[[i32; 3]; 3]) {
//     for row in matrix {
//         println!("{row:?}");
//     }
// }

fn pretty_print<T, Line, Matrix>(matrix: Matrix)
where
    T: Debug,
    Line: AsRef<[T]>,
    Matrix: AsRef<[Line]>,
{
    for row in matrix.as_ref() {
        println!("{:?}", row.as_ref());
    }
}

pub fn subject_2() {
    let matrix = [
        [101, 102, 103], // <-- 这个注释会让 rustfmt 添加一个新行
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix:");
    pretty_print(&matrix);

    let transposed = transpose(matrix);
    println!("transposed:");
    pretty_print(&transposed);
}
