use crossbeam_channel::{Sender, Receiver};

#[derive(Clone, Debug)]
pub enum DemandStyle {
    DemandFull(usize),
    DemandPartial(usize, usize)
}

#[derive(Clone, Debug)]
pub struct Demand {
    pub stage_id: usize,
    pub style: DemandStyle
}

// Demand endpoint struct
#[derive(Clone, Debug)]
pub struct Demander {
    pub tx: Sender<Demand>,
    pub rx: Receiver<Demand>
}
