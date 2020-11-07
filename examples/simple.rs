use futures::{Stream, StreamExt};
use giraffe::{Capability, CommandStatus, FilterExt, InfoRefs, Multiplexed, ReceivePack, Status};
use std::pin::Pin;
use warp::Filter;

#[derive(Clone)]
pub struct Repo;

impl InfoRefs for Repo {
    type Error = ();

    fn capabilities(&self) -> Vec<Capability> {
        vec![Capability::SideBand64k, Capability::ReportStatus]
    }

    fn refs(
        &self,
    ) -> std::pin::Pin<
        Box<dyn futures::Future<Output = Result<Vec<giraffe::Ref>, Self::Error>> + Send>,
    > {
        Box::pin(futures::future::ready(Ok(vec![])))
    }
}

impl ReceivePack for Repo {
    fn receive<'a, S, B>(
        &self,
        stream: S,
    ) -> Pin<Box<dyn Stream<Item = Multiplexed<Status<Self::Error>>> + Send>>
    where
        S: Stream<Item = Result<B, warp::Error>> + Send + 'static,
        B: warp::Buf,
    {
        Box::pin(
            stream
                .filter_map(|_| futures::future::ready(None))
                .chain(futures::stream::iter(vec![
                    Multiplexed::Pack(Status {
                        unpack_error: None,
                        commands: vec![CommandStatus::Ok("refs/heads/main".to_owned())],
                    }),
                    Multiplexed::Progress("This is an example message!".to_owned()),
                ])),
        )
    }
}

#[tokio::main]
async fn main() {
    let info_refs = warp::path!("info" / "refs").map(|| Repo).with_info_refs();

    let git_receive_pack = warp::path!("git-receive-pack")
        .map(|| Repo)
        .with_receive_pack();

    warp::serve(info_refs.or(git_receive_pack))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
