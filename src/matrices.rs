pub mod Matrices {
    use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};


    pub fn scale(s:f32) -> Arr2D {
        Arr2D::from([
            [ s, 0., 0., 0.],
            [0.,  s, 0., 0.],
            [0., 0., s,  0.],
            [0., 0., 0., 1.],
        ])
    }
    pub fn translate(t:(f32, f32, f32)) -> Arr2D {
        Arr2D::from([
            [1., 0., 0., t.0],
            [0., 1., 0., t.1],
            [0., 0., 1., t.2],
            [0., 0., 0., 1.0],
        ])
    }
    pub fn rotate(rx:f32, ry:f32, rz:f32) -> Arr2D {
        let (rrx, rry, rrz) = (rx.to_radians(), ry.to_radians(), rz.to_radians());
        
        let rot_x = Arr2D::from([
            [1.,           0.,         0., 0.],
            [0.,    rrx.cos(), rrx.sin(), 0.],
            [0., -1.*rrx.sin(), rrx.cos(), 0.],
            [0.,           0.,         0., 1.]
            ]);

        let rot_y = Arr2D::from([
            [rry.cos(), 0., -1.*rry.sin(), 0.],
            [       0., 1.,            0., 0.],
            [rry.sin(), 0.,     rry.cos(), 0.],
            [       0., 0.,            0., 1.]
            ]);

        let rot_z = Arr2D::from([
            [rrz.cos(), -1.*rrz.sin(), 0., 0.],
            [rrz.sin(),     rrz.cos(), 0., 0.],
            [       0.,            0., 1., 0.],
            [       0.,            0., 0., 1.],
            ]);

        
        rot_x.dot2d(rot_y.dot2d(rot_z))
    }
    pub fn rotate_around_p(p:(f32, f32, f32), r:(f32, f32, f32)) -> Arr2D {

        // p in form (x_offset, y_offset, z_offset)
        // NOTE : for some reason, y and z switch in calculations
        // thus, p gets deconstructed as :
        let (px, pz, py) = p;
        let (rx, ry, rz) = r;

        let return_to_pos     = translate((px, py, pz));
        let translate_to_zero = translate((-px, -py, -pz));

        let rotate = rotate(rx, ry, rz);

        return_to_pos.dot2d(rotate.dot2d(translate_to_zero))
    }

    pub fn get_orthographic_projection(width:u32, height:u32, zoom:f32, render_distance:u32)
                 -> Arr2D {
        let l = -1.0 * (width / height) as f32 * zoom;
        let r = (width / height) as f32 * zoom;
        let b = -1.0 * zoom as f32;
        let t = zoom as f32;
        let n = -1.0 * render_distance as f32;
        let f = render_distance as f32;

        let orthographic_projection = Arr2D::from([
            [2.0/(r-l), 0.0, 0.0, 0.0],
            [0.0, 2.0/(t-b), 0.0, 0.0],
            [0.0, 0.0, 2.0/(f-n), 0.0],
            [-1.0*(r+l)/(r-l), -1.0*(t+b)/(t-b), -1.0*(f+n)/(f-n), 1.0],
        ]);

        orthographic_projection
    }

    pub fn get_camera_transform(angle:(f32, f32, f32), pan:(f32, f32, f32)) -> Arr2D {
        let camera_rotation = rotate_around_p((0.0, 0.0, 0.0), angle);
        let camera_pan = translate(pan);
        camera_pan.dot2d(camera_rotation)
    }

    pub fn get_world_transform() -> Arr2D {
        let right_handed = Arr2D::from([
            [1.,0.,0.,0.],
            [0.,0.,1.,0.],
            [0.,1.,0.,0.],
            [0.,0.,0.,1.],
            ]);

        let world_transform = right_handed;
        return world_transform
    }
}