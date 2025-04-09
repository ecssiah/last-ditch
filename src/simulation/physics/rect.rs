use glam::IVec2;

pub struct Rect {
    min: IVec2,
    max: IVec2,
}

impl Rect {
    pub fn width(&self) -> i32 {
        self.max.x - self.min.x
    }

    pub fn height(&self) -> i32 {
        self.max.y - self.min.y
    }

    pub fn adjacent(&self, rect: &Rect) -> bool {
        let vertically_adjacent = (self.max.x == rect.min.x || self.min.x == rect.max.x)
            && self.min.y < rect.max.y
            && self.max.y > rect.min.y;

        let horizontally_adjacent = (self.max.y == rect.min.y || self.min.y == rect.max.y)
            && self.min.x < rect.max.x
            && self.max.x > rect.min.x;

        vertically_adjacent || horizontally_adjacent
    }

    pub fn connect(&self, rect: &Rect) -> Option<Rect> {
        if self.adjacent(rect) {
            let min = IVec2::new(self.min.x.min(rect.min.x), self.min.y.min(rect.min.y));
            let max = IVec2::new(self.max.x.max(rect.max.x), self.max.y.max(rect.max.y));

            Some(Rect { min, max })
        } else {
            None
        }
    }
}
