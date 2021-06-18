use raytracer::math::matrix::{self, Matrix};

#[test]
fn construct_4x4_matrix() {
    let vec = vec![
        1.0, 2.0, 3.0, 4.0, //
        5.5, 6.5, 7.5, 8.5, //
        9.0, 10.0, 11.0, 12.0, //
        13.5, 14.5, 15.5, 16.5,
    ];
    let matrix = Matrix::new_from_vec(4, 4, vec);

    assert_eq!(matrix.get(0, 3), 4.0);
    assert_eq!(matrix.get(1, 0), 5.5);
    assert_eq!(matrix.get(1, 2), 7.5);
    assert_eq!(matrix.get(2, 2), 11.0);
    assert_eq!(matrix.get(3, 0), 13.5);
    assert_eq!(matrix.get(3, 2), 15.5);
}

#[test]
fn construct_2x2_matrix() {
    let vec = vec![
        -3.0, 5.0, //
        1.0, -2.0,
    ];

    let matrix = Matrix::new_from_vec(2, 2, vec);

    assert_eq!(matrix.get(0, 0), -3.0);
    assert_eq!(matrix.get(1, 0), 1.0);
    assert_eq!(matrix.get(0, 1), 5.0);
    assert_eq!(matrix.get(1, 1), -2.0);
}

#[test]
fn construct_3x3_matrix() {
    let vec = vec![
        1.0, 2.0, 3.0, //
        5.5, 6.5, 7.5, //
        9.0, 10.0, 11.0, //
    ];
    let matrix = Matrix::new_from_vec(3, 3, vec);

    assert_eq!(matrix.get(0, 0), 1.0);
    assert_eq!(matrix.get(1, 1), 6.5);
    assert_eq!(matrix.get(2, 2), 11.0);
}

#[test]
#[should_panic]
fn panic_with_wrong_matrix() {
    let vec = vec![
        1.0, 2.0, 3.0, //
        5.5, 6.5, 7.5, //
        9.0, 10.0, 11.0, //
    ];
    Matrix::new_from_vec(9, 9, vec);
}

#[test]
#[should_panic]
fn panic_when_getting_out_of_bounds() {
    let vec = vec![
        1.0, 2.0, 3.0, //
        5.5, 6.5, 7.5, //
        9.0, 10.0, 11.0, //
    ];
    Matrix::new_from_vec(3, 3, vec).get(4, 4);
}

#[test]
fn gets_correctly() {
    let vec = vec![
        1.0, 2.0, 3.0, //
        5.5, 6.5, 7.5, //
        9.0, 10.0, 11.0, //
    ];
    let out = Matrix::new_from_vec(3, 3, vec).get(1, 1);
    assert_eq!(out, 6.5)
}

#[test]
fn multiply_4x4_matrices() {
    let vec1 = vec![
        1.0, 2.0, 3.0, 4.0, //
        2.0, 3.0, 4.0, 5.0, //
        3.0, 4.0, 5.0, 6.0, //
        4.0, 5.0, 6.0, 7.0, //
    ];
    let vec2 = vec![
        0.0, 1.0, 2.0, 4.0, //
        1.0, 2.0, 4.0, 8.0, //
        2.0, 4.0, 8.0, 16.0, //
        4.0, 8.0, 16.0, 32.0, //
    ];
    let vec3 = vec![
        24.0, 49.0, 98.0, 196.0, //
        31.0, 64.0, 128.0, 256.0, //
        38.0, 79.0, 158.0, 316.0, //
        45.0, 94.0, 188.0, 376.0,
    ];

    let matrix1 = Matrix::new_from_vec(4, 4, vec1);
    let matrix2 = Matrix::new_from_vec(4, 4, vec2);
    let expected = Matrix::new_from_vec(4, 4, vec3);

    assert_eq!(matrix1 * matrix2, expected)
}

#[test]
fn multiply_4x4_by_4x1() {
    let vec1 = vec![
        1.0, 2.0, 3.0, 4.0, //
        2.0, 4.0, 4.0, 2.0, //
        3.0, 4.0, 5.0, 6.0, //
        4.0, 5.0, 6.0, 7.0, //
    ];
    let vec2 = vec![1.0, 2.0, 3.0, 1.0];

    let vec3 = vec![18.0, 24.0, 32.0, 39.0];

    let matrix1 = Matrix::new_from_vec(4, 4, vec1);
    let matrix2 = Matrix::new_from_vec(4, 1, vec2);

    let expected = Matrix::new_from_vec(4, 1, vec3);
    assert_eq!(matrix1 * matrix2, expected)
}

#[test]
fn transpose_matrix() {
    let vec = vec![
        24.0, 49.0, 98.0, 196.0, //
        31.0, 64.0, 128.0, 256.0, //
        38.0, 79.0, 158.0, 316.0, //
        45.0, 94.0, 188.0, 376.0,
    ];

    let transposed = vec![
        24.0, 31.0, 38.0, 45.0, //
        49.0, 64.0, 79.0, 94.0, //
        98.0, 128.0, 158.0, 188.0, //
        196.0, 256.0, 316.0, 376.0,
    ];
    let matrix = Matrix::new_from_vec(4, 4, vec);
    let trans_matrix = Matrix::new_from_vec(4, 4, transposed);
    assert_eq!(matrix.transpose(), trans_matrix);
}

#[test]
fn transpose_identity_matrix() {
    let identity = matrix::identity::four();
    let inv = matrix::identity::four().transpose();

    assert_eq!(identity, inv)
}

#[test]
fn determinant_of_3x3_matrix() {
    let vec3 = vec![
        1.0, 2.0, 6.0, //
        -5.0, 8.0, -4.0, //
        2.0, 6.0, 4.0,
    ];
    let matrix3 = Matrix::new_from_vec(3, 3, vec3);

    assert_eq!(matrix3.determinant(), -196.0);
}

#[test]
fn determinant_of_4x4_matrix() {
    let vec4 = vec![
        -2.0, -8.0, 3.0, 5.0, //
        -3.0, 1.0, 7.0, 3.0, //
        1.0, 2.0, -9.0, 6.0, //
        -6.0, 7.0, 7.0, -9.0,
    ];
    let matrix4 = Matrix::new_from_vec(4, 4, vec4);

    assert_eq!(matrix4.determinant(), -4071.0);
}

#[test]
fn inverse_matrix() {
    let vec4 = vec![
        -5.0, 2.0, 6.0, -8.0, //
        1.0, -5.0, 1.0, 8.0, //
        7.0, 7.0, -6.0, -7.0, //
        1.0, -3.0, 7.0, 4.0,
    ];

    // aCcUraccy
    let invvec = vec![
        0.21804511278195488,
        0.45112781954887216,
        0.24060150375939848,
        -0.045112781954887216, //
        -0.8082706766917294,
        -1.4567669172932332,
        -0.44360902255639095,
        0.5206766917293233, //
        -0.07894736842105263,
        -0.2236842105263158,
        -0.05263157894736842,
        0.19736842105263158, //
        -0.5225563909774437,
        -0.8139097744360902,
        -0.3007518796992481,
        0.30639097744360905,
    ];

    let matrix4 = Matrix::new_from_vec(4, 4, vec4);
    let inverse = Matrix::new_from_vec(4, 4, invvec);

    assert_eq!(matrix4.determinant(), 532.0);
    assert_eq!(matrix4.inverse(), Some(inverse));
}
