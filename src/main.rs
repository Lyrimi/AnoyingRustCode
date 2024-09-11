use enigo::{
    Enigo, Keyboard, Settings,
};



fn main() {
    let mut enigo = Enigo::new(&Settings::default()).unwrap();
    let resault = enigo.text("hello world!");
    println!("{:?}", resault);
}
