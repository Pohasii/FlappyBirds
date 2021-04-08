pub mod frame {

    pub struct Model {
        pub x:f32,
        pub y:f32,
        pub width:f32,
        pub height:f32,
        pub r#type: f32
    }

    pub type Frames = Vec<Model>;
}