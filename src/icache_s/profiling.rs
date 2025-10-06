#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "PROFILING")]
pub struct Profiling {
    enable: Enable,
    clear: Clear,
    hit: Hit,
    miss: Miss,
    lmiss: Lmiss,
    reads: Reads,
    writes: Writes,
}
impl Profiling {
    #[doc = "0x00 - Enable the profiling counters."]
    #[inline(always)]
    pub const fn enable(&self) -> &Enable {
        &self.enable
    }
    #[doc = "0x04 - Clear the profiling counters."]
    #[inline(always)]
    pub const fn clear(&self) -> &Clear {
        &self.clear
    }
    #[doc = "0x08 - The cache hit counter for cache region."]
    #[inline(always)]
    pub const fn hit(&self) -> &Hit {
        &self.hit
    }
    #[doc = "0x0c - The cache miss counter for cache region."]
    #[inline(always)]
    pub const fn miss(&self) -> &Miss {
        &self.miss
    }
    #[doc = "0x10 - The cache line miss counter for cache region."]
    #[inline(always)]
    pub const fn lmiss(&self) -> &Lmiss {
        &self.lmiss
    }
    #[doc = "0x14 - Number of reads for cache region."]
    #[inline(always)]
    pub const fn reads(&self) -> &Reads {
        &self.reads
    }
    #[doc = "0x18 - Number of writes for cache region."]
    #[inline(always)]
    pub const fn writes(&self) -> &Writes {
        &self.writes
    }
}
#[doc = "ENABLE (rw) register accessor: Enable the profiling counters.\n\nYou can [`read`](crate::Reg::read) this register and get [`enable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`enable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@enable`] module"]
#[doc(alias = "ENABLE")]
pub type Enable = crate::Reg<enable::EnableSpec>;
#[doc = "Enable the profiling counters."]
pub mod enable;
#[doc = "CLEAR (w) register accessor: Clear the profiling counters.\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clear::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@clear`] module"]
#[doc(alias = "CLEAR")]
pub type Clear = crate::Reg<clear::ClearSpec>;
#[doc = "Clear the profiling counters."]
pub mod clear;
#[doc = "HIT (r) register accessor: The cache hit counter for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`hit::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hit`] module"]
#[doc(alias = "HIT")]
pub type Hit = crate::Reg<hit::HitSpec>;
#[doc = "The cache hit counter for cache region."]
pub mod hit;
#[doc = "MISS (r) register accessor: The cache miss counter for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`miss::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@miss`] module"]
#[doc(alias = "MISS")]
pub type Miss = crate::Reg<miss::MissSpec>;
#[doc = "The cache miss counter for cache region."]
pub mod miss;
#[doc = "LMISS (r) register accessor: The cache line miss counter for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`lmiss::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lmiss`] module"]
#[doc(alias = "LMISS")]
pub type Lmiss = crate::Reg<lmiss::LmissSpec>;
#[doc = "The cache line miss counter for cache region."]
pub mod lmiss;
#[doc = "READS (r) register accessor: Number of reads for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`reads::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@reads`] module"]
#[doc(alias = "READS")]
pub type Reads = crate::Reg<reads::ReadsSpec>;
#[doc = "Number of reads for cache region."]
pub mod reads;
#[doc = "WRITES (r) register accessor: Number of writes for cache region.\n\nYou can [`read`](crate::Reg::read) this register and get [`writes::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@writes`] module"]
#[doc(alias = "WRITES")]
pub type Writes = crate::Reg<writes::WritesSpec>;
#[doc = "Number of writes for cache region."]
pub mod writes;
