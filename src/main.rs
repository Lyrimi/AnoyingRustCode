use enigo::{
    Enigo, Keyboard, Settings,
};
use std::{thread, time};
use rand::{self, Rng};

fn main() {
    let mut rng = rand::thread_rng();
    loop {
        thread::sleep(time::Duration::from_millis(1000));
        if rng.gen_range(1..=10) == 1 {
            let mut enigo = Enigo::new(&Settings::default()).unwrap();
            let resault = enigo.text("hello world!");
            println!("Wrote stuff{:?}", resault);
        } 
    }
}
