use std::thread::sleep;
use std::time::{Duration, Instant}; //SystemTime , Instant
use columns::columns;

fn main() {

    let mut col:columns::Model = columns::Model{
        gates: vec![],
        column_width: columns::gate::size::Model{ min: 50.0, max: 200.0 },
        gate_height: columns::gate::size::Model{ min: 100.0, max: 270.0 },
        columns_count: 0.0,
        speed_by_sec: 50.0,
        max_column_count: 5.0,
        last_id: 0.0,
        window_size: columns::size::Model{ width: 1280.0, height: 720.0 }
    };

    stepper(&mut col);
    // test();
}

pub fn stepper(col: &mut columns::Model) {

    let mut start_step_time: Instant; // let mut current_time:SystemTime = SystemTime::now();
    let mut previously_step_time: Instant = Instant::now();
    let mut time_for_calc:f32;

    let step: Duration = Duration::from_millis(1000 / 60); // new(1, 0); //(1000/60);// SystemTime::from(SystemTime(1000/60)); //();

    loop {
        start_step_time = Instant::now();

        // the time for calc physic
        time_for_calc = start_step_time.duration_since(previously_step_time).as_millis() as f32;

        // some process
        // ...
        col.step(time_for_calc);

        let calc_time = Instant::now().duration_since(start_step_time); //.unwrap();
        if calc_time < step {
            let sl = step - calc_time;
            sleep(sl);
        }

        previously_step_time = start_step_time;
    }
}

pub fn check() {
    // let mut test:i32 = 0;
    let mut exit: i32 = 0;
    //let mut border:i32 = 1000;

    let mut current_time: Instant; // let mut current_time:SystemTime = SystemTime::now();
    let step: Duration = Duration::from_millis(1000 / 60); // new(1, 0); //(1000/60);// SystemTime::from(SystemTime(1000/60)); //();

    loop {
        current_time = Instant::now();
        let mut i: i32 = 0;
        while i < 190000000 {
            i += 1;
            if i == 149000000 {
                //println!("{}",&i.to_string());
                exit += 1;
            }
        }

        //border = border + 10000;

        // let next_time:Instant = Instant::now();
        let calc_time = Instant::now().duration_since(current_time); //.unwrap();

        if calc_time < step {
            let sl = step - calc_time;
            //println!("{}", &format!(" How many time to sleep: {:#?}", sl));
            //println!("Before Sleep");
            sleep(sl);
            //println!("after Sleep");
        } else {
            println!("{}", i); // .to_string()
        }

        if !(exit < 200) {
            break;
        }
    }
}
