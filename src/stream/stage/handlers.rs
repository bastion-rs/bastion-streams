use futures::io;
use objekt_clonable::*;

//#[clonable]
//pub trait Handler: Clone {}
////impl<T> Handler for T where T: Clone {}
//impl Debug for dyn Handler {
//    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
//        write!(f, "{:?}", self.type_id())
//    }
//}

#[clonable]
pub trait InHandler: Clone {
    /**
     * Name of the outlet handler
     */
    fn name(&self) -> String;

    /**
     * Called when the input port has a new element available.
     */
    fn on_push(&self);

    /**
     * Called when the input port is finished. After this callback no other callbacks will be called for this port.
     */
    fn on_upstream_finish(&self);

    /**
     * Called when the input port has failed. After this callback no other callbacks will be called for this port.
     */
    fn on_upstream_failure(&self, err: io::Error);
}

#[clonable]
pub trait OutHandler: Clone {
    /**
     * Name of the outlet handler
     */
    fn name(&self) -> String;

    /**
     * Called when the output port has received a pull, and therefore ready to emit an element, i.e. [[GraphStageLogic.push()]]
     * is now allowed to be called on this port.
     */
    fn on_pull(&self);

    /**
     * Called when the input port is finished. After this callback no other callbacks will be called for this port.
     */
    fn on_downstream_finish(&self);

    /**
     * Called when the output port will no longer accept any new elements. After this callback no other callbacks will
     * be called for this port.
     */
    fn on_downstream_finish_explicit(&self, err: io::Error);
}
