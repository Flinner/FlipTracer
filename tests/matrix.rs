use raytracer::math::matrix::Matrix;

#[test]
fn construct_4x4_matrix() {
    let vec = [
        1.0, 2.0, 3.0, 4.0, //
        5.5, 6.5, 7.5, 8.5, //
        9.0, 10.0, 11.0, 12.0, //
        13.5, 14.5, 15.5, 16.5,
    ];
    let matrix = Matrix::new_from_vec(vec);

    assert_eq!(matrix.get(0, 3), 4.0);
    assert_eq!(matrix.get(1, 0), 5.5);
    assert_eq!(matrix.get(1, 2), 7.5);
    assert_eq!(matrix.get(2, 2), 11.0);
    assert_eq!(matrix.get(3, 0), 13.5);
    assert_eq!(matrix.get(3, 2), 15.5);
}

#[test]
fn gets_correctly() {
    let vec = [
        1.0, 2.0, 3.0, 0.0, //
        5.5, 6.5, 7.5, 0.0, //
        9.0, 10.0, 11.0, 0.0, //
        9.0, 10.0, 11.0, 0.0, //
    ];
    let out = Matrix::new_from_vec(vec).get(1, 1);
    assert_eq!(out, 6.5)
}

#[test]
fn multiply_4x4_matrices() {
    let vec1 = [
        1.0, 2.0, 3.0, 4.0, //
        2.0, 3.0, 4.0, 5.0, //
        3.0, 4.0, 5.0, 6.0, //
        4.0, 5.0, 6.0, 7.0, //
    ];
    let vec2 = [
        0.0, 1.0, 2.0, 4.0, //
        1.0, 2.0, 4.0, 8.0, //
        2.0, 4.0, 8.0, 16.0, //
        4.0, 8.0, 16.0, 32.0, //
    ];
    let vec3 = [
        24.0, 49.0, 98.0, 196.0, //
        31.0, 64.0, 128.0, 256.0, //
        38.0, 79.0, 158.0, 316.0, //
        45.0, 94.0, 188.0, 376.0,
    ];

    let matrix1 = Matrix::new_from_vec(vec1);
    let matrix2 = Matrix::new_from_vec(vec2);
    let expected = Matrix::new_from_vec(vec3);

    assert_eq!(matrix1 * matrix2, expected)
}

#[test]
fn transpose_matrix() {
    let vec = [
        24.0, 49.0, 98.0, 196.0, //
        31.0, 64.0, 128.0, 256.0, //
        38.0, 79.0, 158.0, 316.0, //
        45.0, 94.0, 188.0, 376.0,
    ];

    let transposed = [
        24.0, 31.0, 38.0, 45.0, //
        49.0, 64.0, 79.0, 94.0, //
        98.0, 128.0, 158.0, 188.0, //
        196.0, 256.0, 316.0, 376.0,
    ];
    let matrix = Matrix::new_from_vec(vec);
    let trans_matrix = Matrix::new_from_vec(transposed);
    assert_eq!(matrix.transpose(), trans_matrix);
}

#[test]
fn transpose_identity_matrix() {
    let identity = Matrix::identity();
    let inv = Matrix::identity();

    assert_eq!(identity, inv)
}

#[test]
fn determinant_of_4x4_matrix() {
    let vec4 = [
        -2.0, -8.0, 3.0, 5.0, //
        -3.0, 1.0, 7.0, 3.0, //
        1.0, 2.0, -9.0, 6.0, //
        -6.0, 7.0, 7.0, -9.0,
    ];
    let matrix4 = Matrix::new_from_vec(vec4);

    assert_eq!(matrix4.determinant(), -4071.0);
}

#[test]
fn inverse_matrix() {
    let vec4 = [
        -5.0, 2.0, 6.0, -8.0, //
        1.0, -5.0, 1.0, 8.0, //
        7.0, 7.0, -6.0, -7.0, //
        1.0, -3.0, 7.0, 4.0,
    ];

    // aCcUraccy
    let invvec = [
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

    let matrix4 = Matrix::new_from_vec(vec4);
    let inverse = Matrix::new_from_vec(invvec);

    assert_eq!(matrix4.determinant(), 532.0);
    assert_eq!(matrix4.inverse(), Some(inverse));
}

#[test]
fn multiplying_product_with_inverse() {
    let a = [
        3.0, -9.0, 7.0, 3.0, //
        3.0, -8.0, 2.0, -9.0, //
        -4.0, 4.0, 4.0, 1.0, //
        -6.0, 5.0, -1.0, 1.0,
    ];
    let b = [
        8.0, 2.0, 2.0, 2.0, //
        3.0, -1.0, 7.0, 0.0, //
        7.0, 0.0, 5.0, 4.0, //
        6.0, -2.0, 0.0, 5.0,
    ];

    let ma = Matrix::new_from_vec(a);
    let mb = Matrix::new_from_vec(b);

    let mc = ma.clone() * mb.clone();

    let new_ma = mc * mb.inverse().unwrap();

    assert!(new_ma
        .data
        .iter()
        .zip(ma.data.iter())
        .all(|(c, a)| (c - a).abs() < 100.0 * f64::EPSILON));
}
