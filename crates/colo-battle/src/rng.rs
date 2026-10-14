//! The game's one random number generator (sysdolphin `HSD_Rand*`).
//!
//! All draws step one u32 seed at `*(0x80478C94)`. The same seed feeds battle logic (`GSrand`) and
//! particle effects (`HSD_Randf`, ~1600 draws per frame in battle), so a battle's draws depend on
//! everything drawn before them. Dolphin traces record the seed before each battle draw for that reason.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Rng {
    pub seed: u32,
}

impl Rng {
    pub fn new(seed: u32) -> Self { Rng { seed } }

    fn step(&mut self) -> u32 {
        self.seed = self.seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3);
        self.seed
    }

    /// `fn_801ADCD8`: next seed's top 16 bits. `GSrand` (0x800E0C54) returns this as a u16.
    pub fn next(&mut self) -> u32 { self.step() >> 16 }

    /// `GSrand` 0x800E0C54.
    pub fn gs_rand(&mut self) -> u16 { self.next() as u16 }

    /// `HSD_Randf` 0x801ADC7C: (seed >> 16) / 65536.0 in single precision, in [0, 1).
    pub fn randf(&mut self) -> f32 { (self.step() >> 16) as f32 / 65536.0 }

    /// `fn_801ADC3C` (HSD_Randi): `(n * (seed >> 16)) >> 16` as a signed divide by 65536
    /// (`srawi` + `addze`: rounds toward zero).
    pub fn randi(&mut self, n: i32) -> i32 {
        let v = n.wrapping_mul((self.step() >> 16) as i32);
        v / 65536
    }

    /// `_fadeEffectGetRandom__FUl` 0x800E0C04: two draws make a 32-bit value (second draw high),
    /// reduced mod `n`.
    pub fn rand32_mod(&mut self, n: u32) -> u32 {
        let lo = self.next();
        let hi = self.next();
        ((hi << 16) | lo) % n
    }

    /// `fn_800E0BA0`: `randf() + randf() - 1.0` (triangular in (-1, 1)); the second draw is added first.
    pub fn randf_tri(&mut self) -> f32 {
        let a = self.randf();
        let b = self.randf();
        (b + a) - 1.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matches_dolphin_trace() {
        // traces/first.log: battle seed 0x12345678; the first draw in fightSeqInit saw it unchanged.
        let mut r = Rng::new(0x12345678);
        let v = r.next();
        assert_eq!(r.seed, 0x12345678u32.wrapping_mul(0x343FD).wrapping_add(0x269EC3));
        assert_eq!(v, r.seed >> 16);
    }

    #[test]
    fn randi_rounds_toward_zero() {
        let mut r = Rng::new(0);
        let s = r.seed.wrapping_mul(0x343FD).wrapping_add(0x269EC3) >> 16;
        assert_eq!(r.randi(-3), (-3 * s as i32) / 65536);
    }
}
