use futures::{Stream, StreamExt};
use std::pin::Pin;
use warp::Filter;

use super::proto::packet;
use super::{InfoRefs, Multiplexed, Status};

pub trait ReceivePack: InfoRefs {
    /// Should process an incoming packfile and stream the results.
    fn receive<'a, S, B>(
        &self,
        stream: S,
    ) -> Pin<Box<dyn Stream<Item = Multiplexed<Status<Self::Error>>> + Send>>
    where
        S: Stream<Item = Result<B, warp::Error>> + Send + 'static,
        B: warp::Buf;
}

/// This function returns a new handler that implements the `git-receive-pack`
/// endpoint for types that implement the `ReceivePack` trait.
pub fn receive_pack<F, T>(
    filter: F,
) -> impl Filter<Extract = (warp::hyper::Response<warp::hyper::Body>,), Error = warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static
where
    F: Filter<Extract = (T,), Error = warp::Rejection> + Clone + Send + Sync + 'static,
    T: ReceivePack + Clone + Send + Sync + 'static,
{
    filter.and(warp::body::stream()).map(|repo: T, pack| {
        warp::hyper::Response::builder()
            .header("Content-Type", "application/x-git-receive-pack-result")
            .header("Cache-Control", "no-cache")
            .body(warp::hyper::Body::wrap_stream(
                repo.receive(pack)
                    .map(|message| {
                        Ok::<_, warp::Error>(match message {
                            Multiplexed::Pack(_pack) => packet(
                                "\x01".to_owned()
                                    + &packet("unpack ok\n")
                                    + &packet("ok refs/heads/master\n")
                                    + "0000",
                            ),
                            Multiplexed::Progress(progress) => {
                                packet("\x02".to_owned() + &progress + "\n")
                            }
                            Multiplexed::Fatal(_) => todo!("Fatal messages are not yet supported."),
                        })
                    })
                    .chain(futures::stream::once(futures::future::ready(Ok(
                        "0000".to_owned()
                    )))),
            ))
            .unwrap()
    })
}
