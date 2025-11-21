use crate::hkt::{Foldable, Hkt, one_of::OneOf5Hkt};

pub trait FoldableExt<'t, ReqIn: Hkt<'t>, ReqOut: Hkt<'t>, ReqF1: OneOf5Hkt<'t>> : Foldable<'t, ReqIn, ReqOut, ReqF1> {
    
}

impl<'t, ReqIn: Hkt<'t>, ReqOut: Hkt<'t>, ReqF1: OneOf5Hkt<'t>, T: Foldable<'t, ReqIn, ReqOut, ReqF1> {
    
}
