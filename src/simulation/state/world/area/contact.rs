#[derive(Clone, Debug)]
pub enum Contact {
    X {
        x: i32,
        y_range: (i32, i32),
        z_range: (i32, i32),
    },
    Y {
        y: i32,
        z_range: (i32, i32),
        x_range: (i32, i32),
    },
    Z {
        z: i32,
        x_range: (i32, i32),
        y_range: (i32, i32),
    },
}
