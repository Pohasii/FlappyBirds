mod gate;

pub mod columns {

    pub use crate::gate::gate;
    use rand::{thread_rng, Rng};
    pub use structs::structs::*;
    //use structs::structs::new_vec2d;

    #[derive(Debug)]
    pub struct Model {
        pub gates: Vec<gate::Model>,
        pub column_width: gate::size::Model, //size::Model,
        pub gate_height: gate::size::Model,
        pub columns_count: f32,
        pub speed_by_sec: f32,
        pub max_column_count: f32,
        pub last_id: f64,
        pub window_size: size::Model,
    }

    impl Model {
        // step by time
        pub fn step(&mut self, time_step: f32) {
            let mut is_remove:bool = false;
            let mut is_add:bool = false;
            let window_width:f32 = self.window_size.get_width();
            let mut gates_size:usize = 0;

            if !self.gates.is_empty() {
                gates_size = self.gates.len()-1;
            }

            if self.gates.is_empty() {
                self.new_gate();
            } else {
                for (i,g) in self.gates.iter_mut().enumerate() {
                    // move the gate by time (millsek)
                    g.point.set_x(
                        g.point.get_x() -
                            ((self.speed_by_sec / 1000 as f32) * time_step));

                    // remove gate when gone from window
                    if g.point.get_x() + g.get_width() < 0 as f32 {
                        is_remove = true;
                    }

                    // add new gate when previous gate have required distance from border
                    if i == gates_size &&
                        g.point.get_x() + g.get_width() <
                            window_width -
                                (window_width / self.max_column_count){
                        is_add = true;//
                    }
                }
            }

            // remove first gate if it have negative position by x
            if is_remove {
                println!("{:#?}", &self);
                self.remove_first_gate();
            }

            if is_add {
                self.new_gate();
            }
        }

        pub fn get_frame(&self) -> frame::Frames {
            let mut f: frame::Frames = Vec::new();

            self.gates.iter().for_each(|g|{
                // generate first part of column
                f.push(self.get_column_parts(g, true));
                // generate second part of column
                f.push(self.get_column_parts(g, false));
            });

            f
        }

        // get_gates
        // make gates copy
        pub fn get_gates(&self) -> Vec<gate::Model> {
            self.gates.clone()
        }

        // get_column_parts
        // generate parts of column before gate and after
        fn get_column_parts(&self, g: &gate::Model, first_part: bool) -> frame::Model {
            if first_part {
                return frame::Model {
                    x: g.point.get_x(),
                    y: 0.0,
                    width: g.get_width(),
                    height: g.point.get_y(),
                    r#type: 2.0, // 2.0 type of column
                };
            }

            // second part
            frame::Model {
                x: g.point.get_x(),
                y: g.point.get_y() + g.get_height(),
                width: g.get_width(),
                height: self.window_size.get_height(),
                r#type: 2.0, // 2.0 type of column
            }
        }

        // new_gate
        // create and add new gate to vec<gate>
        fn new_gate(&mut self) {
            let mut rng = thread_rng();
            let border_for_gate_position: (f32, f32) = (50 as f32, 50 as f32);
            let gate_size: f32 =
                rng.gen_range(self.gate_height.get_min()..self.gate_height.get_max());

            let g = gate::new_gate(
                self.window_size.get_width(),
                rng.gen_range(
                    border_for_gate_position.0
                        ..self.window_size.get_height() - border_for_gate_position.1 - gate_size,
                ),
                rng.gen_range(self.column_width.get_min()..self.column_width.get_max()),
                gate_size,
                self.get_id(),
            );

            self.gates.push(g);
            self.columns_count += 1 as f32;
        }

        // remove_first_gate
        // remove first gate from Vec
        fn remove_first_gate(&mut self) {
            if !self.gates.is_empty() {
                if self.gates.len() == 1 {
                    self.gates.clear(); //= Vec::new()
                } else {
                    self.gates.remove(0);
                }
                self.columns_count -= 1 as f32;
            }
        }

        fn get_id(&mut self) -> f64 {
            self.last_id += 1 as f64;
            self.last_id
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
