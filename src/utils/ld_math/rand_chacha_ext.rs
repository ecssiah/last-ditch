use rand_chacha::rand_core::RngCore;

#[inline]
pub fn gen_i32(min: i32, max: i32, random_number_generator: &mut impl RngCore) -> i32 {
    let range = (max - min + 1) as u64;
    let value = (random_number_generator.next_u64() % range) as i32;

    min + value
}

#[inline]
pub fn gen_range_i32(min: i32, max: i32, random_number_generator: &mut impl RngCore) -> i32 {
    let range = (max - min + 1) as u64;
    let value = (random_number_generator.next_u64() % range) as i32;

    min + value
}

#[inline]
pub fn gen_f32(random_number_generator: &mut impl RngCore) -> f32 {
    const SCALE: f32 = 1.0 / ((1u64 << 24) as f32);

    ((random_number_generator.next_u32() & 0x00FF_FFFF) as f32) * SCALE
}

#[inline]
pub fn gen_range_f32(min: f32, max: f32, random_number_generator: &mut impl RngCore) -> f32 {
    min + gen_f32(random_number_generator) * (max - min)
}
