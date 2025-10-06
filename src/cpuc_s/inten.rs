#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event FPUIOC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuioc {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Fpuioc> for bool {
    #[inline(always)]
    fn from(variant: Fpuioc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIOC` reader - Enable or disable interrupt for event FPUIOC"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuioc::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuioc::Enabled
    }
}
#[doc = "Field `FPUIOC` writer - Enable or disable interrupt for event FPUIOC"]
pub type FpuiocW<'a, REG> = crate::BitWriter<'a, REG, Fpuioc>;
impl<'a, REG> FpuiocW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuioc::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuioc::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event FPUDZC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpudzc {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Fpudzc> for bool {
    #[inline(always)]
    fn from(variant: Fpudzc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUDZC` reader - Enable or disable interrupt for event FPUDZC"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpudzc::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpudzc::Enabled
    }
}
#[doc = "Field `FPUDZC` writer - Enable or disable interrupt for event FPUDZC"]
pub type FpudzcW<'a, REG> = crate::BitWriter<'a, REG, Fpudzc>;
impl<'a, REG> FpudzcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpudzc::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpudzc::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event FPUOFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuofc {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Fpuofc> for bool {
    #[inline(always)]
    fn from(variant: Fpuofc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUOFC` reader - Enable or disable interrupt for event FPUOFC"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuofc::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuofc::Enabled
    }
}
#[doc = "Field `FPUOFC` writer - Enable or disable interrupt for event FPUOFC"]
pub type FpuofcW<'a, REG> = crate::BitWriter<'a, REG, Fpuofc>;
impl<'a, REG> FpuofcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuofc::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuofc::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event FPUUFC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuufc {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Fpuufc> for bool {
    #[inline(always)]
    fn from(variant: Fpuufc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUUFC` reader - Enable or disable interrupt for event FPUUFC"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuufc::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuufc::Enabled
    }
}
#[doc = "Field `FPUUFC` writer - Enable or disable interrupt for event FPUUFC"]
pub type FpuufcW<'a, REG> = crate::BitWriter<'a, REG, Fpuufc>;
impl<'a, REG> FpuufcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuufc::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuufc::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event FPUIXC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuixc {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Fpuixc> for bool {
    #[inline(always)]
    fn from(variant: Fpuixc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIXC` reader - Enable or disable interrupt for event FPUIXC"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuixc::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuixc::Enabled
    }
}
#[doc = "Field `FPUIXC` writer - Enable or disable interrupt for event FPUIXC"]
pub type FpuixcW<'a, REG> = crate::BitWriter<'a, REG, Fpuixc>;
impl<'a, REG> FpuixcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuixc::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuixc::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event FPUIDC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Fpuidc {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Fpuidc> for bool {
    #[inline(always)]
    fn from(variant: Fpuidc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FPUIDC` reader - Enable or disable interrupt for event FPUIDC"]
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
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Fpuidc::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Fpuidc::Enabled
    }
}
#[doc = "Field `FPUIDC` writer - Enable or disable interrupt for event FPUIDC"]
pub type FpuidcW<'a, REG> = crate::BitWriter<'a, REG, Fpuidc>;
impl<'a, REG> FpuidcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuidc::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Fpuidc::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event FPUIOC"]
    #[inline(always)]
    pub fn fpuioc(&self) -> FpuiocR {
        FpuiocR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FPUDZC"]
    #[inline(always)]
    pub fn fpudzc(&self) -> FpudzcR {
        FpudzcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event FPUOFC"]
    #[inline(always)]
    pub fn fpuofc(&self) -> FpuofcR {
        FpuofcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event FPUUFC"]
    #[inline(always)]
    pub fn fpuufc(&self) -> FpuufcR {
        FpuufcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event FPUIXC"]
    #[inline(always)]
    pub fn fpuixc(&self) -> FpuixcR {
        FpuixcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event FPUIDC"]
    #[inline(always)]
    pub fn fpuidc(&self) -> FpuidcR {
        FpuidcR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event FPUIOC"]
    #[inline(always)]
    pub fn fpuioc(&mut self) -> FpuiocW<'_, IntenSpec> {
        FpuiocW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event FPUDZC"]
    #[inline(always)]
    pub fn fpudzc(&mut self) -> FpudzcW<'_, IntenSpec> {
        FpudzcW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event FPUOFC"]
    #[inline(always)]
    pub fn fpuofc(&mut self) -> FpuofcW<'_, IntenSpec> {
        FpuofcW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event FPUUFC"]
    #[inline(always)]
    pub fn fpuufc(&mut self) -> FpuufcW<'_, IntenSpec> {
        FpuufcW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event FPUIXC"]
    #[inline(always)]
    pub fn fpuixc(&mut self) -> FpuixcW<'_, IntenSpec> {
        FpuixcW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event FPUIDC"]
    #[inline(always)]
    pub fn fpuidc(&mut self) -> FpuidcW<'_, IntenSpec> {
        FpuidcW::new(self, 5)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {}
