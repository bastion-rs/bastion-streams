use crate::stream::stage::demand::{Demand};

use crate::stream::stage::graph::GraphStage;
use crate::stream::stage::shape::ShapeType;
use multiqueue::{broadcast_queue, BroadcastReceiver, BroadcastSender};

pub struct Architect<'a> {
    demand_tx: BroadcastSender<Demand>,
    demand_rx: BroadcastReceiver<Demand>,

    stages: Vec<Box<dyn GraphStage<'a>>>
}


impl<'a> Architect<'a> {
    pub fn graph(stages: Vec<Box<dyn GraphStage<'static>>>) -> Architect {
        let stage_count = stages.len();
        let (demand_tx, demand_rx) =
            broadcast_queue(stage_count as u64);

        Architect {
            demand_rx,
            demand_tx,
            stages
        }
    }

    pub fn run() {
        unimplemented!()
    }

    fn check_bounds(&'a self) {
        if let Some(root) = self.stages.first() {
            if root.get_shape() != ShapeType::Source {
                unimplemented!()
            }
        }
    }

    fn visit_stages(&'a mut self) {
        let tx = self.demand_tx.clone();
        let rx = self.demand_rx.add_stream();

        self.stages.iter_mut().for_each(|stage| {
            stage.build_demand(tx.clone(), rx.clone())
        });
    }
}
