use crate::stream::stage::prelude::*;
use crossbeam_channel::{unbounded, Receiver, Sender};
use futures::io::Error;
use objekt_clonable::clonable;
use std::mem::MaybeUninit;


#[clonable]
pub trait MapClosure<I, O>: Fn(I) -> O + Clone + Send + Sync + 'static {}
impl<I, O, T> MapClosure<I, O> for T where T: Fn(I) -> O + Clone + Send + Sync + 'static {}

type MapFn<I, O> = Box<dyn MapClosure<I, O>>;


pub struct Map<I, O> {
    pub shape: FlowShape<'static, I, O>,
    pub stage_id: usize,

    pub map_fn: MapFn<I, O>,

    pub demand_rx: BroadcastReceiver<Demand>,
    pub demand_tx: BroadcastSender<Demand>,

    pub in_handler: Box<dyn InHandler>,
    pub out_handler: Box<dyn OutHandler>,
    pub logic: GraphStageLogic,
}

impl<I, O> Map<I, O>
where
    I: Clone,
    O: Clone,
{
    pub fn new(map_fn: MapFn<I, O>) -> Self {
        Self {
            shape: unsafe { MaybeUninit::uninit().assume_init() },
            stage_id: unsafe { MaybeUninit::uninit().assume_init() },

            map_fn,

            demand_rx: unsafe { MaybeUninit::uninit().assume_init() },
            demand_tx: unsafe { MaybeUninit::uninit().assume_init() },

            in_handler: unsafe { MaybeUninit::uninit().assume_init() },
            out_handler: unsafe { MaybeUninit::uninit().assume_init() },
            logic: unsafe { MaybeUninit::uninit().assume_init() },
        }
    }
}

///// Map handler
///////////////////////////
#[derive(Clone)]
struct MapHandler<I, O> {
    map_fn: Option<MapFn<I, O>>,
    pub stage_id: usize,

    pub in_rx: Option<Receiver<I>>,
    pub in_tx: Option<Sender<I>>,

    pub demand_rx: Option<BroadcastReceiver<Demand>>,
    pub demand_tx: Option<BroadcastSender<Demand>>,

    pub out_rx: Option<Receiver<O>>,
    pub out_tx: Option<Sender<O>>,
}

impl<I, O> OutHandler for MapHandler<I, O>
    where
        I: Clone + Send + Sync + 'static,
        O: Clone + Send + Sync + 'static,
{
    fn name(&self) -> String {
        String::from("map-flow-out")
    }

    fn on_pull(&self) {
        unimplemented!()
    }

    fn on_downstream_finish(&self) {
        unimplemented!()
    }

    fn on_downstream_finish_explicit(&self, _err: Error) {
        unimplemented!()
    }
}

impl<I, O> InHandler for MapHandler<I, O>
    where
        I: Clone + Send + Sync,
        O: Clone + Send + Sync,
{
    fn name(&self) -> String {
        String::from("map-flow-in")
    }

    fn on_push(&self) {
        if let Ok(elem) = self.in_rx.as_ref().unwrap().try_recv() {
            let resp: O = self.map_fn.as_ref().unwrap()(elem);
            self.out_tx.as_ref().unwrap().send(resp);
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

impl<I, O> Default for MapHandler<I, O> {
    fn default() -> Self {
        MapHandler {
            map_fn: None,
            stage_id: 0,

            in_rx: None,
            in_tx: None,

            demand_rx: None,
            demand_tx: None,

            out_tx: None,
            out_rx: None,
        }
    }
}

impl<I, O> GraphStage for Map<I, O>
where
    I: Clone + Send + Sync + 'static,
    O: Clone + Send + Sync + 'static,
{
    fn build_shape(&mut self) {
        let map_flow_inlet = Inlet::<I>::new(0, "Map.in");
        let map_flow_outlet = Outlet::<O>::new(0, "Map.out");

        self.shape = FlowShape {
            inlet: map_flow_inlet,
            outlet: map_flow_outlet,
        };
    }

    fn build_demand(&mut self, tx: BroadcastSender<Demand>, rx: BroadcastReceiver<Demand>) {
        self.demand_tx = tx;
        self.demand_rx = rx;
    }

    fn create_logic(&mut self, _attributes: Attributes) -> GraphStageLogic {
        self.build_shape();

        let (in_tx, in_rx) = unbounded::<I>();
        let (out_tx, out_rx) = unbounded::<O>();

        let handler = Box::new(MapHandler {
            map_fn: Some(self.map_fn.clone()),
            in_tx: Some(in_tx),
            in_rx: Some(in_rx),
            out_rx: Some(out_rx),
            out_tx: Some(out_tx),
            demand_rx: Some(self.demand_rx.clone()),
            demand_tx: Some(self.demand_tx.clone()),
            stage_id: self.stage_id
        });

        self.in_handler = handler.clone();
        self.out_handler = handler.clone();

        let shape = Box::new(self.shape.clone());

        let mut gsl = GraphStageLogic::from_shape::<I, O>(shape);
        gsl.set_inlet_handler(self.shape.inlet.clone(), self.in_handler.clone());
        gsl.set_outlet_handler(self.shape.outlet.clone(), self.out_handler.clone());
        self.logic = gsl.clone();
        gsl
    }

    fn get_shape(&self) -> ShapeType {
        self.shape.shape_type()
    }
}
