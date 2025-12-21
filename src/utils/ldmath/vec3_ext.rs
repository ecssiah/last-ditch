use ultraviolet::Vec3;

#[inline]
pub fn approx_eq(vec3_left: Vec3, vec3_right: Vec3, epsilon: f32) -> bool {
    (vec3_left.x - vec3_right.x).abs() <= epsilon
        && (vec3_left.y - vec3_right.y).abs() <= epsilon
        && (vec3_left.z - vec3_right.z).abs() <= epsilon
}
