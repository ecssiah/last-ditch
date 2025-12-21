#[inline]
pub fn approx_eq(left: f32, right: f32, epsilon: f32) -> bool {
    (left - right).abs() <= epsilon
}
