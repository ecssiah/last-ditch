#[derive(Clone)]
pub struct Age {
    pub years: u32,
    pub period: u32,
}

impl Age {
    pub fn new(years: u32) -> Self {
        let period = Self::get_period(years);

        Self { years, period }
    }

    pub fn get_period(years: u32) -> u32 {
        if (0..2).contains(&years) {
            0
        } else if (2..5).contains(&years) {
            1
        } else if (5..12).contains(&years) {
            2
        } else if (12..22).contains(&years) {
            3
        } else if (22..30).contains(&years) {
            4
        } else if (30..42).contains(&years) {
            5
        } else if (42..50).contains(&years) {
            6
        } else if (50..70).contains(&years) {
            7
        } else {
            8
        }
    }
}
