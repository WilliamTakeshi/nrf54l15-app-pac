#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event FPUIOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuioc {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fpuioc> for bool {
    #[inline(always)]
    fn from(variant: Fpuioc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIOC` reader - Write '1' to enable interrupt for event FPUIOC"]
pub type FpuiocR = crate::BitReader<Fpuioc>;
impl FpuiocR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpuioc {
        match self.bits {
            false => Fpuioc::Disabled,
            true => Fpuioc::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuioc::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuioc::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FPUIOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuiocWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FpuiocWO> for bool {
    #[inline(always)]
    fn from(variant: FpuiocWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIOC` writer - Write '1' to enable interrupt for event FPUIOC"]
pub type FpuiocW<'a, REG> = crate::BitWriter<'a, REG, FpuiocWO>;
impl<'a, REG> FpuiocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FpuiocWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event FPUDZC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpudzc {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fpudzc> for bool {
    #[inline(always)]
    fn from(variant: Fpudzc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUDZC` reader - Write '1' to enable interrupt for event FPUDZC"]
pub type FpudzcR = crate::BitReader<Fpudzc>;
impl FpudzcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpudzc {
        match self.bits {
            false => Fpudzc::Disabled,
            true => Fpudzc::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpudzc::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpudzc::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FPUDZC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpudzcWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FpudzcWO> for bool {
    #[inline(always)]
    fn from(variant: FpudzcWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUDZC` writer - Write '1' to enable interrupt for event FPUDZC"]
pub type FpudzcW<'a, REG> = crate::BitWriter<'a, REG, FpudzcWO>;
impl<'a, REG> FpudzcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FpudzcWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event FPUOFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuofc {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fpuofc> for bool {
    #[inline(always)]
    fn from(variant: Fpuofc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUOFC` reader - Write '1' to enable interrupt for event FPUOFC"]
pub type FpuofcR = crate::BitReader<Fpuofc>;
impl FpuofcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpuofc {
        match self.bits {
            false => Fpuofc::Disabled,
            true => Fpuofc::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuofc::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuofc::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FPUOFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuofcWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FpuofcWO> for bool {
    #[inline(always)]
    fn from(variant: FpuofcWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUOFC` writer - Write '1' to enable interrupt for event FPUOFC"]
pub type FpuofcW<'a, REG> = crate::BitWriter<'a, REG, FpuofcWO>;
impl<'a, REG> FpuofcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FpuofcWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event FPUUFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuufc {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fpuufc> for bool {
    #[inline(always)]
    fn from(variant: Fpuufc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUUFC` reader - Write '1' to enable interrupt for event FPUUFC"]
pub type FpuufcR = crate::BitReader<Fpuufc>;
impl FpuufcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpuufc {
        match self.bits {
            false => Fpuufc::Disabled,
            true => Fpuufc::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuufc::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuufc::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FPUUFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuufcWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FpuufcWO> for bool {
    #[inline(always)]
    fn from(variant: FpuufcWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUUFC` writer - Write '1' to enable interrupt for event FPUUFC"]
pub type FpuufcW<'a, REG> = crate::BitWriter<'a, REG, FpuufcWO>;
impl<'a, REG> FpuufcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FpuufcWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event FPUIXC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuixc {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fpuixc> for bool {
    #[inline(always)]
    fn from(variant: Fpuixc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIXC` reader - Write '1' to enable interrupt for event FPUIXC"]
pub type FpuixcR = crate::BitReader<Fpuixc>;
impl FpuixcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpuixc {
        match self.bits {
            false => Fpuixc::Disabled,
            true => Fpuixc::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuixc::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuixc::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FPUIXC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuixcWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FpuixcWO> for bool {
    #[inline(always)]
    fn from(variant: FpuixcWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIXC` writer - Write '1' to enable interrupt for event FPUIXC"]
pub type FpuixcW<'a, REG> = crate::BitWriter<'a, REG, FpuixcWO>;
impl<'a, REG> FpuixcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FpuixcWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event FPUIDC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuidc {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Fpuidc> for bool {
    #[inline(always)]
    fn from(variant: Fpuidc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIDC` reader - Write '1' to enable interrupt for event FPUIDC"]
pub type FpuidcR = crate::BitReader<Fpuidc>;
impl FpuidcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Fpuidc {
        match self.bits {
            false => Fpuidc::Disabled,
            true => Fpuidc::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuidc::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuidc::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event FPUIDC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FpuidcWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<FpuidcWO> for bool {
    #[inline(always)]
    fn from(variant: FpuidcWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIDC` writer - Write '1' to enable interrupt for event FPUIDC"]
pub type FpuidcW<'a, REG> = crate::BitWriter<'a, REG, FpuidcWO>;
impl<'a, REG> FpuidcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(FpuidcWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event FPUIOC"]
    #[inline(always)]
    pub fn fpuioc(&self) -> FpuiocR {
        FpuiocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event FPUDZC"]
    #[inline(always)]
    pub fn fpudzc(&self) -> FpudzcR {
        FpudzcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event FPUOFC"]
    #[inline(always)]
    pub fn fpuofc(&self) -> FpuofcR {
        FpuofcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event FPUUFC"]
    #[inline(always)]
    pub fn fpuufc(&self) -> FpuufcR {
        FpuufcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event FPUIXC"]
    #[inline(always)]
    pub fn fpuixc(&self) -> FpuixcR {
        FpuixcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event FPUIDC"]
    #[inline(always)]
    pub fn fpuidc(&self) -> FpuidcR {
        FpuidcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event FPUIOC"]
    #[inline(always)]
    pub fn fpuioc(&mut self) -> FpuiocW<'_, IntensetSpec> {
        FpuiocW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event FPUDZC"]
    #[inline(always)]
    pub fn fpudzc(&mut self) -> FpudzcW<'_, IntensetSpec> {
        FpudzcW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event FPUOFC"]
    #[inline(always)]
    pub fn fpuofc(&mut self) -> FpuofcW<'_, IntensetSpec> {
        FpuofcW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event FPUUFC"]
    #[inline(always)]
    pub fn fpuufc(&mut self) -> FpuufcW<'_, IntensetSpec> {
        FpuufcW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event FPUIXC"]
    #[inline(always)]
    pub fn fpuixc(&mut self) -> FpuixcW<'_, IntensetSpec> {
        FpuixcW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event FPUIDC"]
    #[inline(always)]
    pub fn fpuidc(&mut self) -> FpuidcW<'_, IntensetSpec> {
        FpuidcW::new(self, 5)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {}
