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
pub fn get_key_mapped(keycode: Keycode) -> Option<u8> {
    match keycode {
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