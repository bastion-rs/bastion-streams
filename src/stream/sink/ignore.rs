use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;
use std::mem::MaybeUninit;

pub struct Ignore<I> {
    pub shape: SinkShape<'static, I>,
    pub stage_id: usize,

    pub demand_rx: BroadcastReceiver<Demand>,
    pub demand_tx: BroadcastSender<Demand>,

    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

impl<I> Ignore<I>
where
    I: Clone
{
    pub fn new() -> Self {
        Self {
            shape: unsafe { MaybeUninit::uninit().assume_init() },
            stage_id: unsafe { MaybeUninit::uninit().assume_init() },

            demand_rx: unsafe { MaybeUninit::uninit().assume_init() },
            demand_tx: unsafe { MaybeUninit::uninit().assume_init() },

            in_handler: unsafe { MaybeUninit::uninit().assume_init() },
            out_handler: unsafe { MaybeUninit::uninit().assume_init() },
            logic: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

#[derive(Clone)]
struct IgnoreHandler<I> {
    pub stage_id: usize,

    pub in_rx: Option<Receiver<I>>,
    pub in_tx: Option<Sender<I>>,

    pub demand_rx: Option<BroadcastReceiver<Demand>>,
    pub demand_tx: Option<BroadcastSender<Demand>>,
}

impl<I> InHandler for IgnoreHandler<I>
    where
        I: Clone + 'static,
{
    fn name(&self) -> String {
        String::from("ignore-sink-in")
    }

    fn on_push(&self) {
        if let Ok(_elem) = self.in_rx.as_ref().unwrap().try_recv() {
            println!("Ignored");
        } else {
            // todo: handle error case of try_recv
            // todo: on_pull make demand from the upper
            let demand = Demand {
                stage_id: self.stage_id,
                style: DemandStyle::DemandFull(100)
            };
            self.demand_tx.as_ref().unwrap().try_send(demand).unwrap();
        }
    }

    fn on_upstream_finish(&self) {
        unimplemented!()
    }

    fn on_upstream_failure(&self, _err: Error) {
        unimplemented!()
    }
}

impl<I> GraphStage for Ignore<I>
    where
        I: Clone +  'static,
{
    fn build_shape(&mut self) {
        let ignore_sink_inlet = Inlet::<I>::new(0, "Sink.out");
        self.shape = SinkShape {
            inlet: ignore_sink_inlet,
        };
    }

    fn build_demand(&mut self, tx: BroadcastSender<Demand>, rx: BroadcastReceiver<Demand>) {
        self.demand_tx = tx;
        self.demand_rx = rx;
    }

    fn create_logic(&mut self, _attributes: Attributes) -> GraphStageLogic {
        self.build_shape();

        let (tx, rx) = unbounded::<I>();

        self.in_handler = Box::new(IgnoreHandler {
            in_tx: Some(tx),
            in_rx: Some(rx),
            demand_rx: Some(self.demand_rx.clone()),
            demand_tx: Some(self.demand_tx.clone()),
            stage_id: self.stage_id
        });

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<I, NotUsed>(shape);
        gsl.set_inlet_handler(self.shape.inlet.clone(), self.in_handler.clone());
        self.logic = gsl.clone();
        gsl
    }

    fn get_shape(&self) -> ShapeType {
        let shape: &dyn Shape<I, NotUsed> = &self.shape;
        shape.shape_type()
    }
}
