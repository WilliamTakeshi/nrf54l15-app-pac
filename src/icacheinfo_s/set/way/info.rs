#[doc = "Register `INFO` reader"]
pub type R = crate::R<InfoSpec>;
#[doc = "Register `INFO` writer"]
pub type W = crate::W<InfoSpec>;
#[doc = "Field `TAG` reader - Cache tag."]
pub type TagR = crate::FieldReader<u32>;
#[doc = "Data unit valid info.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duv0 {
    #[doc = "0: Invalid data unit"]
    Invalid = 0,
    #[doc = "1: Valid data unit"]
    Valid = 1,
}
impl From<Duv0> for bool {
    #[inline(always)]
    fn from(variant: Duv0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUV_0` reader - Data unit valid info."]
pub type Duv0R = crate::BitReader<Duv0>;
impl Duv0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duv0 {
        match self.bits {
            false => Duv0::Invalid,
            true => Duv0::Valid,
        }
    }
    #[doc = "Invalid data unit"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Duv0::Invalid
    }
    #[doc = "Valid data unit"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Duv0::Valid
    }
}
#[doc = "Data unit valid info.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duv1 {
    #[doc = "0: Invalid data unit"]
    Invalid = 0,
    #[doc = "1: Valid data unit"]
    Valid = 1,
}
impl From<Duv1> for bool {
    #[inline(always)]
    fn from(variant: Duv1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUV_1` reader - Data unit valid info."]
pub type Duv1R = crate::BitReader<Duv1>;
impl Duv1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duv1 {
        match self.bits {
            false => Duv1::Invalid,
            true => Duv1::Valid,
        }
    }
    #[doc = "Invalid data unit"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Duv1::Invalid
    }
    #[doc = "Valid data unit"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Duv1::Valid
    }
}
#[doc = "Data unit valid info.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duv2 {
    #[doc = "0: Invalid data unit"]
    Invalid = 0,
    #[doc = "1: Valid data unit"]
    Valid = 1,
}
impl From<Duv2> for bool {
    #[inline(always)]
    fn from(variant: Duv2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUV_2` reader - Data unit valid info."]
pub type Duv2R = crate::BitReader<Duv2>;
impl Duv2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duv2 {
        match self.bits {
            false => Duv2::Invalid,
            true => Duv2::Valid,
        }
    }
    #[doc = "Invalid data unit"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Duv2::Invalid
    }
    #[doc = "Valid data unit"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Duv2::Valid
    }
}
#[doc = "Data unit valid info.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Duv3 {
    #[doc = "0: Invalid data unit"]
    Invalid = 0,
    #[doc = "1: Valid data unit"]
    Valid = 1,
}
impl From<Duv3> for bool {
    #[inline(always)]
    fn from(variant: Duv3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `DUV_3` reader - Data unit valid info."]
pub type Duv3R = crate::BitReader<Duv3>;
impl Duv3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Duv3 {
        match self.bits {
            false => Duv3::Invalid,
            true => Duv3::Valid,
        }
    }
    #[doc = "Invalid data unit"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == Duv3::Invalid
    }
    #[doc = "Valid data unit"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == Duv3::Valid
    }
}
#[doc = "Line valid bit.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum V {
    #[doc = "0: Invalid cache line"]
    Invalid = 0,
    #[doc = "1: Valid cache line"]
    Valid = 1,
}
impl From<V> for bool {
    #[inline(always)]
    fn from(variant: V) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `V` reader - Line valid bit."]
pub type VR = crate::BitReader<V>;
impl VR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> V {
        match self.bits {
            false => V::Invalid,
            true => V::Valid,
        }
    }
    #[doc = "Invalid cache line"]
    #[inline(always)]
    pub fn is_invalid(&self) -> bool {
        *self == V::Invalid
    }
    #[doc = "Valid cache line"]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        *self == V::Valid
    }
}
#[doc = "Most recently used way.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mru {
    #[doc = "0: Way0 was most recently used"]
    Way0 = 0,
    #[doc = "1: Way1 was most recently used"]
    Way1 = 1,
}
impl From<Mru> for bool {
    #[inline(always)]
    fn from(variant: Mru) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `MRU` reader - Most recently used way."]
pub type MruR = crate::BitReader<Mru>;
impl MruR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Mru {
        match self.bits {
            false => Mru::Way0,
            true => Mru::Way1,
        }
    }
    #[doc = "Way0 was most recently used"]
    #[inline(always)]
    pub fn is_way0(&self) -> bool {
        *self == Mru::Way0
    }
    #[doc = "Way1 was most recently used"]
    #[inline(always)]
    pub fn is_way1(&self) -> bool {
        *self == Mru::Way1
    }
}
impl R {
    #[doc = "Bits 0:23 - Cache tag."]
    #[inline(always)]
    pub fn tag(&self) -> TagR {
        TagR::new(self.bits & 0x00ff_ffff)
    }
    #[doc = "Bit 24 - Data unit valid info."]
    #[inline(always)]
    pub fn duv_0(&self) -> Duv0R {
        Duv0R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Data unit valid info."]
    #[inline(always)]
    pub fn duv_1(&self) -> Duv1R {
        Duv1R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Data unit valid info."]
    #[inline(always)]
    pub fn duv_2(&self) -> Duv2R {
        Duv2R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Data unit valid info."]
    #[inline(always)]
    pub fn duv_3(&self) -> Duv3R {
        Duv3R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 30 - Line valid bit."]
    #[inline(always)]
    pub fn v(&self) -> VR {
        VR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Most recently used way."]
    #[inline(always)]
    pub fn mru(&self) -> MruR {
        MruR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {}
#[doc = "Description cluster: Cache information for SET\\[n\\], WAY\\[o\\].\n\nYou can [`read`](crate::Reg::read) this register and get [`info::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`info::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct InfoSpec;
impl crate::RegisterSpec for InfoSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`info::R`](R) reader structure"]
impl crate::Readable for InfoSpec {}
#[doc = "`write(|w| ..)` method takes [`info::W`](W) writer structure"]
impl crate::Writable for InfoSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INFO to value 0"]
impl crate::Resettable for InfoSpec {}
