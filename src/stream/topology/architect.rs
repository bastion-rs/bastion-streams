use crate::stream::stage::demand::{Demand};

use crate::stream::stage::graph::GraphStage;
use crate::stream::stage::shape::ShapeType;
use multiqueue2::{broadcast_queue, BroadcastReceiver, BroadcastSender};
use crate::stream::topology::container::Container;

pub struct Architect {
    demand_tx: BroadcastSender<Demand>,
    demand_rx: BroadcastReceiver<Demand>,

    stages: Vec<Container>
}


impl Architect {
    pub fn graph(stages: Vec<Container>) -> Architect {
        let stage_count = stages.len() * 2;
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

    pub fn check_bounds(&self) {
        if let Some(root) = self.stages.first() {
            use Container::*;

            match root {
                Transform(_) | Sink(_) => {
                    panic!("Stage traversal failed. Stream graphs start with a Source.")
                },
                _ => ()
            }
        }
    }

    pub fn visit_stages(&mut self) {
        let tx = self.demand_tx.clone();
        let rx = self.demand_rx.add_stream();

        self.stages.iter_mut().for_each(|stage| {
            use Container::*;

            match stage {
                Source(s) | Transform(s) | Sink(s) =>
                    s.build_demand(tx.clone(), rx.clone())
            }
        });
    }
}
