use ultraviolet::Vec2;

#[inline]
pub fn approx_eq(left: Vec2, right: Vec2, epsilon: f32) -> bool {
    (left.x - right.x).abs() <= epsilon && (left.y - right.y).abs() <= epsilon
}
