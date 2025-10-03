use std::time::Instant;

use matrices::Matrix2d;
use matrices::MatrixError;

pub struct Lighting {
    pub ambient_strength:f32,
    pub ambient_colour:(f32, f32, f32),
    pub diffuse_strength:f32,
    pub diffuse_base:f32,
    pub light_source_pos:(f32, f32, f32),
    pub light_source_colour:(f32, f32, f32),
    pub specular_strength:f32,
    pub view_vec:(f32, f32, f32, f32), // what the hell is this ???
    //pub camera_viewpos:(f32, f32, f32),
    pub specular_power:u32,
    pub light_y_transform:Matrix2d, // mat4
}
impl Lighting {
    pub fn new() -> Lighting {
        Lighting {
            ambient_strength: 0.9,
            //ambient_strength: 0.25,
            //ambient_strength: 0.1,
            ambient_colour: (24.0/255.0, 128.0/255.0, 0.0),
            diffuse_strength: 0.5,
            diffuse_base: 0.2,
            light_source_pos: (0.0, -10.0,0.0), // in world space // what does this mean???
            light_source_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
            specular_strength: 0.8,
            view_vec: (0.0, 0.0, 32.0, 1.0),
            //camera_viewpos: (),
            specular_power: 2,
            light_y_transform: Matrix2d::from_array([
                [-1.0,  0.0, 0.0, 0.0],
                [ 0.0, -1.0, 0.0, 0.0],
                [ 0.0,  0.0, 1.0, 0.0],
                [ 0.0,  0.0, 0.0, 1.0],
            ])
        }
    }
}

pub struct Camera {
    pub render_distance:u32,
    pub angle_xyz:(f32, f32, f32),
    pub pan_xyz:(f32, f32, f32),
    pub zoom:f32,
    pub pan_sensitivity:f32,
    pub angle_sensitivity:f32,
    pub panning:bool, pub angling:bool, pub paused:bool,
    pub pause_time:Instant, pub current:Instant, pub dt:u64,
    pub background_colour:(f32, f32, f32),
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            render_distance:512,
            angle_xyz:(0.0, 0.0, 0.0),
            pan_xyz:(0.0, 0.0, 0.0),
            zoom:20.0,
            pan_sensitivity:0.001,
            angle_sensitivity:0.01,
            panning:false, angling:false, paused:false,
            pause_time:Instant::now(), current:Instant::now(), dt:0,
            background_colour:(0.5, 0.5, 0.5),
        }
    }
    pub fn get_orthographic_projection(&self, width:u32, height:u32)
                -> Matrix2d {
        let l = -1.0 * (width / height) as f32 * self.zoom;
        let r = (width / height) as f32 * self.zoom;
        let b = -1.0 * self.zoom as f32;
        let t = self.zoom as f32;
        let n = -1.0 * self.render_distance as f32;
        let f = self.render_distance as f32;

        let orthographic_projection = Matrix2d::from_array([
            [2.0/(r-l), 0.0, 0.0, 0.0],
            [0.0, 2.0/(t-b), 0.0, 0.0],
            [0.0, 0.0, 2.0/(f-n), 0.0],
            [-1.0*(r+l)/(r-l), -1.0*(t+b)/(t-b), -1.0*(f+n)/(f-n), 1.0],
        ]);

        orthographic_projection
    }

    pub fn get_camera_transform(&self) -> Result<Matrix2d, MatrixError> {
        let camera_rotation = Matrix2d::rotate_around_p((0.0, 0.0, 0.0), self.angle_xyz)?;
        let camera_pan = Matrix2d::translate(self.pan_xyz);
        camera_pan.matmul(&camera_rotation)
    }
}