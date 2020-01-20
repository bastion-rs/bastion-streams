use crate::stream::stage::demand::{Demand, DemandStyle};

use crate::stream::stage::attributes::Attributes;
use crate::stream::stage::graph::GraphStage;

use crate::stream::topology::container::Container;
use multiqueue2::{broadcast_queue, BroadcastReceiver, BroadcastSender};

pub struct Architect {
    demand_tx: BroadcastSender<Demand>,
    demand_rx: BroadcastReceiver<Demand>,

    stages: Vec<Container>,
}

impl Architect {
    pub fn graph(stages: Vec<Container>) -> Architect {
        let stage_count = stages.len() * 2;
        let (demand_tx, demand_rx) = broadcast_queue(stage_count as u64);

        Architect {
            demand_rx,
            demand_tx,
            stages,
        }
    }

    pub fn run(&mut self) {
        if let Some(Container::Sink(sink)) = self.stages.last() {
            let stage_id = (**sink).get_stage_id();
            let demand = Demand::new(stage_id, DemandStyle::DemandFull(100));
            self.demand_tx.try_send(demand);
            self.stages.iter_mut().rev().for_each(|c| {
                use Container::*;

                match c {
                    Source(s) | Transform(s) | Sink(s) => {
                        for x in (**s).get_logic().in_handlers.iter() {
                            dbg!("on_push");
                            x.on_push();
                        }

                        for x in (**s).get_logic().out_handlers.iter() {
                            dbg!("on_pull");
                            x.on_pull();
                        }
                    }
                }
            });
        } else {
            panic!("Run failed");
        }
    }

    pub fn check_bounds(&self) {
        if let Some(first_stage) = self.stages.first() {
            use Container::*;

            match first_stage {
                Transform(_) | Sink(_) => {
                    panic!("Stage traversal failed. Stream graph starts with a Source.")
                }
                _ => (),
            }
        }

        if let Some(last_stage) = self.stages.last() {
            use Container::*;

            match last_stage {
                Source(_) | Transform(_) => {
                    panic!("Stage traversal failed. Stream graph ends with a Sink.")
                }
                _ => (),
            }
        }
    }

    pub fn visit_stages(&mut self) {
        let tx = self.demand_tx.clone();
        let rx = self.demand_rx.add_stream();

        self.stages.iter_mut().for_each(|stage| {
            use Container::*;

            match stage {
                Source(s) | Transform(s) | Sink(s) => {
                    s.build_demand(tx.clone(), rx.clone());
                }
                _ => (),
            }
        });

        self.stages
            .iter_mut()
            .rev()
            .enumerate()
            .for_each(|(stage_id, stage)| {
                use Container::*;

                match stage {
                    Source(s) | Transform(s) | Sink(s) => {
                        s.create_logic(stage_id, Attributes {});
                    }
                    _ => (),
                }
            });
    }
}
