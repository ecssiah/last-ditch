#[derive(Clone, Copy, Debug)]
pub enum Kind {
    Door,
    Stairs,
    Platform,
}

impl Kind {
    pub fn to_string(object_kind: Self) -> String {
        match object_kind {
            Self::Door => String::from("door"),
            Self::Stairs => String::from("stairs"),
            Self::Platform => String::from("platform"),
        }
    }
}
