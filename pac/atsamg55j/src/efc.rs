#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    fmr: Fmr,
    fcr: Fcr,
    fsr: Fsr,
    frr: Frr,
    _reserved4: [u8; 0xd4],
    wpmr: Wpmr,
}
impl RegisterBlock {
    #[doc = "0x00 - EEFC Flash Mode Register"]
    #[inline(always)]
    pub const fn fmr(&self) -> &Fmr {
        &self.fmr
    }
    #[doc = "0x04 - EEFC Flash Command Register"]
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    #[doc = "0x08 - EEFC Flash Status Register"]
    #[inline(always)]
    pub const fn fsr(&self) -> &Fsr {
        &self.fsr
    }
    #[doc = "0x0c - EEFC Flash Result Register"]
    #[inline(always)]
    pub const fn frr(&self) -> &Frr {
        &self.frr
    }
    #[doc = "0xe4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn wpmr(&self) -> &Wpmr {
        &self.wpmr
    }
}
#[doc = "FMR (rw) register accessor: EEFC Flash Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fmr`]
module"]
#[doc(alias = "FMR")]
pub type Fmr = crate::Reg<fmr::FmrSpec>;
#[doc = "EEFC Flash Mode Register"]
pub mod fmr;
#[doc = "FCR (w) register accessor: EEFC Flash Command Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fcr`]
module"]
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
#[doc = "EEFC Flash Command Register"]
pub mod fcr;
#[doc = "FSR (r) register accessor: EEFC Flash Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fsr`]
module"]
#[doc(alias = "FSR")]
pub type Fsr = crate::Reg<fsr::FsrSpec>;
#[doc = "EEFC Flash Status Register"]
pub mod fsr;
#[doc = "FRR (r) register accessor: EEFC Flash Result Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frr`]
module"]
#[doc(alias = "FRR")]
pub type Frr = crate::Reg<frr::FrrSpec>;
#[doc = "EEFC Flash Result Register"]
pub mod frr;
#[doc = "WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@wpmr`]
module"]
#[doc(alias = "WPMR")]
pub type Wpmr = crate::Reg<wpmr::WpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod wpmr;
