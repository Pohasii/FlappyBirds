
pub mod vec2d {

    #[derive(Debug, Copy, Clone)]
    pub struct Model {
        pub x: f32,
        pub y: f32,
    }

    pub fn new_vec2d(x: f32, y: f32) -> Model {
        Model { x, y }
    }

    impl Model {
        pub fn get_x(&self) -> f32 {
            self.x
        }

        pub fn get_y(&self) -> f32 {
            self.y
        }

        pub fn get_xy(&self) -> (f32, f32) {
            (self.get_x(), self.get_y())
        }

        pub fn set_x(&mut self, x: f32) {
            self.x = x
        }

        pub fn set_y(&mut self, y: f32) {
            self.y = y
        }

        pub fn set_xy(&mut self, xy: (f32, f32)) {
            self.set_x(xy.0);
            self.set_y(xy.1);
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::vec2d::vec2d::*;

    #[test]
    fn check_vec_2d_get_x() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let test_vec_2d_struct = new_vec2d(one, two);

        assert_eq!(test_vec_2d_struct.get_x(), one);
    }

    #[test]
    fn check_vec_2d_get_y() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let test_vec_2d_struct = new_vec2d(one, two);

        assert_eq!(test_vec_2d_struct.get_y(), two);
    }

    #[test]
    fn check_vec_2d_get_xy() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let test_vec_2d_struct = new_vec2d(one, two);

        assert_eq!(test_vec_2d_struct.get_xy(), (one, two));
    }

    #[test]
    fn check_vec_2d_set_x() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let mut test_vec_2d_struct = new_vec2d(one, two);

        test_vec_2d_struct.set_x(two);

        assert_eq!(test_vec_2d_struct.get_x(), two);
    }

    #[test]
    fn check_vec_2d_set_y() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let mut test_vec_2d_struct = new_vec2d(one, two);

        test_vec_2d_struct.set_y(one);

        assert_eq!(test_vec_2d_struct.get_y(), one);
    }

    #[test]
    fn check_vec_2d_set_xy() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let mut test_vec_2d_struct = new_vec2d(one, two);

        test_vec_2d_struct.set_xy((two, one));

        assert_eq!(test_vec_2d_struct.get_xy(), (two, one));
    }

    #[test]
    fn check_vec_2d_mut_xy() {
        let one: f32 = 5.0;
        let two: f32 = 3.0;

        let mut test_vec_2d_struct = new_vec2d(one, two);

        test_vec_2d_struct.x = two;
        test_vec_2d_struct.y = one;

        assert_eq!(test_vec_2d_struct.get_xy(), (two, one));
    }
}
