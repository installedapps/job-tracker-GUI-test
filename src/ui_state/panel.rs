#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActivePanel {
    Jobs,
    Stats,
}

impl ActivePanel {
    pub const fn switch_to(self, panel: ActivePanel) -> ActivePanel {
        panel
    }

    pub const fn shows_stats(self) -> bool {
        matches!(self, ActivePanel::Stats)
    }
}
