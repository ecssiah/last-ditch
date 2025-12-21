use ultraviolet::Vec3;

#[inline]
pub fn approx_eq(left: Vec3, right: Vec3, epsilon: f32) -> bool {
    (left.x - right.x).abs() <= epsilon
        && (left.y - right.y).abs() <= epsilon
        && (left.z - right.z).abs() <= epsilon
}
