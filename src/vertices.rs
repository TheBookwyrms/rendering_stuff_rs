pub mod Vertices {


    #[derive(Copy, Clone, Debug)]
    #[repr(C, packed)]
    pub struct Mat4x4 {
        pub p00:f32, pub p01:f32, pub p02:f32, pub p03:f32,
        pub p10:f32, pub p11:f32, pub p12:f32, pub p13:f32,
        pub p20:f32, pub p21:f32, pub p22:f32, pub p23:f32,
        pub p30:f32, pub p31:f32, pub p32:f32, pub p33:f32,
    }


    #[derive(Copy, Clone, Debug)]
    #[repr(C, packed)]
    pub struct V3 {
        pub v0:f32, pub v1:f32, pub v2:f32
    }


    #[derive(Copy, Clone, Debug)]
    #[repr(C, packed)]
    pub struct V4 {
        pub v0:f32, pub v1:f32, pub v2:f32, pub v3:f32
    }


    #[derive(Copy, Clone, Debug)]
    //#[repr(C, packed)]
    pub struct V7 {
        pub v0:f32,
        pub v1:f32,
        pub v2:f32,
        pub v3:f32,
        pub v4:f32,
        pub v5:f32,
        pub v6:f32,
    }
    impl V7 {
        pub fn from(v:[f32;7]) -> V7 { V7 { v0:v[0], v1:v[1], v2:v[2], v3:v[3], v4:v[4], v5:v[5], v6:v[6] } }
    }
    impl Vertex for V7 {
        fn num_items(&self) -> usize { 7 }
    }


    #[derive(Copy, Clone, Debug)]
    #[repr(C, packed)]
    pub struct V10 {
        pub v0:f32,
        pub v1:f32,
        pub v2:f32,
        pub v3:f32,
        pub v4:f32,
        pub v5:f32,
        pub v6:f32,
        pub v7:f32,
        pub v8:f32,
        pub v9:f32
    }
    impl V10 {
        pub fn from(v:[f32;10]) -> V10 { V10 { v0:v[0], v1:v[1], v2:v[2], v3:v[3], v4:v[4],
                                               v5:v[5], v6:v[6], v7:v[7], v8:v[8], v9:v[9] } }
    }
    impl Vertex for V10 {
        fn num_items(&self) -> usize { 10 }
    }


    pub trait Vertex {
        fn num_items(&self) -> usize;
        //fn as_ptr(&self) -> *const f32;
    }
}