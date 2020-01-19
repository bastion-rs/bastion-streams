//#[macro_export]
//macro_rules! source_stage {
//    ($source_name:ty, $out_type:ty) => {
//        pub struct $source_name<'a, $out_type> {
//            pub shape: Box<dyn Shape<'a, NotUsed, $out_type>>,
//            pub in_handler: Box<dyn InHandler>,
//            pub out_handler: Box<dyn OutHandler>,
//            pub logic: GraphStageLogic
//        }
//    };
//}
