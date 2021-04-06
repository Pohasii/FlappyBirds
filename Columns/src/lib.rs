mod gate;

pub mod columns {

    use crate::gate::gate;
    use structs::structs::*;
    //use structs::structs::new_vec2d;

    pub struct Model {
        gate: Vec<gate::Model>,
        column_size: size::Model ,//size::Model,
        gate_size: gate::size::Model,
        columns_count: f32,
        speed_by_sec: f32,
        max_column_count: f32,
        last_id: f32,
        window_size: size::Model,
    }

    pub fn test() {
        let test = vec2d::new_vec2d(3 as f32, 4 as f32);
        println!("{:#?}", test);
        // check()
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
