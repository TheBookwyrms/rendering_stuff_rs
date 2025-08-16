pub mod matrices {
    use ndarray;

    pub fn translate(tx:f32, ty:f32, tz:f32) -> ndarray::ArrayBase {
        ndarray::array![
            [1, 0, 0, tx],
            [0, 1, 0, ty],
            [0, 0, 1, tz],
            [0, 0, 0, 1],
        ]
    }
    pub fn rotate(rx:f32, ry:f32, rz:f32) -> ndarray::ArrayBase {
        let (rrx, rry, rrz) = (rx.to_radians(), ry.to_radians(), rz.to_radians());
        
        rot_x = ndarray::array![
            [1,            0,           0, 0],
            [0,  &rrx.cos(), np.sin(rrx), 0],
            [0, -1*&rrx.sin(), &rrx.cos(), 0],
            [0,            0,           0, 1]
            ];

        rot_y = ndarray::array![
            [&rry.cos(), 0, -1*rry.sin(), 0],
            [          0, 1,            0, 0],
            [&rry, 0,  &rry.cos(), 0],
            [          0, 0,            0, 1]
            ];

        rot_z = ndarray::array![
            [&rrz.cos(), -1*&rrz.sin(), 0, 0],
            [&rrz.sin(),  &rrz.cos(), 0, 0],
            [          0,            0, 1, 0],
            [          0,            0, 0, 1],
            ];

        
        &rot_x.dot(&rot_y.dot(&rot_z))
    }
    pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> ndarray::ArrayBase {

        // p in form (x_offset, y_offset, z_offset)
        // NOTE : for some reason, y and z switch in calculations
        // thus, p gets deconstructed as :
        let (px, pz, py) = p;
        let (rx, ry, rz) = r;

        let return_to_pos     = translate(px, py, pz);
        let translate_to_zero = translate(-px, -py, -pz);

        let rotate = rotate(r);

        &return_to_pos.dot(&rotate.dot(&translate_to_zero))
    }
}