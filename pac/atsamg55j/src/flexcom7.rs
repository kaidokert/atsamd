#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    mr: Mr,
    _reserved1: [u8; 0x0c],
    rhr: Rhr,
    _reserved2: [u8; 0x0c],
    thr: Thr,
}
impl RegisterBlock {
    #[doc = "0x00 - FLEXCOM Mode register"]
    #[inline(always)]
    pub const fn mr(&self) -> &Mr {
        &self.mr
    }
    #[doc = "0x10 - FLEXCOM Receive Holding Register"]
    #[inline(always)]
    pub const fn rhr(&self) -> &Rhr {
        &self.rhr
    }
    #[doc = "0x20 - FLEXCOM Transmit Holding Register"]
    #[inline(always)]
    pub const fn thr(&self) -> &Thr {
        &self.thr
    }
}
#[doc = "MR (rw) register accessor: FLEXCOM Mode register\n\nYou can [`read`](crate::Reg::read) this register and get [`mr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mr`]
module"]
#[doc(alias = "MR")]
pub type Mr = crate::Reg<mr::MrSpec>;
#[doc = "FLEXCOM Mode register"]
pub mod mr;
#[doc = "RHR (r) register accessor: FLEXCOM Receive Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rhr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rhr`]
module"]
#[doc(alias = "RHR")]
pub type Rhr = crate::Reg<rhr::RhrSpec>;
#[doc = "FLEXCOM Receive Holding Register"]
pub mod rhr;
#[doc = "THR (rw) register accessor: FLEXCOM Transmit Holding Register\n\nYou can [`read`](crate::Reg::read) this register and get [`thr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`thr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@thr`]
module"]
#[doc(alias = "THR")]
pub type Thr = crate::Reg<thr::ThrSpec>;
#[doc = "FLEXCOM Transmit Holding Register"]
pub mod thr;
