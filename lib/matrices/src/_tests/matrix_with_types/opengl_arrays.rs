use crate::_tests::matrix_with_types::matrix::Matrix;
use crate::_tests::matrix_with_types::errors::MatrixError;

impl Matrix<f32> {
    
    pub fn scale(sx:f32, sy:f32, sz:f32) -> Matrix<f32> {
        Matrix::from_2darray([
            [sx, 0., 0., 0.],
            [0., sy, 0., 0.],
            [0., 0., sz, 0.],
            [0., 0., 0., 1.],
        ])
    }

    pub fn translate(t:(f32, f32, f32)) -> Matrix<f32> {
        Matrix::from_2darray([
            [1., 0., 0., t.0],
            [0., 1., 0., t.1],
            [0., 0., 1., t.2],
            [0., 0., 0., 1.0],
        ])
    }

    pub fn rotate(rx:f32, ry:f32, rz:f32) -> Result<Matrix<f32>, MatrixError> {
        let (rrx, rry, rrz) = (rx.to_radians(), ry.to_radians(), rz.to_radians());
        
        let rot_x = Matrix::from_2darray([
            [1.,           0.,         0., 0.],
            [0.,    rrx.cos(), rrx.sin(), 0.],
            [0., -1.*rrx.sin(), rrx.cos(), 0.],
            [0.,           0.,         0., 1.]
            ]);

        let rot_y = Matrix::from_2darray([
            [rry.cos(), 0., -1.*rry.sin(), 0.],
            [       0., 1.,            0., 0.],
            [rry.sin(), 0.,     rry.cos(), 0.],
            [       0., 0.,            0., 1.]
            ]);

        let rot_z = Matrix::from_2darray([
            [rrz.cos(), -1.*rrz.sin(), 0., 0.],
            [rrz.sin(),     rrz.cos(), 0., 0.],
            [       0.,            0., 1., 0.],
            [       0.,            0., 0., 1.],
            ]);

        
        Ok(rot_x.matmul(&rot_y.matmul(&rot_z)?)?)
    }

    pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> Result<Matrix<f32>, MatrixError> {

        // p in form (x_offset, y_offset, z_offset)
        // NOTE : for some reason, y and z switch in calculations
        // thus, p gets deconstructed as :
        let (px, pz, py) = p;
        let (rx, ry, rz) = r;

        let return_to_pos     = Matrix::translate((px, py, pz));
        let translate_to_zero = Matrix::translate((-px, -py, -pz));

        let rotate = Matrix::rotate(rx, ry, rz)?;

        Ok(return_to_pos.matmul(&rotate.matmul(&translate_to_zero)?)?)
    }

    pub fn opengl_to_right_handed() -> Matrix<f32> {
        Matrix::from_2darray([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
        ])
    }
}