#[doc = "Register `MODE` reader"]
pub type R = crate::R<ModeSpec>;
#[doc = "Register `MODE` writer"]
pub type W = crate::W<ModeSpec>;
#[doc = "Cache mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    #[doc = "0: Cache mode"]
    Cache = 0,
    #[doc = "1: RAM mode"]
    Ram = 1,
}
impl From<Mode> for bool {
    #[inline(always)]
    fn from(variant: Mode) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MODE` reader - Cache mode"]
pub type ModeR = crate::BitReader<Mode>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mode {
        match self.bits {
            false => Mode::Cache,
            true => Mode::Ram,
        }
    }
    #[doc = "Cache mode"]
    #[inline(always)]
    pub fn is_cache(&self) -> bool {
        *self == Mode::Cache
    }
    #[doc = "RAM mode"]
    #[inline(always)]
    pub fn is_ram(&self) -> bool {
        *self == Mode::Ram
    }
}
#[doc = "Field `MODE` writer - Cache mode"]
pub type ModeW<'a, REG> = crate::BitWriter<'a, REG, Mode>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cache mode"]
    #[inline(always)]
    pub fn cache(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Cache)
    }
    #[doc = "RAM mode"]
    #[inline(always)]
    pub fn ram(self) -> &'a mut crate::W<REG> {
        self.variant(Mode::Ram)
    }
}
#[doc = "RAM size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Ramsize {
    #[doc = "0: All RAM is used for cache memory"]
    All = 0,
    #[doc = "1: Half of the RAM is used for cache memory"]
    Half = 1,
    #[doc = "2: Quarter of the RAM is used for cache memory"]
    Quarter = 2,
    #[doc = "3: None of the RAM is used for cache memory"]
    None = 3,
}
impl From<Ramsize> for u8 {
    #[inline(always)]
    fn from(variant: Ramsize) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Ramsize {
    type Ux = u8;
}
impl crate::IsEnum for Ramsize {}
#[doc = "Field `RAMSIZE` reader - RAM size"]
pub type RamsizeR = crate::FieldReader<Ramsize>;
impl RamsizeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Ramsize {
        match self.bits {
            0 => Ramsize::All,
            1 => Ramsize::Half,
            2 => Ramsize::Quarter,
            3 => Ramsize::None,
            _ => unreachable!(),
        }
    }
    #[doc = "All RAM is used for cache memory"]
    #[inline(always)]
    pub fn is_all(&self) -> bool {
        *self == Ramsize::All
    }
    #[doc = "Half of the RAM is used for cache memory"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == Ramsize::Half
    }
    #[doc = "Quarter of the RAM is used for cache memory"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == Ramsize::Quarter
    }
    #[doc = "None of the RAM is used for cache memory"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == Ramsize::None
    }
}
#[doc = "Field `RAMSIZE` writer - RAM size"]
pub type RamsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Ramsize, crate::Safe>;
impl<'a, REG> RamsizeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "All RAM is used for cache memory"]
    #[inline(always)]
    pub fn all(self) -> &'a mut crate::W<REG> {
        self.variant(Ramsize::All)
    }
    #[doc = "Half of the RAM is used for cache memory"]
    #[inline(always)]
    pub fn half(self) -> &'a mut crate::W<REG> {
        self.variant(Ramsize::Half)
    }
    #[doc = "Quarter of the RAM is used for cache memory"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut crate::W<REG> {
        self.variant(Ramsize::Quarter)
    }
    #[doc = "None of the RAM is used for cache memory"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(Ramsize::None)
    }
}
impl R {
    #[doc = "Bit 0 - Cache mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 4:5 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&self) -> RamsizeR {
        RamsizeR::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Cache mode"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<'_, ModeSpec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 4:5 - RAM size"]
    #[inline(always)]
    pub fn ramsize(&mut self) -> RamsizeW<'_, ModeSpec> {
        RamsizeW::new(self, 4)
    }
}
#[doc = "Cache mode.\n\nYou can [`read`](crate::Reg::read) this register and get [`mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ModeSpec;
impl crate::RegisterSpec for ModeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mode::R`](R) reader structure"]
impl crate::Readable for ModeSpec {}
#[doc = "`write(|w| ..)` method takes [`mode::W`](W) writer structure"]
impl crate::Writable for ModeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MODE to value 0"]
impl crate::Resettable for ModeSpec {}
