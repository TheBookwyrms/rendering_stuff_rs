pub mod Camera {

    use std::time::{Duration, Instant};

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