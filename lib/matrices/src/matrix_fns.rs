use crate::matrix::Matrix2d;


pub fn scale(s:f32) -> Matrix2d {
    Matrix2d::from_array([
        [ s, 0., 0., 0.],
        [0.,  s, 0., 0.],
        [0., 0., s,  0.],
        [0., 0., 0., 1.],
    ])
}
pub fn translate(t:(f32, f32, f32)) -> Matrix2d {
    Matrix2d::from_array([
        [1., 0., 0., t.0],
        [0., 1., 0., t.1],
        [0., 0., 1., t.2],
        [0., 0., 0., 1.0],
    ])
}
pub fn rotate(rx:f32, ry:f32, rz:f32) -> Matrix2d {
    let (rrx, rry, rrz) = (rx.to_radians(), ry.to_radians(), rz.to_radians());
    
    let rot_x = Matrix2d::from_array([
        [1.,           0.,         0., 0.],
        [0.,    rrx.cos(), rrx.sin(), 0.],
        [0., -1.*rrx.sin(), rrx.cos(), 0.],
        [0.,           0.,         0., 1.]
        ]);

    let rot_y = Matrix2d::from_array([
        [rry.cos(), 0., -1.*rry.sin(), 0.],
        [       0., 1.,            0., 0.],
        [rry.sin(), 0.,     rry.cos(), 0.],
        [       0., 0.,            0., 1.]
        ]);

    let rot_z = Matrix2d::from_array([
        [rrz.cos(), -1.*rrz.sin(), 0., 0.],
        [rrz.sin(),     rrz.cos(), 0., 0.],
        [       0.,            0., 1., 0.],
        [       0.,            0., 0., 1.],
        ]);

    
    rot_x.matmul(&rot_y.matmul(&rot_z))
}
pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> Matrix2d {

    // p in form (x_offset, y_offset, z_offset)
    // NOTE : for some reason, y and z switch in calculations
    // thus, p gets deconstructed as :
    let (px, pz, py) = p;
    let (rx, ry, rz) = r;

    let return_to_pos     = translate((px, py, pz));
    let translate_to_zero = translate((-px, -py, -pz));

    let rotate = rotate(rx, ry, rz);

    return_to_pos.matmul(&rotate.matmul(&translate_to_zero))
}

pub fn right_handed() -> Matrix2d {
    Matrix2d::from_array([
        [1.,0.,0.,0.],
        [0.,0.,1.,0.],
        [0.,1.,0.,0.],
        [0.,0.,0.,1.],
    ])
}