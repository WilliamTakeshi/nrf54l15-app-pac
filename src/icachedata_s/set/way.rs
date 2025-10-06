#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "WAY")]
pub struct Way {
    du: [Du; 4],
}
impl Way {
    #[doc = "0x00..0x20 - Unspecified"]
    #[inline(always)]
    pub const fn du(&self, n: usize) -> &Du {
        &self.du[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x20 - Unspecified"]
    #[inline(always)]
    pub fn du_iter(&self) -> impl Iterator<Item = &Du> {
        self.du.iter()
    }
}
#[doc = "Unspecified"]
pub use self::du::Du;
#[doc = r"Cluster"]
#[doc = "Unspecified"]
pub mod du;
