use crate::stream::stage::lets::{Inlet, Outlet};

#[derive(PartialEq)]
pub enum ShapeType {
    Source,
    Flow,
    Sink,
}

pub trait Shape<'a, I, O> {
    fn shape_type(&self) -> ShapeType;
    fn inlets(&self) -> Vec<Inlet<'a, I>>;
    fn outlets(&self) -> Vec<Outlet<'a, O>>;
}

////////////////
// Source Shape
////////////////
#[derive(Clone)]
pub struct SourceShape<'a, O> {
    pub outlet: Outlet<'a, O>,
}

impl<'a, O> SourceShape<'a, O>
where
    O: Clone,
{
    pub fn new_from(outlet: Outlet<'a, O>) -> Box<Self> {
        Box::new(SourceShape { outlet })
    }
}

impl<'a, I, O> Shape<'a, I, O> for SourceShape<'a, O>
where
    O: Clone,
{
    fn shape_type(&self) -> ShapeType {
        ShapeType::Source
    }

    fn inlets(&self) -> Vec<Inlet<'a, I>> {
        Vec::new()
    }

    fn outlets(&self) -> Vec<Outlet<'a, O>> {
        vec![self.outlet.clone()]
    }
}

////////////////
// Flow Shape
////////////////
#[derive(Clone)]
pub struct FlowShape<'a, I, O> {
    pub inlet: Inlet<'a, I>,
    pub outlet: Outlet<'a, O>,
}

impl<'a, I, O> FlowShape<'a, I, O>
where
    I: Clone,
    O: Clone,
{
    pub fn new_from(inlet: Inlet<'a, I>, outlet: Outlet<'a, O>) -> Box<Self> {
        Box::new(FlowShape { inlet, outlet })
    }
}

impl<'a, I, O> Shape<'a, I, O> for FlowShape<'a, I, O>
where
    I: Clone,
    O: Clone,
{
    fn shape_type(&self) -> ShapeType {
        ShapeType::Flow
    }

    fn inlets(&self) -> Vec<Inlet<'a, I>> {
        vec![self.inlet.clone()]
    }

    fn outlets(&self) -> Vec<Outlet<'a, O>> {
        vec![self.outlet.clone()]
    }
}

////////////////
// Sink Shape
////////////////
#[derive(Clone)]
pub struct SinkShape<'a, I> {
    pub inlet: Inlet<'a, I>,
}

impl<'a, I> SinkShape<'a, I>
where
    I: Clone,
{
    pub fn new_from(inlet: Inlet<'a, I>) -> Box<Self> {
        Box::new(SinkShape { inlet })
    }
}

impl<'a, I, O> Shape<'a, I, O> for SinkShape<'a, I>
where
    I: Clone,
{
    fn shape_type(&self) -> ShapeType {
        ShapeType::Sink
    }

    fn inlets(&self) -> Vec<Inlet<'a, I>> {
        vec![self.inlet.clone()]
    }

    fn outlets(&self) -> Vec<Outlet<'a, O>> {
        Vec::new()
    }
}
