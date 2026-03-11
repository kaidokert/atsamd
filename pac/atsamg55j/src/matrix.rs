#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    matrix_mcfg: [MatrixMcfg; 3],
    _reserved1: [u8; 0x34],
    matrix_scfg: [MatrixScfg; 4],
    _reserved2: [u8; 0x30],
    matrix_pras0: MatrixPras0,
    _reserved3: [u8; 0x04],
    matrix_pras1: MatrixPras1,
    _reserved4: [u8; 0x04],
    matrix_pras2: MatrixPras2,
    _reserved5: [u8; 0x04],
    matrix_pras3: MatrixPras3,
    _reserved6: [u8; 0x78],
    ccfg_sysio: CcfgSysio,
    ccfg_dynckg: CcfgDynckg,
    ccfg_i2sclksel: CcfgI2sclksel,
    ccfg_usbmr: CcfgUsbmr,
    _reserved10: [u8; 0xc0],
    matrix_wpmr: MatrixWpmr,
    matrix_wpsr: MatrixWpsr,
}
impl RegisterBlock {
    #[doc = "0x00..0x0c - Master Configuration Register"]
    #[inline(always)]
    pub const fn matrix_mcfg(&self, n: usize) -> &MatrixMcfg {
        &self.matrix_mcfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x0c - Master Configuration Register"]
    #[inline(always)]
    pub fn matrix_mcfg_iter(&self) -> impl Iterator<Item = &MatrixMcfg> {
        self.matrix_mcfg.iter()
    }
    #[doc = "0x40..0x50 - Slave Configuration Register"]
    #[inline(always)]
    pub const fn matrix_scfg(&self, n: usize) -> &MatrixScfg {
        &self.matrix_scfg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x40..0x50 - Slave Configuration Register"]
    #[inline(always)]
    pub fn matrix_scfg_iter(&self) -> impl Iterator<Item = &MatrixScfg> {
        self.matrix_scfg.iter()
    }
    #[doc = "0x80 - Priority Register A for Slave 0"]
    #[inline(always)]
    pub const fn matrix_pras0(&self) -> &MatrixPras0 {
        &self.matrix_pras0
    }
    #[doc = "0x88 - Priority Register A for Slave 1"]
    #[inline(always)]
    pub const fn matrix_pras1(&self) -> &MatrixPras1 {
        &self.matrix_pras1
    }
    #[doc = "0x90 - Priority Register A for Slave 2"]
    #[inline(always)]
    pub const fn matrix_pras2(&self) -> &MatrixPras2 {
        &self.matrix_pras2
    }
    #[doc = "0x98 - Priority Register A for Slave 3"]
    #[inline(always)]
    pub const fn matrix_pras3(&self) -> &MatrixPras3 {
        &self.matrix_pras3
    }
    #[doc = "0x114 - System I/O Configuration Register"]
    #[inline(always)]
    pub const fn ccfg_sysio(&self) -> &CcfgSysio {
        &self.ccfg_sysio
    }
    #[doc = "0x118 - Dynamic Clock Gating Register"]
    #[inline(always)]
    pub const fn ccfg_dynckg(&self) -> &CcfgDynckg {
        &self.ccfg_dynckg
    }
    #[doc = "0x11c - I2S Clock Source Selection Register"]
    #[inline(always)]
    pub const fn ccfg_i2sclksel(&self) -> &CcfgI2sclksel {
        &self.ccfg_i2sclksel
    }
    #[doc = "0x120 - USB Management Register"]
    #[inline(always)]
    pub const fn ccfg_usbmr(&self) -> &CcfgUsbmr {
        &self.ccfg_usbmr
    }
    #[doc = "0x1e4 - Write Protection Mode Register"]
    #[inline(always)]
    pub const fn matrix_wpmr(&self) -> &MatrixWpmr {
        &self.matrix_wpmr
    }
    #[doc = "0x1e8 - Write Protection Status Register"]
    #[inline(always)]
    pub const fn matrix_wpsr(&self) -> &MatrixWpsr {
        &self.matrix_wpsr
    }
}
#[doc = "MATRIX_MCFG (rw) register accessor: Master Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_mcfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_mcfg`]
module"]
#[doc(alias = "MATRIX_MCFG")]
pub type MatrixMcfg = crate::Reg<matrix_mcfg::MatrixMcfgSpec>;
#[doc = "Master Configuration Register"]
pub mod matrix_mcfg;
#[doc = "MATRIX_SCFG (rw) register accessor: Slave Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_scfg::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_scfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_scfg`]
module"]
#[doc(alias = "MATRIX_SCFG")]
pub type MatrixScfg = crate::Reg<matrix_scfg::MatrixScfgSpec>;
#[doc = "Slave Configuration Register"]
pub mod matrix_scfg;
#[doc = "MATRIX_PRAS0 (rw) register accessor: Priority Register A for Slave 0\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras0`]
module"]
#[doc(alias = "MATRIX_PRAS0")]
pub type MatrixPras0 = crate::Reg<matrix_pras0::MatrixPras0Spec>;
#[doc = "Priority Register A for Slave 0"]
pub mod matrix_pras0;
#[doc = "MATRIX_PRAS1 (rw) register accessor: Priority Register A for Slave 1\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras1`]
module"]
#[doc(alias = "MATRIX_PRAS1")]
pub type MatrixPras1 = crate::Reg<matrix_pras1::MatrixPras1Spec>;
#[doc = "Priority Register A for Slave 1"]
pub mod matrix_pras1;
#[doc = "MATRIX_PRAS2 (rw) register accessor: Priority Register A for Slave 2\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras2`]
module"]
#[doc(alias = "MATRIX_PRAS2")]
pub type MatrixPras2 = crate::Reg<matrix_pras2::MatrixPras2Spec>;
#[doc = "Priority Register A for Slave 2"]
pub mod matrix_pras2;
#[doc = "MATRIX_PRAS3 (rw) register accessor: Priority Register A for Slave 3\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_pras3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_pras3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_pras3`]
module"]
#[doc(alias = "MATRIX_PRAS3")]
pub type MatrixPras3 = crate::Reg<matrix_pras3::MatrixPras3Spec>;
#[doc = "Priority Register A for Slave 3"]
pub mod matrix_pras3;
#[doc = "CCFG_SYSIO (rw) register accessor: System I/O Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_sysio::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_sysio::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_sysio`]
module"]
#[doc(alias = "CCFG_SYSIO")]
pub type CcfgSysio = crate::Reg<ccfg_sysio::CcfgSysioSpec>;
#[doc = "System I/O Configuration Register"]
pub mod ccfg_sysio;
#[doc = "CCFG_DYNCKG (rw) register accessor: Dynamic Clock Gating Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_dynckg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_dynckg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_dynckg`]
module"]
#[doc(alias = "CCFG_DYNCKG")]
pub type CcfgDynckg = crate::Reg<ccfg_dynckg::CcfgDynckgSpec>;
#[doc = "Dynamic Clock Gating Register"]
pub mod ccfg_dynckg;
#[doc = "CCFG_I2SCLKSEL (rw) register accessor: I2S Clock Source Selection Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_i2sclksel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_i2sclksel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_i2sclksel`]
module"]
#[doc(alias = "CCFG_I2SCLKSEL")]
pub type CcfgI2sclksel = crate::Reg<ccfg_i2sclksel::CcfgI2sclkselSpec>;
#[doc = "I2S Clock Source Selection Register"]
pub mod ccfg_i2sclksel;
#[doc = "CCFG_USBMR (rw) register accessor: USB Management Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg_usbmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg_usbmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ccfg_usbmr`]
module"]
#[doc(alias = "CCFG_USBMR")]
pub type CcfgUsbmr = crate::Reg<ccfg_usbmr::CcfgUsbmrSpec>;
#[doc = "USB Management Register"]
pub mod ccfg_usbmr;
#[doc = "MATRIX_WPMR (rw) register accessor: Write Protection Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_wpmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`matrix_wpmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpmr`]
module"]
#[doc(alias = "MATRIX_WPMR")]
pub type MatrixWpmr = crate::Reg<matrix_wpmr::MatrixWpmrSpec>;
#[doc = "Write Protection Mode Register"]
pub mod matrix_wpmr;
#[doc = "MATRIX_WPSR (r) register accessor: Write Protection Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`matrix_wpsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@matrix_wpsr`]
module"]
#[doc(alias = "MATRIX_WPSR")]
pub type MatrixWpsr = crate::Reg<matrix_wpsr::MatrixWpsrSpec>;
#[doc = "Write Protection Status Register"]
pub mod matrix_wpsr;
