
use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;
use std::mem::MaybeUninit;


pub struct Single<O> {
    pub shape: SourceShape<'static, O>,

    pub element: O,

    pub demand_rx: BroadcastReceiver<Demand>,
    pub demand_tx: BroadcastSender<Demand>,

    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

impl<O> Single<O>
where
    O: Clone
{
    pub fn new(element: O) -> Self {
        Self {
            shape: unsafe { MaybeUninit::uninit().assume_init() },

            element,

            demand_rx: unsafe { MaybeUninit::uninit().assume_init() },
            demand_tx: unsafe { MaybeUninit::uninit().assume_init() },

            in_handler: unsafe { MaybeUninit::uninit().assume_init() },
            out_handler: unsafe { MaybeUninit::uninit().assume_init() },
            logic: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

#[derive(Clone, Debug)]
struct SingleHandler<O> {
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
    O: Clone +  'static,
{
    fn build_shape(&mut self) {
        let single_source_outlet = Outlet::<O>::new(0, "Single.out");
        self.shape = SourceShape {
            outlet: single_source_outlet,
        };
    }

    fn build_demand(&mut self, tx: BroadcastSender<Demand>, rx: BroadcastReceiver<Demand>) {
        self.demand_tx = tx;
        self.demand_rx = rx;
    }

    fn create_logic(&mut self, _attributes: Attributes) -> GraphStageLogic {
        self.build_shape();

        let (tx, rx) = unbounded();

        self.out_handler = Box::new(SingleHandler {
            elem: self.element.clone(),
            tx,
            rx,
        });

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<NotUsed, O>(shape);
        gsl.set_outlet_handler(self.shape.outlet.clone(), self.out_handler.clone());
        self.logic = gsl.clone();
        gsl
    }

    fn get_shape(&self) -> ShapeType {
        let shape: &dyn Shape<NotUsed, O> = &self.shape;
        shape.shape_type()
    }
}
