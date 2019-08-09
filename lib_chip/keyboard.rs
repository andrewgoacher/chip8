use sdl2::keyboard::Keycode;

/// Maps an SDL Keycode to an 8bit chip8 key.
/// 
/// Returns an option, if no key is pressed, None is returned.
/// 
/// Examples:
/// 
/// ```
/// # use lib_chip::keyboard::*;
/// let mapped_key = get_key_mapped(None);
/// assert_eq!(None, mapped_key);
/// ```
/// 
/// ```
/// # use lib_chip::keyboard::*;
/// # use sdl2::keyboard::Keycode;
/// let mapped_key = get_key_mapped(Some(Keycode::A));
/// assert_eq!(Some(0x7), mapped_key);
/// ```
/// 
/// If the keyboard is pressed and an unrecognised key is used, 
/// it will also map to none.
/// 
/// ```
/// # use lib_chip::keyboard::*;
/// # use sdl2::keyboard::Keycode;
/// let mapped_key = get_key_mapped(Some(Keycode::Num9));
/// assert_eq!(None, mapped_key);
/// ```
pub fn get_key_mapped(keycode: Option<Keycode>) -> Option<u8> {
    match keycode {
        None => None,
        Some(code) => {
            match code {
                Keycode::Num1 => Some(0x01),
                Keycode::Num2 => Some(0x02),
                Keycode::Num3 => Some(0x03),
                Keycode::Num4 => Some(0x0c),
                Keycode::Q => Some(0x04),
                Keycode::W => Some(0x05),
                Keycode::E => Some(0x06),
                Keycode::R => Some(0x0d),
                Keycode::A => Some(0x07),
                Keycode::S => Some(0x08),
                Keycode::D => Some(0x09),
                Keycode::F => Some(0x0e),
                Keycode::Z => Some(0x0a),
                Keycode::X => Some(0x00),
                Keycode::C => Some(0x0b),
                Keycode::V => Some(0x0f),
                _ => None
            }
        }
    }
}


/// Returns the name of the chip8 key as a string
/// 
/// If no key is provided it will return None
/// 
/// Examples:
/// 
/// ```
/// # use lib_chip::keyboard::*;
/// let key = get_unmapped_key(None);
/// assert_eq!(None, key);
/// ```
/// 
/// ```
/// # use lib_chip::keyboard::*;
/// let key = get_unmapped_key(Some(0x3));
/// assert_eq!(Some(String::from("3")), key);
/// ```
/// 
/// If the key provided cannot be found, it will also return None.
/// 
/// ```
/// # use lib_chip::keyboard::*;
/// let key = get_unmapped_key(Some(0xFF));
/// assert_eq!(None, key);
/// ```
pub fn get_unmapped_key(key: Option<u8>) -> Option<String> {
    match key {
        None => None,
        Some(k) => {
            match k {
                0x00 => Some(String::from("X")),
                0x01 => Some(String::from("1")),
                0x02 => Some(String::from("2")),
                0x03 => Some(String::from("3")),
                0x04 => Some(String::from("Q")),
                0x05 => Some(String::from("W")),
                0x06 => Some(String::from("E")),
                0x07 => Some(String::from("A")),
                0x08 => Some(String::from("S")),
                0x09 => Some(String::from("D")),
                0x0a => Some(String::from("Z")),
                0x0b => Some(String::from("C")),
                0x0c => Some(String::from("4")),
                0x0d => Some(String::from("R")),
                0x0e => Some(String::from("F")),
                0x0f => Some(String::from("V")),
                _ => None
            }
        }
    }
}