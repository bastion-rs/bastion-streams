use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;

pub struct Ignore<I> {
    pub shape: Option<SinkShape<'static, I>>,
    pub stage_id: Option<usize>,

    pub demand_rx: Option<BroadcastReceiver<Demand>>,
    pub demand_tx: Option<BroadcastSender<Demand>>,

    pub in_handler: Option<Box<dyn InHandler>>,
    pub out_handler: Option<Box<dyn OutHandler>>,
    pub logic: Option<GraphStageLogic>,
}

impl<I> Ignore<I>
where
    I: Clone,
{
    pub fn new() -> Self {
        Self {
            shape: None,
            stage_id: None,

            demand_rx: None,
            demand_tx: None,

            in_handler: None,
            out_handler: None,
            logic: None,
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
            println!("Demanding");
            // todo: handle error case of try_recv
            // todo: on_pull make demand from the upper
            let demand = Demand {
                stage_id: self.stage_id,
                style: DemandStyle::DemandFull(100),
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
    I: Clone + 'static,
{
    fn build_shape(&mut self) {
        let ignore_sink_inlet = Inlet::<I>::new(0, "Sink.out");
        self.shape = Some(SinkShape {
            inlet: ignore_sink_inlet,
        });
    }

    fn build_demand(&mut self, tx: BroadcastSender<Demand>, rx: BroadcastReceiver<Demand>) {
        self.demand_tx = Some(tx);
        self.demand_rx = Some(rx);
    }

    fn create_logic(&mut self, stage_id: usize, _attributes: Attributes) {
        self.build_shape();

        let (tx, rx) = unbounded::<I>();

        self.in_handler = Some(Box::new(IgnoreHandler {
            in_tx: Some(tx),
            in_rx: Some(rx),
            demand_rx: Some(self.demand_rx.as_ref().unwrap().clone()),
            demand_tx: Some(self.demand_tx.as_ref().unwrap().clone()),
            stage_id,
        }));

        self.stage_id = Some(stage_id);

        let shape = Box::new(self.shape.as_ref().unwrap().clone());

        let mut gsl = GraphStageLogic::from_shape::<I, NotUsed>(shape);
        gsl.set_inlet_handler(
            self.shape.as_ref().unwrap().inlet.clone(),
            self.in_handler.as_ref().unwrap().clone(),
        );
        self.logic = Some(gsl);
    }

    fn get_shape(&self) -> ShapeType {
        let shape: &dyn Shape<I, NotUsed> = self.shape.as_ref().unwrap();
        shape.shape_type()
    }

    fn get_stage_id(&self) -> usize {
        self.stage_id.unwrap()
    }

    fn get_logic(&self) -> &GraphStageLogic {
        self.logic.as_ref().unwrap()
    }
}
