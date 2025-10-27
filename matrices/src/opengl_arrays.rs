use crate::matrix::Matrix;
use crate::enums::MatrixError;

impl Matrix<f32> {
    
    /// 3D scale matrix based on x, y, and z scale factors
    pub fn scale(sx:f32, sy:f32, sz:f32) -> Matrix<f32> {
        Matrix::from_2darray([
            [sx, 0., 0., 0.],
            [0., sy, 0., 0.],
            [0., 0., sz, 0.],
            [0., 0., 0., 1.],
        ])
    }

    /// 3D translation matrix based on x, y, and z translation factors
    pub fn translate(t:(f32, f32, f32)) -> Matrix<f32> {
        Matrix::from_2darray([
            [1., 0., 0., t.0],
            [0., 1., 0., t.1],
            [0., 0., 1., t.2],
            [0., 0., 0., 1.0],
        ])
    }

    /// 3D rotation matrix based on x, y, and z rotation factors
    /// (rx, ry, rz) are in degrees
    /// rotation occurs around the (relative) origin for the points
    pub fn rotate(rx:f32, ry:f32, rz:f32) -> Result<Matrix<f32>, MatrixError<f32>> {
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

    /// creates rotation matrix around an arbitrary point
    /// 
    /// rotation is in degrees
    /// 
    /// first translates the position to be at the origin,
    /// then rotates it accordingly,
    /// lastly translates back to the original position
    pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> Result<Matrix<f32>, MatrixError<f32>> {

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

    /// creates matrix that transforms between right-handed
    /// coordinate system and the opengl coordinate system
    pub fn opengl_to_right_handed() -> Matrix<f32> {
        Matrix::from_2darray([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
        ])
    }
}