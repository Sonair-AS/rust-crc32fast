#[cfg(all(
    target_feature = "sse2",
    any(target_arch = "x86", target_arch = "x86_64")
))]
mod pclmulqdq;
#[cfg(all(
    target_feature = "sse2",
    any(target_arch = "x86", target_arch = "x86_64")
))]
pub use self::pclmulqdq::State;

#[cfg(all(stable_arm_crc32_intrinsics, target_arch = "aarch64"))]
mod aarch64;
#[cfg(all(stable_arm_crc32_intrinsics, target_arch = "aarch64"))]
pub use self::aarch64::State;

#[cfg(not(any(
    all(
        target_feature = "sse2",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(stable_arm_crc32_intrinsics, target_arch = "aarch64")
)))]
#[derive(Clone)]
pub enum State {}
#[cfg(not(any(
    all(
        target_feature = "sse2",
        any(target_arch = "x86", target_arch = "x86_64")
    ),
    all(stable_arm_crc32_intrinsics, target_arch = "aarch64")
)))]
impl State {
    pub fn new(_: u32) -> Option<Self> {
        None
    }

    pub fn update(&mut self, _buf: &[u8]) {
        match *self {}
    }

    pub fn finalize(self) -> u32 {
        match self {}
    }

    pub fn reset(&mut self) {
        match *self {}
    }

    pub fn combine(&mut self, _other: u32, _amount: u64) {
        match *self {}
    }
}
