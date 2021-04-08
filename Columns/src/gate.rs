
// pub(crate) mod gate {
pub mod gate {

    use structs::structs::*;

    #[derive(Debug, Copy, Clone)]
    pub struct Model {
        pub point: vec2d::Model,
        pub width: f32,
        pub height: f32,
        pub id: f64,
    }

    pub fn new_gate(x:f32, y:f32, width:f32, height:f32, id:f64) -> Model {
        Model {
            point: vec2d::new_vec2d(x, y),
            width,
            height,
            id,
        }
    }

    impl Model {
        pub fn get_width(&self) -> f32 {
            self.width
        }

        pub fn get_height(&self) -> f32 {
            self.height
        }

        pub fn get_id(&self) -> f64 {
            self.id
        }

        pub fn set_width(&mut self, width: f32) {
            self.width = width;
        }

        pub fn set_height(&mut self, height: f32) {
            self.height = height;
        }

        pub fn set_id(&mut self, id: f64) {
            self.id = id;
        }

        pub fn get_gate(&self) -> Model {
            self.clone()
        }
    }

    pub mod size {
        #[derive(Debug)]
        pub struct Model {
            pub min: f32,
            pub max: f32,
        }

        impl Model {
            pub fn get_min(&self) -> f32 {
                self.min
            }

            pub fn get_max(&self) -> f32 {
                self.max
            }

            pub fn set_min(&mut self, min: f32) {
                self.min = min
            }

            pub fn set_max(&mut self, max: f32) {
                self.max = max
            }
        }
    }
}
