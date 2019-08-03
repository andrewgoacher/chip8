use sdl2::keyboard::Keycode;

pub fn get_key_mapped(keycode: Option<Keycode>) -> Option<u8> {
    match keycode {
        None => None,
        Some(code) => {
            match code {
                Keycode::Num1 => Some(0x1),
                Keycode::Num2 => Some(0x2),
                Keycode::Num3 => Some(0x3),
                Keycode::Num4 => Some(0xc),
                Keycode::Q => Some(0x4),
                Keycode::W => Some(0x5),
                Keycode::E => Some(0x6),
                Keycode::R => Some(0xd),
                Keycode::A => Some(0x7),
                Keycode::S => Some(0x8),
                Keycode::D => Some(0x9),
                Keycode::F => Some(0xe),
                Keycode::Z => Some(0xa),
                Keycode::X => Some(0x0),
                Keycode::C => Some(0xb),
                Keycode::V => Some(0xf),
                _ => None
            }
        }
    }
}

pub fn get_unmapped_key(key: Option<u8>) -> Option<String> {
    match key {
        None => None,
        Some(k) => {
            match k {
                0x0 => Some(String::from("X")),
                0x1 => Some(String::from("1")),
                0x2 => Some(String::from("2")),
                0x3 => Some(String::from("3")),
                0x4 => Some(String::from("Q")),
                0x5 => Some(String::from("W")),
                0x6 => Some(String::from("E")),
                0x7 => Some(String::from("A")),
                0x8 => Some(String::from("S")),
                0x9 => Some(String::from("D")),
                0xa => Some(String::from("Z")),
                0xb => Some(String::from("C")),
                0xc => Some(String::from("4")),
                0xd => Some(String::from("R")),
                0xe => Some(String::from("F")),
                0xf => Some(String::from("V")),
                _ => None
            }
        }
    }
}