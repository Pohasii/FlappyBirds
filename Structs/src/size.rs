pub mod size {

    pub struct Model {
        pub width: f32,
        pub height: f32,
    }

    impl Model {
        pub fn get_width(&self) -> f32 {
            self.width
        }

        pub fn get_height(&self) -> f32 {
            self.height
        }

        pub fn set_width(&mut self, width: f32) {
            self.width = width;
        }

        pub fn set_height(&mut self, height: f32) {
            self.height = height;
        }

        pub fn get_size(&self) -> & Model {
            self.clone()
        }
    }
}