#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    frm_num: FrmNum,
    glb_stat: GlbStat,
    faddr: Faddr,
    _reserved3: [u8; 0x04],
    ier: Ier,
    idr: Idr,
    imr: Imr,
    isr: Isr,
    icr: Icr,
    _reserved8: [u8; 0x04],
    rst_ep: RstEp,
    _reserved9: [u8; 0x04],
    _reserved_9_csr: [u8; 0x18],
    _reserved10: [u8; 0x08],
    fdr: [Fdr; 6],
    _reserved11: [u8; 0x0c],
    txvc: Txvc,
}
impl RegisterBlock {
    #[doc = "0x00 - Frame Number Register"]
    #[inline(always)]
    pub const fn frm_num(&self) -> &FrmNum {
        &self.frm_num
    }
    #[doc = "0x04 - Global State Register"]
    #[inline(always)]
    pub const fn glb_stat(&self) -> &GlbStat {
        &self.glb_stat
    }
    #[doc = "0x08 - Function Address Register"]
    #[inline(always)]
    pub const fn faddr(&self) -> &Faddr {
        &self.faddr
    }
    #[doc = "0x10 - Interrupt Enable Register"]
    #[inline(always)]
    pub const fn ier(&self) -> &Ier {
        &self.ier
    }
    #[doc = "0x14 - Interrupt Disable Register"]
    #[inline(always)]
    pub const fn idr(&self) -> &Idr {
        &self.idr
    }
    #[doc = "0x18 - Interrupt Mask Register"]
    #[inline(always)]
    pub const fn imr(&self) -> &Imr {
        &self.imr
    }
    #[doc = "0x1c - Interrupt Status Register"]
    #[inline(always)]
    pub const fn isr(&self) -> &Isr {
        &self.isr
    }
    #[doc = "0x20 - Interrupt Clear Register"]
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    #[doc = "0x28 - Reset Endpoint Register"]
    #[inline(always)]
    pub const fn rst_ep(&self) -> &RstEp {
        &self.rst_ep
    }
    #[doc = "0x30 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub const fn isochronous_csr0_isochronous(&self) -> &IsochronousCsr0Isochronous {
        unsafe { &*(self as *const Self).cast::<u8>().add(48).cast() }
    }
    #[doc = "0x30..0x48 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub const fn csr(&self, n: usize) -> &Csr {
        #[allow(clippy::no_effect)]
        [(); 6][n];
        unsafe { &*(self as *const Self).cast::<u8>().add(48).add(4 * n).cast() }
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x30..0x48 - Endpoint Control and Status Register"]
    #[inline(always)]
    pub fn csr_iter(&self) -> impl Iterator<Item = &Csr> {
        (0..6)
            .map(move |n| unsafe { &*(self as *const Self).cast::<u8>().add(48).add(4 * n).cast() })
    }
    #[doc = "0x50..0x68 - Endpoint FIFO Data Register"]
    #[inline(always)]
    pub const fn fdr(&self, n: usize) -> &Fdr {
        &self.fdr[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x50..0x68 - Endpoint FIFO Data Register"]
    #[inline(always)]
    pub fn fdr_iter(&self) -> impl Iterator<Item = &Fdr> {
        self.fdr.iter()
    }
    #[doc = "0x74 - Transceiver Control Register"]
    #[inline(always)]
    pub const fn txvc(&self) -> &Txvc {
        &self.txvc
    }
}
#[doc = "FRM_NUM (r) register accessor: Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`frm_num::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@frm_num`]
module"]
#[doc(alias = "FRM_NUM")]
pub type FrmNum = crate::Reg<frm_num::FrmNumSpec>;
#[doc = "Frame Number Register"]
pub mod frm_num;
#[doc = "GLB_STAT (rw) register accessor: Global State Register\n\nYou can [`read`](crate::Reg::read) this register and get [`glb_stat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`glb_stat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@glb_stat`]
module"]
#[doc(alias = "GLB_STAT")]
pub type GlbStat = crate::Reg<glb_stat::GlbStatSpec>;
#[doc = "Global State Register"]
pub mod glb_stat;
#[doc = "FADDR (rw) register accessor: Function Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`faddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@faddr`]
module"]
#[doc(alias = "FADDR")]
pub type Faddr = crate::Reg<faddr::FaddrSpec>;
#[doc = "Function Address Register"]
pub mod faddr;
#[doc = "IER (w) register accessor: Interrupt Enable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@ier`]
module"]
#[doc(alias = "IER")]
pub type Ier = crate::Reg<ier::IerSpec>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "IDR (w) register accessor: Interrupt Disable Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`idr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@idr`]
module"]
#[doc(alias = "IDR")]
pub type Idr = crate::Reg<idr::IdrSpec>;
#[doc = "Interrupt Disable Register"]
pub mod idr;
#[doc = "IMR (r) register accessor: Interrupt Mask Register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@imr`]
module"]
#[doc(alias = "IMR")]
pub type Imr = crate::Reg<imr::ImrSpec>;
#[doc = "Interrupt Mask Register"]
pub mod imr;
#[doc = "ISR (r) register accessor: Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isr`]
module"]
#[doc(alias = "ISR")]
pub type Isr = crate::Reg<isr::IsrSpec>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR (w) register accessor: Interrupt Clear Register\n\nYou can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@icr`]
module"]
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "RST_EP (rw) register accessor: Reset Endpoint Register\n\nYou can [`read`](crate::Reg::read) this register and get [`rst_ep::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rst_ep::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@rst_ep`]
module"]
#[doc(alias = "RST_EP")]
pub type RstEp = crate::Reg<rst_ep::RstEpSpec>;
#[doc = "Reset Endpoint Register"]
pub mod rst_ep;
#[doc = "CSR (rw) register accessor: Endpoint Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`csr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@csr`]
module"]
#[doc(alias = "CSR")]
pub type Csr = crate::Reg<csr::CsrSpec>;
#[doc = "Endpoint Control and Status Register"]
pub mod csr;
#[doc = "ISOCHRONOUS_CSR0_ISOCHRONOUS (rw) register accessor: Endpoint Control and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isochronous_csr0_isochronous::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isochronous_csr0_isochronous::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@isochronous_csr0_isochronous`]
module"]
#[doc(alias = "ISOCHRONOUS_CSR0_ISOCHRONOUS")]
pub type IsochronousCsr0Isochronous =
    crate::Reg<isochronous_csr0_isochronous::IsochronousCsr0IsochronousSpec>;
#[doc = "Endpoint Control and Status Register"]
pub mod isochronous_csr0_isochronous;
#[doc = "FDR (rw) register accessor: Endpoint FIFO Data Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdr::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@fdr`]
module"]
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
#[doc = "Endpoint FIFO Data Register"]
pub mod fdr;
#[doc = "TXVC (rw) register accessor: Transceiver Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`txvc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txvc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@txvc`]
module"]
#[doc(alias = "TXVC")]
pub type Txvc = crate::Reg<txvc::TxvcSpec>;
#[doc = "Transceiver Control Register"]
pub mod txvc;
