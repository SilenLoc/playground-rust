use rdev::{EventType, Key, simulate, SimulateError};
use rdev::Key::{KeyA, KeyB, KeyC, KeyD, KeyE, KeyF, KeyG, KeyH, KeyI, KeyJ, KeyK, KeyL, KeyM, KeyN, KeyO, KeyP, KeyQ, KeyR, KeyS, KeyT, KeyU, KeyV, KeyW, KeyX, KeyY, KeyZ};

pub fn type_word(str: &str) {
    let mut tmp = [0u8; 4];

    for c in str.chars() {
        if c.is_uppercase() {
            type_string(c.to_ascii_lowercase().encode_utf8(&mut tmp), type_key_uppercase);
        } else {
            type_string(c.encode_utf8(&mut tmp), type_key)
        }
    }
}

fn type_key(key: Key) {
    send(&EventType::KeyPress(key));
    send(&EventType::KeyRelease(key));
}

fn type_key_uppercase(key: Key) {
    send(&EventType::KeyPress(Key::ShiftLeft));
    send(&EventType::KeyPress(key));
    send(&EventType::KeyRelease(key));
    send(&EventType::KeyRelease(Key::ShiftLeft));
}




fn send(event_type: &EventType) {

    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not send {:?}", event_type);
        }
    }
}


fn type_string(str: &str, out_action: fn(Key)){
    match str{
        "a" => {out_action(KeyA)},
        "b" => {out_action(KeyB)},
        "c" => {out_action(KeyC)},
        "d" => {out_action(KeyD)},
        "e" => {out_action(KeyE)},
        "f" => {out_action(KeyF)},
        "g" => {out_action(KeyG)},
        "h" => {out_action(KeyH)},
        "i" => {out_action(KeyI)},
        "j" => {out_action(KeyJ)},
        "k" => {out_action(KeyK)},
        "m" => {out_action(KeyM)},
        "n" => {out_action(KeyN)},
        "l" => {out_action(KeyL)},
        "o" => {out_action(KeyO)},
        "p" => {out_action(KeyP)},
        "q" => {out_action(KeyQ)},
        "r" => {out_action(KeyR)},
        "s" => {out_action(KeyS)},
        "t" => {out_action(KeyT)},
        "u" => {out_action(KeyU)},
        "v" => {out_action(KeyV)},
        "w" => {out_action(KeyW)},
        "x" => {out_action(KeyX)},
        "y" => {out_action(KeyY)},
        "z" => {out_action(KeyZ)},
        " " => {out_action(Key::Space)},
        "." => {out_action(Key::Dot)},
        "," => {out_action(Key::Comma)}
        _ => {}
    }
}






