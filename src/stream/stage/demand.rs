use crossbeam_channel::{Receiver, Sender};

#[derive(Clone, Debug)]
pub enum DemandStyle {
    DemandFull(usize),
    DemandPartial(usize, usize),
}

#[derive(Clone, Debug)]
pub struct Demand {
    pub stage_id: usize,
    pub style: DemandStyle,
}

impl Demand {
    pub fn new(stage_id: usize, style: DemandStyle) -> Self {
        Demand { stage_id, style }
    }
}

// Demand endpoint struct
#[derive(Clone, Debug)]
pub struct Demander {
    pub tx: Sender<Demand>,
    pub rx: Receiver<Demand>,
}
