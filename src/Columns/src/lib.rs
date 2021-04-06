


pub mod Columns {

    extern crate structs;
    use structs::structs::new_vec2d;

    pub fn test () {

        let test = new_vec2d(3 as f32, 4 as f32);
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
