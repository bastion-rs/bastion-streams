use crate::stream::stage::graph::GraphStage;

pub enum Container {
    Source(Box<dyn GraphStage>),
    Transform(Box<dyn GraphStage>),
    Sink(Box<dyn GraphStage>),
}
