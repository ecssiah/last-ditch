use rand_chacha::rand_core::RngCore;

pub struct RngExt {}

impl RngExt {
    #[inline]
    pub fn gen_i32(min: i32, max: i32, rng: &mut impl RngCore) -> i32 {
        let range = (max - min + 1) as u64;
        let value = (rng.next_u64() % range) as i32;

        min + value
    }

    #[inline]
    pub fn gen_f32(rng: &mut impl RngCore) -> f32 {
        const SCALE: f32 = 1.0 / ((1u64 << 24) as f32);

        ((rng.next_u32() & 0x00FF_FFFF) as f32) * SCALE
    }

    #[inline]
    pub fn gen_range_f32(min: f32, max: f32, rng: &mut impl RngCore) -> f32 {
        min + Self::gen_f32(rng) * (max - min)
    }
}
