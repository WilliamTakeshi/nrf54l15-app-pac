#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x0100],
    events_fpuioc: EventsFpuioc,
    events_fpudzc: EventsFpudzc,
    events_fpuofc: EventsFpuofc,
    events_fpuufc: EventsFpuufc,
    events_fpuixc: EventsFpuixc,
    events_fpuidc: EventsFpuidc,
    _reserved6: [u8; 0x01e8],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
    _reserved9: [u8; 0x01f4],
    lock: Lock,
    cpuid: Cpuid,
}
impl RegisterBlock {
    #[doc = "0x100 - An invalid operation exception has occurred in the FPU."]
    #[inline(always)]
    pub const fn events_fpuioc(&self) -> &EventsFpuioc {
        &self.events_fpuioc
    }
    #[doc = "0x104 - A floating-point divide-by-zero exception has occurred in the FPU."]
    #[inline(always)]
    pub const fn events_fpudzc(&self) -> &EventsFpudzc {
        &self.events_fpudzc
    }
    #[doc = "0x108 - A floating-point overflow exception has occurred in the FPU."]
    #[inline(always)]
    pub const fn events_fpuofc(&self) -> &EventsFpuofc {
        &self.events_fpuofc
    }
    #[doc = "0x10c - A floating-point underflow exception has occurred in the FPU."]
    #[inline(always)]
    pub const fn events_fpuufc(&self) -> &EventsFpuufc {
        &self.events_fpuufc
    }
    #[doc = "0x110 - A floating-point inexact exception has occurred in the FPU."]
    #[inline(always)]
    pub const fn events_fpuixc(&self) -> &EventsFpuixc {
        &self.events_fpuixc
    }
    #[doc = "0x114 - A floating-point input denormal exception has occurred in the FPU."]
    #[inline(always)]
    pub const fn events_fpuidc(&self) -> &EventsFpuidc {
        &self.events_fpuidc
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
    #[doc = "0x500 - Register to lock the certain parts of the CPU from being modified."]
    #[inline(always)]
    pub const fn lock(&self) -> &Lock {
        &self.lock
    }
    #[doc = "0x504 - The identifier for the CPU in this subsystem."]
    #[inline(always)]
    pub const fn cpuid(&self) -> &Cpuid {
        &self.cpuid
    }
}
#[doc = "EVENTS_FPUIOC (rw) register accessor: An invalid operation exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuioc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuioc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fpuioc`] module"]
#[doc(alias = "EVENTS_FPUIOC")]
pub type EventsFpuioc = crate::Reg<events_fpuioc::EventsFpuiocSpec>;
#[doc = "An invalid operation exception has occurred in the FPU."]
pub mod events_fpuioc;
#[doc = "EVENTS_FPUDZC (rw) register accessor: A floating-point divide-by-zero exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpudzc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpudzc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fpudzc`] module"]
#[doc(alias = "EVENTS_FPUDZC")]
pub type EventsFpudzc = crate::Reg<events_fpudzc::EventsFpudzcSpec>;
#[doc = "A floating-point divide-by-zero exception has occurred in the FPU."]
pub mod events_fpudzc;
#[doc = "EVENTS_FPUOFC (rw) register accessor: A floating-point overflow exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuofc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuofc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fpuofc`] module"]
#[doc(alias = "EVENTS_FPUOFC")]
pub type EventsFpuofc = crate::Reg<events_fpuofc::EventsFpuofcSpec>;
#[doc = "A floating-point overflow exception has occurred in the FPU."]
pub mod events_fpuofc;
#[doc = "EVENTS_FPUUFC (rw) register accessor: A floating-point underflow exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuufc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuufc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fpuufc`] module"]
#[doc(alias = "EVENTS_FPUUFC")]
pub type EventsFpuufc = crate::Reg<events_fpuufc::EventsFpuufcSpec>;
#[doc = "A floating-point underflow exception has occurred in the FPU."]
pub mod events_fpuufc;
#[doc = "EVENTS_FPUIXC (rw) register accessor: A floating-point inexact exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuixc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuixc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fpuixc`] module"]
#[doc(alias = "EVENTS_FPUIXC")]
pub type EventsFpuixc = crate::Reg<events_fpuixc::EventsFpuixcSpec>;
#[doc = "A floating-point inexact exception has occurred in the FPU."]
pub mod events_fpuixc;
#[doc = "EVENTS_FPUIDC (rw) register accessor: A floating-point input denormal exception has occurred in the FPU.\n\nYou can [`read`](crate::Reg::read) this register and get [`events_fpuidc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_fpuidc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_fpuidc`] module"]
#[doc(alias = "EVENTS_FPUIDC")]
pub type EventsFpuidc = crate::Reg<events_fpuidc::EventsFpuidcSpec>;
#[doc = "A floating-point input denormal exception has occurred in the FPU."]
pub mod events_fpuidc;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`] module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`] module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`] module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
#[doc = "LOCK (rw) register accessor: Register to lock the certain parts of the CPU from being modified.\n\nYou can [`read`](crate::Reg::read) this register and get [`lock::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lock::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@lock`] module"]
#[doc(alias = "LOCK")]
pub type Lock = crate::Reg<lock::LockSpec>;
#[doc = "Register to lock the certain parts of the CPU from being modified."]
pub mod lock;
#[doc = "CPUID (r) register accessor: The identifier for the CPU in this subsystem.\n\nYou can [`read`](crate::Reg::read) this register and get [`cpuid::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cpuid`] module"]
#[doc(alias = "CPUID")]
pub type Cpuid = crate::Reg<cpuid::CpuidSpec>;
#[doc = "The identifier for the CPU in this subsystem."]
pub mod cpuid;
