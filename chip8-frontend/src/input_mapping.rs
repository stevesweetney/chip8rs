use macroquad::input::KeyCode;

pub const ACCEPTED_KEYS: [KeyCode; 16] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Q,
    KeyCode::W,
    KeyCode::E,
    KeyCode::R,
    KeyCode::A,
    KeyCode::S,
    KeyCode::D,
    KeyCode::F,
    KeyCode::Z,
    KeyCode::X,
    KeyCode::C,
    KeyCode::V,
];
pub struct KeyValue(pub u8);

impl TryFrom<KeyCode> for KeyValue {
    type Error = String;

    fn try_from(keycode: KeyCode) -> Result<Self, Self::Error> {
        match keycode {
            KeyCode::Key1 => Ok(KeyValue(0x1)), // 1 => 1
            KeyCode::Key2 => Ok(KeyValue(0x2)), // 2 => 2
            KeyCode::Key3 => Ok(KeyValue(0x3)), // 3 => 3
            KeyCode::Key4 => Ok(KeyValue(0xC)), // 4 => C
            KeyCode::Q => Ok(KeyValue(0x4)),    // Q => 4
            KeyCode::W => Ok(KeyValue(0x5)),    // W => 5
            KeyCode::E => Ok(KeyValue(0x6)),    // E => 6
            KeyCode::R => Ok(KeyValue(0xD)),    // R => D
            KeyCode::A => Ok(KeyValue(0x7)),    // A => 7
            KeyCode::S => Ok(KeyValue(0x8)),    // S => 8
            KeyCode::D => Ok(KeyValue(0x9)),    // D => 9
            KeyCode::F => Ok(KeyValue(0xE)),    // F => E
            KeyCode::Z => Ok(KeyValue(0xA)),    // Z => A
            KeyCode::X => Ok(KeyValue(0x0)),    // X => 0
            KeyCode::C => Ok(KeyValue(0xB)),    // C => B
            KeyCode::V => Ok(KeyValue(0xF)),    // V => F
            _ => Err(format!("Unimplemented keycode {:?}", keycode)),
        }
    }
}
