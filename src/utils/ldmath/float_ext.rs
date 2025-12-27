#[inline]
pub fn equal(left: f32, right: f32, epsilon: f32) -> bool {
    (left - right).abs() <= epsilon
}

#[inline]
pub fn not_equal(left: f32, right: f32, epsilon: f32) -> bool {
    !equal(left, right, epsilon)
}
