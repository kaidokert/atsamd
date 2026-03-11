#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    type_: Type,
    cfg: Cfg,
    ctrl: Ctrl,
    sr: Sr,
    _reserved4: [u8; 0x10],
    maint0: Maint0,
    maint1: Maint1,
    mcfg: Mcfg,
    men: Men,
    mctrl: Mctrl,
    msr: Msr,
}
impl RegisterBlock {
    #[doc = "0x00 - Cache Controller Type Register"]
    #[inline(always)]
    pub const fn type_(&self) -> &Type {
        &self.type_
    }
    #[doc = "0x04 - Cache Controller Configuration Register"]
    #[inline(always)]
    pub const fn cfg(&self) -> &Cfg {
        &self.cfg
    }
    #[doc = "0x08 - Cache Controller Control Register"]
    #[inline(always)]
    pub const fn ctrl(&self) -> &Ctrl {
        &self.ctrl
    }
    #[doc = "0x0c - Cache Controller Status Register"]
    #[inline(always)]
    pub const fn sr(&self) -> &Sr {
        &self.sr
    }
    #[doc = "0x20 - Cache Controller Maintenance Register 0"]
    #[inline(always)]
    pub const fn maint0(&self) -> &Maint0 {
        &self.maint0
    }
    #[doc = "0x24 - Cache Controller Maintenance Register 1"]
    #[inline(always)]
    pub const fn maint1(&self) -> &Maint1 {
        &self.maint1
    }
    #[doc = "0x28 - Cache Controller Monitor Configuration Register"]
    #[inline(always)]
    pub const fn mcfg(&self) -> &Mcfg {
        &self.mcfg
    }
    #[doc = "0x2c - Cache Controller Monitor Enable Register"]
    #[inline(always)]
    pub const fn men(&self) -> &Men {
        &self.men
    }
    #[doc = "0x30 - Cache Controller Monitor Control Register"]
    #[inline(always)]
    pub const fn mctrl(&self) -> &Mctrl {
        &self.mctrl
    }
    #[doc = "0x34 - Cache Controller Monitor Status Register"]
    #[inline(always)]
    pub const fn msr(&self) -> &Msr {
        &self.msr
    }
}
#[doc = "TYPE (r) register accessor: Cache Controller Type Register\n\nYou can [`read`](crate::Reg::read) this register and get [`type_::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@type_`]
module"]
#[doc(alias = "TYPE")]
pub type Type = crate::Reg<type_::TypeSpec>;
#[doc = "Cache Controller Type Register"]
pub mod type_;
#[doc = "CFG (rw) register accessor: Cache Controller Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@cfg`]
module"]
#[doc(alias = "CFG")]
pub type Cfg = crate::Reg<cfg::CfgSpec>;
#[doc = "Cache Controller Configuration Register"]
pub mod cfg;
#[doc = "CTRL (w) register accessor: Cache Controller Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ctrl`]
module"]
#[doc(alias = "CTRL")]
pub type Ctrl = crate::Reg<ctrl::CtrlSpec>;
#[doc = "Cache Controller Control Register"]
pub mod ctrl;
#[doc = "SR (r) register accessor: Cache Controller Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@sr`]
module"]
#[doc(alias = "SR")]
pub type Sr = crate::Reg<sr::SrSpec>;
#[doc = "Cache Controller Status Register"]
pub mod sr;
#[doc = "MAINT0 (w) register accessor: Cache Controller Maintenance Register 0\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maint0::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maint0`]
module"]
#[doc(alias = "MAINT0")]
pub type Maint0 = crate::Reg<maint0::Maint0Spec>;
#[doc = "Cache Controller Maintenance Register 0"]
pub mod maint0;
#[doc = "MAINT1 (w) register accessor: Cache Controller Maintenance Register 1\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maint1::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@maint1`]
module"]
#[doc(alias = "MAINT1")]
pub type Maint1 = crate::Reg<maint1::Maint1Spec>;
#[doc = "Cache Controller Maintenance Register 1"]
pub mod maint1;
#[doc = "MCFG (rw) register accessor: Cache Controller Monitor Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mcfg`]
module"]
#[doc(alias = "MCFG")]
pub type Mcfg = crate::Reg<mcfg::McfgSpec>;
#[doc = "Cache Controller Monitor Configuration Register"]
pub mod mcfg;
#[doc = "MEN (rw) register accessor: Cache Controller Monitor Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`men::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`men::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@men`]
module"]
#[doc(alias = "MEN")]
pub type Men = crate::Reg<men::MenSpec>;
#[doc = "Cache Controller Monitor Enable Register"]
pub mod men;
#[doc = "MCTRL (w) register accessor: Cache Controller Monitor Control Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mctrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@mctrl`]
module"]
#[doc(alias = "MCTRL")]
pub type Mctrl = crate::Reg<mctrl::MctrlSpec>;
#[doc = "Cache Controller Monitor Control Register"]
pub mod mctrl;
#[doc = "MSR (r) register accessor: Cache Controller Monitor Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`msr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@msr`]
module"]
#[doc(alias = "MSR")]
pub type Msr = crate::Reg<msr::MsrSpec>;
#[doc = "Cache Controller Monitor Status Register"]
pub mod msr;
