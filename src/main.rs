
use std::time::{Duration, Instant}; //SystemTime , Instant
use std::thread::sleep;
use columns::Columns::test;
//use structs::structs::new_vec2d;

fn main () {

    // let test =  new_vec2d(3 as f32, 4 as f32);
    // println!("{:#?}", test);
    check();
    //test()
}

pub fn check() {
    // let mut test:i32 = 0;
    let mut exit:i32 = 0;
    //let mut border:i32 = 1000;

    let mut current_time:Instant; // let mut current_time:SystemTime = SystemTime::now();
    let step: Duration = Duration::from_millis(1000/60); // new(1, 0); //(1000/60);// SystemTime::from(SystemTime(1000/60)); //();

    loop {
        current_time = Instant::now();
        let mut i:i32 = 0;
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
            println!("{}",i); // .to_string()
        }

        if !(exit < 200) {
            break;
        }

    }
}