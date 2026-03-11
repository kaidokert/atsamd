#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    hcrevision: Hcrevision,
    hccontrol: Hccontrol,
    hccommandstatus: Hccommandstatus,
    hcinterruptstatus: Hcinterruptstatus,
    hcinterruptenable: Hcinterruptenable,
    hcinterruptdisable: Hcinterruptdisable,
    hchcca: Hchcca,
    hcperiodcurrented: Hcperiodcurrented,
    hccontrolheaded: Hccontrolheaded,
    hccontrolcurrented: Hccontrolcurrented,
    hcbulkheaded: Hcbulkheaded,
    hcbulkcurrented: Hcbulkcurrented,
    hcdonehead: Hcdonehead,
    hcfminterval: Hcfminterval,
    hcfmremaining: Hcfmremaining,
    hcfmnumber: Hcfmnumber,
    hcperiodicstart: Hcperiodicstart,
    hclsthreshold: Hclsthreshold,
    hcrhdescriptora: Hcrhdescriptora,
    hcrhdescriptorb: Hcrhdescriptorb,
    hcrhstatus: Hcrhstatus,
    hcrhportstatus: [Hcrhportstatus; 2],
}
impl RegisterBlock {
    #[doc = "0x00 - OHCI Revision Number Register"]
    #[inline(always)]
    pub const fn hcrevision(&self) -> &Hcrevision {
        &self.hcrevision
    }
    #[doc = "0x04 - HC Operating Mode Register"]
    #[inline(always)]
    pub const fn hccontrol(&self) -> &Hccontrol {
        &self.hccontrol
    }
    #[doc = "0x08 - HC Command and Status Register"]
    #[inline(always)]
    pub const fn hccommandstatus(&self) -> &Hccommandstatus {
        &self.hccommandstatus
    }
    #[doc = "0x0c - HC Interrupt and Status Register"]
    #[inline(always)]
    pub const fn hcinterruptstatus(&self) -> &Hcinterruptstatus {
        &self.hcinterruptstatus
    }
    #[doc = "0x10 - HC Interrupt Enable Register"]
    #[inline(always)]
    pub const fn hcinterruptenable(&self) -> &Hcinterruptenable {
        &self.hcinterruptenable
    }
    #[doc = "0x14 - HC Interrupt Disable Register"]
    #[inline(always)]
    pub const fn hcinterruptdisable(&self) -> &Hcinterruptdisable {
        &self.hcinterruptdisable
    }
    #[doc = "0x18 - HC HCCA Address Register"]
    #[inline(always)]
    pub const fn hchcca(&self) -> &Hchcca {
        &self.hchcca
    }
    #[doc = "0x1c - HC Current Periodic Register"]
    #[inline(always)]
    pub const fn hcperiodcurrented(&self) -> &Hcperiodcurrented {
        &self.hcperiodcurrented
    }
    #[doc = "0x20 - HC Head Control Register"]
    #[inline(always)]
    pub const fn hccontrolheaded(&self) -> &Hccontrolheaded {
        &self.hccontrolheaded
    }
    #[doc = "0x24 - HC Current Control Register"]
    #[inline(always)]
    pub const fn hccontrolcurrented(&self) -> &Hccontrolcurrented {
        &self.hccontrolcurrented
    }
    #[doc = "0x28 - HC Head Bulk Register"]
    #[inline(always)]
    pub const fn hcbulkheaded(&self) -> &Hcbulkheaded {
        &self.hcbulkheaded
    }
    #[doc = "0x2c - HC Current Bulk Register"]
    #[inline(always)]
    pub const fn hcbulkcurrented(&self) -> &Hcbulkcurrented {
        &self.hcbulkcurrented
    }
    #[doc = "0x30 - HC Head Done Register"]
    #[inline(always)]
    pub const fn hcdonehead(&self) -> &Hcdonehead {
        &self.hcdonehead
    }
    #[doc = "0x34 - HC Frame Interval Register"]
    #[inline(always)]
    pub const fn hcfminterval(&self) -> &Hcfminterval {
        &self.hcfminterval
    }
    #[doc = "0x38 - HC Frame Remaining Register"]
    #[inline(always)]
    pub const fn hcfmremaining(&self) -> &Hcfmremaining {
        &self.hcfmremaining
    }
    #[doc = "0x3c - HC Frame Number Register"]
    #[inline(always)]
    pub const fn hcfmnumber(&self) -> &Hcfmnumber {
        &self.hcfmnumber
    }
    #[doc = "0x40 - HC Periodic Start Register"]
    #[inline(always)]
    pub const fn hcperiodicstart(&self) -> &Hcperiodicstart {
        &self.hcperiodicstart
    }
    #[doc = "0x44 - HC Low-Speed Threshold Register"]
    #[inline(always)]
    pub const fn hclsthreshold(&self) -> &Hclsthreshold {
        &self.hclsthreshold
    }
    #[doc = "0x48 - HC Root Hub A Register"]
    #[inline(always)]
    pub const fn hcrhdescriptora(&self) -> &Hcrhdescriptora {
        &self.hcrhdescriptora
    }
    #[doc = "0x4c - HC Root Hub B Register"]
    #[inline(always)]
    pub const fn hcrhdescriptorb(&self) -> &Hcrhdescriptorb {
        &self.hcrhdescriptorb
    }
    #[doc = "0x50 - HC Root Hub Status Register"]
    #[inline(always)]
    pub const fn hcrhstatus(&self) -> &Hcrhstatus {
        &self.hcrhstatus
    }
    #[doc = "0x54..0x5c - HC Port 1 Status and Control Register"]
    #[inline(always)]
    pub const fn hcrhportstatus(&self, n: usize) -> &Hcrhportstatus {
        &self.hcrhportstatus[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x54..0x5c - HC Port 1 Status and Control Register"]
    #[inline(always)]
    pub fn hcrhportstatus_iter(&self) -> impl Iterator<Item = &Hcrhportstatus> {
        self.hcrhportstatus.iter()
    }
}
#[doc = "HCREVISION (r) register accessor: OHCI Revision Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrevision::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrevision`]
module"]
#[doc(alias = "HCREVISION")]
pub type Hcrevision = crate::Reg<hcrevision::HcrevisionSpec>;
#[doc = "OHCI Revision Number Register"]
pub mod hcrevision;
#[doc = "HCCONTROL (rw) register accessor: HC Operating Mode Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccontrol`]
module"]
#[doc(alias = "HCCONTROL")]
pub type Hccontrol = crate::Reg<hccontrol::HccontrolSpec>;
#[doc = "HC Operating Mode Register"]
pub mod hccontrol;
#[doc = "HCCOMMANDSTATUS (rw) register accessor: HC Command and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccommandstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccommandstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccommandstatus`]
module"]
#[doc(alias = "HCCOMMANDSTATUS")]
pub type Hccommandstatus = crate::Reg<hccommandstatus::HccommandstatusSpec>;
#[doc = "HC Command and Status Register"]
pub mod hccommandstatus;
#[doc = "HCINTERRUPTSTATUS (rw) register accessor: HC Interrupt and Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcinterruptstatus`]
module"]
#[doc(alias = "HCINTERRUPTSTATUS")]
pub type Hcinterruptstatus = crate::Reg<hcinterruptstatus::HcinterruptstatusSpec>;
#[doc = "HC Interrupt and Status Register"]
pub mod hcinterruptstatus;
#[doc = "HCINTERRUPTENABLE (rw) register accessor: HC Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptenable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptenable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcinterruptenable`]
module"]
#[doc(alias = "HCINTERRUPTENABLE")]
pub type Hcinterruptenable = crate::Reg<hcinterruptenable::HcinterruptenableSpec>;
#[doc = "HC Interrupt Enable Register"]
pub mod hcinterruptenable;
#[doc = "HCINTERRUPTDISABLE (rw) register accessor: HC Interrupt Disable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcinterruptdisable::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcinterruptdisable::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcinterruptdisable`]
module"]
#[doc(alias = "HCINTERRUPTDISABLE")]
pub type Hcinterruptdisable = crate::Reg<hcinterruptdisable::HcinterruptdisableSpec>;
#[doc = "HC Interrupt Disable Register"]
pub mod hcinterruptdisable;
#[doc = "HCHCCA (rw) register accessor: HC HCCA Address Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hchcca::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hchcca::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hchcca`]
module"]
#[doc(alias = "HCHCCA")]
pub type Hchcca = crate::Reg<hchcca::HchccaSpec>;
#[doc = "HC HCCA Address Register"]
pub mod hchcca;
#[doc = "HCPERIODCURRENTED (r) register accessor: HC Current Periodic Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodcurrented::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcperiodcurrented`]
module"]
#[doc(alias = "HCPERIODCURRENTED")]
pub type Hcperiodcurrented = crate::Reg<hcperiodcurrented::HcperiodcurrentedSpec>;
#[doc = "HC Current Periodic Register"]
pub mod hcperiodcurrented;
#[doc = "HCCONTROLHEADED (rw) register accessor: HC Head Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolheaded::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolheaded::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccontrolheaded`]
module"]
#[doc(alias = "HCCONTROLHEADED")]
pub type Hccontrolheaded = crate::Reg<hccontrolheaded::HccontrolheadedSpec>;
#[doc = "HC Head Control Register"]
pub mod hccontrolheaded;
#[doc = "HCCONTROLCURRENTED (rw) register accessor: HC Current Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hccontrolcurrented::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hccontrolcurrented::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hccontrolcurrented`]
module"]
#[doc(alias = "HCCONTROLCURRENTED")]
pub type Hccontrolcurrented = crate::Reg<hccontrolcurrented::HccontrolcurrentedSpec>;
#[doc = "HC Current Control Register"]
pub mod hccontrolcurrented;
#[doc = "HCBULKHEADED (rw) register accessor: HC Head Bulk Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkheaded::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkheaded::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbulkheaded`]
module"]
#[doc(alias = "HCBULKHEADED")]
pub type Hcbulkheaded = crate::Reg<hcbulkheaded::HcbulkheadedSpec>;
#[doc = "HC Head Bulk Register"]
pub mod hcbulkheaded;
#[doc = "HCBULKCURRENTED (rw) register accessor: HC Current Bulk Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcbulkcurrented::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcbulkcurrented::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcbulkcurrented`]
module"]
#[doc(alias = "HCBULKCURRENTED")]
pub type Hcbulkcurrented = crate::Reg<hcbulkcurrented::HcbulkcurrentedSpec>;
#[doc = "HC Current Bulk Register"]
pub mod hcbulkcurrented;
#[doc = "HCDONEHEAD (r) register accessor: HC Head Done Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcdonehead::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcdonehead`]
module"]
#[doc(alias = "HCDONEHEAD")]
pub type Hcdonehead = crate::Reg<hcdonehead::HcdoneheadSpec>;
#[doc = "HC Head Done Register"]
pub mod hcdonehead;
#[doc = "HCFMINTERVAL (rw) register accessor: HC Frame Interval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfminterval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcfminterval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfminterval`]
module"]
#[doc(alias = "HCFMINTERVAL")]
pub type Hcfminterval = crate::Reg<hcfminterval::HcfmintervalSpec>;
#[doc = "HC Frame Interval Register"]
pub mod hcfminterval;
#[doc = "HCFMREMAINING (r) register accessor: HC Frame Remaining Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmremaining::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfmremaining`]
module"]
#[doc(alias = "HCFMREMAINING")]
pub type Hcfmremaining = crate::Reg<hcfmremaining::HcfmremainingSpec>;
#[doc = "HC Frame Remaining Register"]
pub mod hcfmremaining;
#[doc = "HCFMNUMBER (r) register accessor: HC Frame Number Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcfmnumber::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcfmnumber`]
module"]
#[doc(alias = "HCFMNUMBER")]
pub type Hcfmnumber = crate::Reg<hcfmnumber::HcfmnumberSpec>;
#[doc = "HC Frame Number Register"]
pub mod hcfmnumber;
#[doc = "HCPERIODICSTART (rw) register accessor: HC Periodic Start Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcperiodicstart::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcperiodicstart::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcperiodicstart`]
module"]
#[doc(alias = "HCPERIODICSTART")]
pub type Hcperiodicstart = crate::Reg<hcperiodicstart::HcperiodicstartSpec>;
#[doc = "HC Periodic Start Register"]
pub mod hcperiodicstart;
#[doc = "HCLSTHRESHOLD (rw) register accessor: HC Low-Speed Threshold Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hclsthreshold::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hclsthreshold::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hclsthreshold`]
module"]
#[doc(alias = "HCLSTHRESHOLD")]
pub type Hclsthreshold = crate::Reg<hclsthreshold::HclsthresholdSpec>;
#[doc = "HC Low-Speed Threshold Register"]
pub mod hclsthreshold;
#[doc = "HCRHDESCRIPTORA (rw) register accessor: HC Root Hub A Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhdescriptora::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhdescriptora::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhdescriptora`]
module"]
#[doc(alias = "HCRHDESCRIPTORA")]
pub type Hcrhdescriptora = crate::Reg<hcrhdescriptora::HcrhdescriptoraSpec>;
#[doc = "HC Root Hub A Register"]
pub mod hcrhdescriptora;
#[doc = "HCRHDESCRIPTORB (rw) register accessor: HC Root Hub B Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhdescriptorb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhdescriptorb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhdescriptorb`]
module"]
#[doc(alias = "HCRHDESCRIPTORB")]
pub type Hcrhdescriptorb = crate::Reg<hcrhdescriptorb::HcrhdescriptorbSpec>;
#[doc = "HC Root Hub B Register"]
pub mod hcrhdescriptorb;
#[doc = "HCRHSTATUS (rw) register accessor: HC Root Hub Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhstatus::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhstatus`]
module"]
#[doc(alias = "HCRHSTATUS")]
pub type Hcrhstatus = crate::Reg<hcrhstatus::HcrhstatusSpec>;
#[doc = "HC Root Hub Status Register"]
pub mod hcrhstatus;
#[doc = "HCRHPORTSTATUS (rw) register accessor: HC Port 1 Status and Control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hcrhportstatus::R`]. You can [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hcrhportstatus::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@hcrhportstatus`]
module"]
#[doc(alias = "HCRHPORTSTATUS")]
pub type Hcrhportstatus = crate::Reg<hcrhportstatus::HcrhportstatusSpec>;
#[doc = "HC Port 1 Status and Control Register"]
pub mod hcrhportstatus;
