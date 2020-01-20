use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;

pub struct Single<O> {
    pub shape: Option<SourceShape<'static, O>>,
    pub stage_id: Option<usize>,

    pub element: O,

    pub demand_rx: Option<BroadcastReceiver<Demand>>,
    pub demand_tx: Option<BroadcastSender<Demand>>,

    pub in_handler: Option<Box<dyn InHandler>>,
    pub out_handler: Option<Box<dyn OutHandler>>,
    pub logic: Option<GraphStageLogic>,
}

impl<O> Single<O>
where
    O: Clone,
{
    pub fn new(element: O) -> Self {
        Self {
            shape: None,
            stage_id: None,

            element,

            demand_rx: None,
            demand_tx: None,

            in_handler: None,
            out_handler: None,
            logic: None,
        }
    }
}

#[derive(Clone, Debug)]
struct SingleHandler<O> {
    pub stage_id: usize,
    elem: O,
    pub rx: Receiver<O>,
    pub tx: Sender<O>,
}

impl<O> OutHandler for SingleHandler<O>
where
    O: Clone + 'static,
{
    fn name(&self) -> String {
        String::from("single-source-out")
    }

    fn on_pull(&self) {
        self.tx.send(self.elem.clone());
        self.on_downstream_finish();
    }

    fn on_downstream_finish(&self) {
        // TODO: Signal stop to the runtime (architect)
    }

    fn on_downstream_finish_explicit(&self, _err: Error) {
        unimplemented!()
    }
}

impl<O> GraphStage for Single<O>
where
    O: Clone + 'static,
{
    fn build_shape(&mut self) {
        let single_source_outlet = Outlet::<O>::new(0, "Single.out");
        self.shape = Some(SourceShape {
            outlet: single_source_outlet,
        });
    }

    fn build_demand(&mut self, tx: BroadcastSender<Demand>, rx: BroadcastReceiver<Demand>) {
        self.demand_tx = Some(tx);
        self.demand_rx = Some(rx);
    }

    fn create_logic(&mut self, stage_id: usize, _attributes: Attributes) {
        self.build_shape();

        let (tx, rx) = unbounded();

        self.out_handler = Some(Box::new(SingleHandler {
            elem: self.element.clone(),
            tx,
            rx,
            stage_id,
        }));

        self.stage_id = Some(stage_id);

        let shape = Box::new(self.shape.as_ref().unwrap().clone());

        let mut gsl = GraphStageLogic::from_shape::<NotUsed, O>(shape);
        gsl.set_outlet_handler(
            self.shape.as_ref().unwrap().outlet.clone(),
            self.out_handler.as_ref().unwrap().clone(),
        );
        self.logic = Some(gsl);
    }

    fn get_shape(&self) -> ShapeType {
        let shape: &dyn Shape<NotUsed, O> = self.shape.as_ref().unwrap();
        shape.shape_type()
    }

    fn get_stage_id(&self) -> usize {
        self.stage_id.unwrap()
    }

    fn get_logic(&self) -> &GraphStageLogic {
        self.logic.as_ref().unwrap()
    }
}
