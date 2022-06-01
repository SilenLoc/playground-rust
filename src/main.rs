use keyboard::type_word;
use std::{thread, time};



pub mod keyboard;

fn main() {
    let delay = time::Duration::from_millis(1000);
    thread::sleep(delay);
    type_word("some Word")
}





