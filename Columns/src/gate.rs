pub(crate) mod gate {

    use structs::structs::*;

    pub struct Model {
        pub point: vec2d::Model,
        pub width: f32,
        pub height: f32,
        pub id: f32,
    }

    impl Model {
        pub fn get_width(&self) -> f32 {
            self.width
        }

        pub fn get_height(&self) -> f32 {
            self.height
        }

        pub fn get_id(&self) -> f32 {
            self.id
        }

        pub fn set_width(&mut self, width: f32) {
            self.width = width;
        }

        pub fn set_height(&mut self, height: f32) {
            self.height = height;
        }

        pub fn set_id(&mut self, id: f32) {
            self.id = id;
        }

        pub fn get_gate(&self) -> &Model {
            self.clone()
        }
    }

    pub(crate) mod size {

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
