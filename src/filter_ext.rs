use warp::filters::BoxedFilter;
use warp::hyper::{Body, Response};
use warp::Filter;

use super::{info_refs, receive_pack, InfoRefs, ReceivePack};

pub trait UnitTuple {
    type Value;
}

impl<T> UnitTuple for (T,) {
    type Value = T;
}

pub trait FilterExt: Filter {
    fn with_info_refs(self) -> BoxedFilter<(Response<Body>,)>
    where
        Self::Extract: UnitTuple,
        <Self::Extract as UnitTuple>::Value: InfoRefs;

    fn with_receive_pack(self) -> BoxedFilter<(Response<Body>,)>
    where
        Self::Extract: UnitTuple,
        <Self::Extract as UnitTuple>::Value: ReceivePack;
}

impl<F, T> FilterExt for F
where
    F: Filter<Extract = (T,), Error = warp::Rejection> + Clone + Send + Sync + 'static,
    T: Clone + Send + Sync + 'static,
{
    fn with_info_refs(self) -> BoxedFilter<(Response<Body>,)>
    where
        T: InfoRefs,
    {
        self.with(warp::wrap_fn(info_refs)).boxed()
    }

    fn with_receive_pack(self) -> BoxedFilter<(Response<Body>,)>
    where
        T: ReceivePack,
    {
        self.with(warp::wrap_fn(receive_pack)).boxed()
    }
}
