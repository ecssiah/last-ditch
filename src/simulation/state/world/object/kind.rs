#[derive(Clone, Copy, Debug)]
pub enum Kind {
    DoorOpen,
    DoorClosed,
    Stairs,
    Platform,
    Ladder,
}

impl Kind {
    pub fn to_string(object_kind: Self) -> String {
        match object_kind {
            Self::DoorOpen => String::from("door_open"),
            Self::DoorClosed => String::from("door_closed"),
            Self::Stairs => String::from("stairs"),
            Self::Platform => String::from("platform"),
            Self::Ladder => String::from("ladder"),
        }
    }
}
