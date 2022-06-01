use keyboard::type_word;
use std::{thread, time};
use rdev::{listen, Event};


pub mod keyboard;

fn main() {
    let delay = time::Duration::from_millis(1000);
    thread::sleep(delay);

        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }

    type_word("some Word")


fn callback(event: Event) {

    match event.name {
        Some(string) => println!("User wrote {:?}", string),
        None => (),
    }
}




