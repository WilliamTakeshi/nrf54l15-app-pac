#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "WAY")]
pub struct Way {
    info: Info,
}
impl Way {
    #[doc = "0x00 - Description cluster: Cache information for SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn info(&self) -> &Info {
        &self.info
    }
}
#[doc = "INFO (rw) register accessor: Description cluster: Cache information for SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@info`] module"]
#[doc(alias = "INFO")]
pub type Info = crate::Reg<info::InfoSpec>;
#[doc = "Description cluster: Cache information for SET\\[n\\], WAY\\[o\\]."]
pub mod info;
