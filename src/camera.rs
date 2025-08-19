pub mod Camera {

    use std::time::{Duration, Instant};
    //use ndarray;
    use crate::ndarray_abstractions::MyArray::{Arr1D, Arr2D, Arr3D, Arr4D};
    

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
        pub light_y_transform:Arr2D, // mat4
    }
    impl Lighting {
        pub fn new() -> Lighting {
            Lighting {
                ambient_strength: 0.1,
                ambient_colour: (24.0/255.0, 128.0/255.0, 0.0),
                diffuse_strength: 0.5,
                diffuse_base: 0.2,
                light_source_pos: (0.0, -10.0,0.0), // in world space // what does this mean???
                light_source_colour: (209.0/255.0, 6.0/255.0, 141.0/255.0),
                specular_strength: 0.8,
                view_vec: (0.0, 0.0, 32.0, 1.0),
                //camera_viewpos: (),
                specular_power: 2,
                light_y_transform: Arr2D::from([
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
    }
}