use macroquad::input::KeyCode;

pub const ACCEPTED_KEYS: [KeyCode; 16] = [
    KeyCode::Key0,
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
    KeyCode::Key7,
    KeyCode::Key8,
    KeyCode::Key9,
    KeyCode::A,
    KeyCode::B,
    KeyCode::C,
    KeyCode::D,
    KeyCode::E,
    KeyCode::F,
];
pub struct KeyValue(pub u8);

impl TryFrom<KeyCode> for KeyValue {
    type Error = String;

    fn try_from(keycode: KeyCode) -> Result<Self, Self::Error> {
        match keycode {
            KeyCode::Key0 => Ok(KeyValue(0)),
            KeyCode::Key1 => Ok(KeyValue(1)),
            KeyCode::Key2 => Ok(KeyValue(2)),
            KeyCode::Key3 => Ok(KeyValue(3)),
            KeyCode::Key4 => Ok(KeyValue(4)),
            KeyCode::Key5 => Ok(KeyValue(5)),
            KeyCode::Key6 => Ok(KeyValue(6)),
            KeyCode::Key7 => Ok(KeyValue(7)),
            KeyCode::Key8 => Ok(KeyValue(8)),
            KeyCode::Key9 => Ok(KeyValue(9)),
            KeyCode::A => Ok(KeyValue(10)),
            KeyCode::B => Ok(KeyValue(11)),
            KeyCode::C => Ok(KeyValue(12)),
            KeyCode::D => Ok(KeyValue(13)),
            KeyCode::E => Ok(KeyValue(14)),
            KeyCode::F => Ok(KeyValue(15)),
            _ => Err(format!("Unimplemented keycode {:?}", keycode)),
        }
    }
}
