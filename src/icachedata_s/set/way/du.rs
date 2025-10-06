#[repr(C)]
#[doc = "Unspecified"]
#[doc(alias = "DU")]
pub struct Du {
    data: [Data; 2],
}
impl Du {
    #[doc = "0x00..0x08 - Description collection: Cache data bits for DATA\\[q\\] in DU\\[p\\] (DataUnit) of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub const fn data(&self, n: usize) -> &Data {
        &self.data[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x08 - Description collection: Cache data bits for DATA\\[q\\] in DU\\[p\\] (DataUnit) of SET\\[n\\], WAY\\[o\\]."]
    #[inline(always)]
    pub fn data_iter(&self) -> impl Iterator<Item = &Data> {
        self.data.iter()
    }
}
#[doc = "DATA (rw) register accessor: Description collection: Cache data bits for DATA\\[q\\] in DU\\[p\\] (DataUnit) of SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`data::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`data::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@data`] module"]
#[doc(alias = "DATA")]
pub type Data = crate::Reg<data::DataSpec>;
#[doc = "Description collection: Cache data bits for DATA\\[q\\] in DU\\[p\\] (DataUnit) of SET\\[n\\], WAY\\[o\\]."]
pub mod data;
