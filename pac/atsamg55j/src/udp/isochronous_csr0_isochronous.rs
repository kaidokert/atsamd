#[doc = "Register `CSR0_ISOCHRONOUS` reader"]
pub type R = crate::R<IsochronousCsr0IsochronousSpec>;
#[doc = "Register `CSR0_ISOCHRONOUS` writer"]
pub type W = crate::W<IsochronousCsr0IsochronousSpec>;
#[doc = "Field `TXCOMP` reader - Generates an IN Packet with Data Previously Written in the DPR"]
pub type TxcompR = crate::BitReader;
#[doc = "Field `TXCOMP` writer - Generates an IN Packet with Data Previously Written in the DPR"]
pub type TxcompW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_BK0` reader - Receive Data Bank 0"]
pub type RxDataBk0R = crate::BitReader;
#[doc = "Field `RX_DATA_BK0` writer - Receive Data Bank 0"]
pub type RxDataBk0W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXSETUP` reader - Received Setup"]
pub type RxsetupR = crate::BitReader;
#[doc = "Field `RXSETUP` writer - Received Setup"]
pub type RxsetupW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ISOERROR` reader - A CRC error has been detected in an isochronous transfer"]
pub type IsoerrorR = crate::BitReader;
#[doc = "Field `ISOERROR` writer - A CRC error has been detected in an isochronous transfer"]
pub type IsoerrorW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXPKTRDY` reader - Transmit Packet Ready"]
pub type TxpktrdyR = crate::BitReader;
#[doc = "Field `TXPKTRDY` writer - Transmit Packet Ready"]
pub type TxpktrdyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FORCESTALL` reader - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub type ForcestallR = crate::BitReader;
#[doc = "Field `FORCESTALL` writer - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
pub type ForcestallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_DATA_BK1` reader - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub type RxDataBk1R = crate::BitReader;
#[doc = "Field `RX_DATA_BK1` writer - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
pub type RxDataBk1W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Transfer Direction (only available for control endpoints) (Read/Write)"]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Transfer Direction (only available for control endpoints) (Read/Write)"]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Endpoint Type (Read/Write)"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Eptype {
    #[doc = "0: Control"]
    Ctrl = 0,
    #[doc = "1: Isochronous OUT"]
    IsoOut = 1,
    #[doc = "5: Isochronous IN"]
    IsoIn = 5,
    #[doc = "2: Bulk OUT"]
    BulkOut = 2,
    #[doc = "6: Bulk IN"]
    BulkIn = 6,
    #[doc = "3: Interrupt OUT"]
    IntOut = 3,
    #[doc = "7: Interrupt IN"]
    IntIn = 7,
}
impl From<Eptype> for u8 {
    #[inline(always)]
    fn from(variant: Eptype) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Eptype {
    type Ux = u8;
}
impl crate::IsEnum for Eptype {}
#[doc = "Field `EPTYPE` reader - Endpoint Type (Read/Write)"]
pub type EptypeR = crate::FieldReader<Eptype>;
impl EptypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Eptype> {
        match self.bits {
            0 => Some(Eptype::Ctrl),
            1 => Some(Eptype::IsoOut),
            5 => Some(Eptype::IsoIn),
            2 => Some(Eptype::BulkOut),
            6 => Some(Eptype::BulkIn),
            3 => Some(Eptype::IntOut),
            7 => Some(Eptype::IntIn),
            _ => None,
        }
    }
    #[doc = "Control"]
    #[inline(always)]
    pub fn is_ctrl(&self) -> bool {
        *self == Eptype::Ctrl
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn is_iso_out(&self) -> bool {
        *self == Eptype::IsoOut
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn is_iso_in(&self) -> bool {
        *self == Eptype::IsoIn
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn is_bulk_out(&self) -> bool {
        *self == Eptype::BulkOut
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn is_bulk_in(&self) -> bool {
        *self == Eptype::BulkIn
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn is_int_out(&self) -> bool {
        *self == Eptype::IntOut
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn is_int_in(&self) -> bool {
        *self == Eptype::IntIn
    }
}
#[doc = "Field `EPTYPE` writer - Endpoint Type (Read/Write)"]
pub type EptypeW<'a, REG> = crate::FieldWriter<'a, REG, 3, Eptype>;
impl<'a, REG> EptypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Control"]
    #[inline(always)]
    pub fn ctrl(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::Ctrl)
    }
    #[doc = "Isochronous OUT"]
    #[inline(always)]
    pub fn iso_out(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::IsoOut)
    }
    #[doc = "Isochronous IN"]
    #[inline(always)]
    pub fn iso_in(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::IsoIn)
    }
    #[doc = "Bulk OUT"]
    #[inline(always)]
    pub fn bulk_out(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::BulkOut)
    }
    #[doc = "Bulk IN"]
    #[inline(always)]
    pub fn bulk_in(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::BulkIn)
    }
    #[doc = "Interrupt OUT"]
    #[inline(always)]
    pub fn int_out(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::IntOut)
    }
    #[doc = "Interrupt IN"]
    #[inline(always)]
    pub fn int_in(self) -> &'a mut crate::W<REG> {
        self.variant(Eptype::IntIn)
    }
}
#[doc = "Field `DTGLE` reader - Data Toggle (Read-only)"]
pub type DtgleR = crate::BitReader;
#[doc = "Field `DTGLE` writer - Data Toggle (Read-only)"]
pub type DtgleW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EPEDS` reader - Endpoint Enable Disable"]
pub type EpedsR = crate::BitReader;
#[doc = "Field `EPEDS` writer - Endpoint Enable Disable"]
pub type EpedsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXBYTECNT` reader - Number of Bytes Available in the FIFO (Read-only)"]
pub type RxbytecntR = crate::FieldReader<u16>;
#[doc = "Field `RXBYTECNT` writer - Number of Bytes Available in the FIFO (Read-only)"]
pub type RxbytecntW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    pub fn txcomp(&self) -> TxcompR {
        TxcompR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    pub fn rx_data_bk0(&self) -> RxDataBk0R {
        RxDataBk0R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    pub fn rxsetup(&self) -> RxsetupR {
        RxsetupR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    pub fn isoerror(&self) -> IsoerrorR {
        IsoerrorR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    pub fn txpktrdy(&self) -> TxpktrdyR {
        TxpktrdyR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    pub fn forcestall(&self) -> ForcestallR {
        ForcestallR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    pub fn rx_data_bk1(&self) -> RxDataBk1R {
        RxDataBk1R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints) (Read/Write)"]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Endpoint Type (Read/Write)"]
    #[inline(always)]
    pub fn eptype(&self) -> EptypeR {
        EptypeR::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - Data Toggle (Read-only)"]
    #[inline(always)]
    pub fn dtgle(&self) -> DtgleR {
        DtgleR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    pub fn epeds(&self) -> EpedsR {
        EpedsR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO (Read-only)"]
    #[inline(always)]
    pub fn rxbytecnt(&self) -> RxbytecntR {
        RxbytecntR::new(((self.bits >> 16) & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - Generates an IN Packet with Data Previously Written in the DPR"]
    #[inline(always)]
    #[must_use]
    pub fn txcomp(&mut self) -> TxcompW<IsochronousCsr0IsochronousSpec> {
        TxcompW::new(self, 0)
    }
    #[doc = "Bit 1 - Receive Data Bank 0"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_bk0(&mut self) -> RxDataBk0W<IsochronousCsr0IsochronousSpec> {
        RxDataBk0W::new(self, 1)
    }
    #[doc = "Bit 2 - Received Setup"]
    #[inline(always)]
    #[must_use]
    pub fn rxsetup(&mut self) -> RxsetupW<IsochronousCsr0IsochronousSpec> {
        RxsetupW::new(self, 2)
    }
    #[doc = "Bit 3 - A CRC error has been detected in an isochronous transfer"]
    #[inline(always)]
    #[must_use]
    pub fn isoerror(&mut self) -> IsoerrorW<IsochronousCsr0IsochronousSpec> {
        IsoerrorW::new(self, 3)
    }
    #[doc = "Bit 4 - Transmit Packet Ready"]
    #[inline(always)]
    #[must_use]
    pub fn txpktrdy(&mut self) -> TxpktrdyW<IsochronousCsr0IsochronousSpec> {
        TxpktrdyW::new(self, 4)
    }
    #[doc = "Bit 5 - Force Stall (used by Control, Bulk and Isochronous Endpoints)"]
    #[inline(always)]
    #[must_use]
    pub fn forcestall(&mut self) -> ForcestallW<IsochronousCsr0IsochronousSpec> {
        ForcestallW::new(self, 5)
    }
    #[doc = "Bit 6 - Receive Data Bank 1 (only used by endpoints with ping-pong attributes)"]
    #[inline(always)]
    #[must_use]
    pub fn rx_data_bk1(&mut self) -> RxDataBk1W<IsochronousCsr0IsochronousSpec> {
        RxDataBk1W::new(self, 6)
    }
    #[doc = "Bit 7 - Transfer Direction (only available for control endpoints) (Read/Write)"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DirW<IsochronousCsr0IsochronousSpec> {
        DirW::new(self, 7)
    }
    #[doc = "Bits 8:10 - Endpoint Type (Read/Write)"]
    #[inline(always)]
    #[must_use]
    pub fn eptype(&mut self) -> EptypeW<IsochronousCsr0IsochronousSpec> {
        EptypeW::new(self, 8)
    }
    #[doc = "Bit 11 - Data Toggle (Read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn dtgle(&mut self) -> DtgleW<IsochronousCsr0IsochronousSpec> {
        DtgleW::new(self, 11)
    }
    #[doc = "Bit 15 - Endpoint Enable Disable"]
    #[inline(always)]
    #[must_use]
    pub fn epeds(&mut self) -> EpedsW<IsochronousCsr0IsochronousSpec> {
        EpedsW::new(self, 15)
    }
    #[doc = "Bits 16:26 - Number of Bytes Available in the FIFO (Read-only)"]
    #[inline(always)]
    #[must_use]
    pub fn rxbytecnt(&mut self) -> RxbytecntW<IsochronousCsr0IsochronousSpec> {
        RxbytecntW::new(self, 16)
    }
}
#[doc = "Endpoint Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isochronous_csr0_isochronous::R`](R). You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isochronous_csr0_isochronous::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsochronousCsr0IsochronousSpec;
impl crate::RegisterSpec for IsochronousCsr0IsochronousSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isochronous_csr0_isochronous::R`](R) reader structure"]
impl crate::Readable for IsochronousCsr0IsochronousSpec {}
#[doc = "`write(|w| ..)` method takes [`isochronous_csr0_isochronous::W`](W) writer structure"]
impl crate::Writable for IsochronousCsr0IsochronousSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
