use serde::Deserialize;
use std::future::Future;
use std::pin::Pin;
use warp::hyper::Response;
use warp::Filter;

use super::proto::packet;
use super::{Capability, Ref};

pub trait InfoRefs {
    type Error: std::fmt::Debug;

    /// Should return the capabilities of this implementation of
    /// `git-receive-pack`.
    fn capabilities(&self) -> Vec<Capability>;

    /// Should return the refs that the server already has. For each branch or
    /// tag, this should be the most recent ref.
    fn refs(&self) -> Pin<Box<dyn Future<Output = Result<Vec<Ref>, Self::Error>> + Send>>;
}

fn build_capabilities(capabilities: &[Capability]) -> String {
    let mut result = vec![];

    for capability in capabilities {
        result.push(match capability {
            Capability::SideBand64k => "side-band-64k",
            Capability::ReportStatus => "report-status",
            cap => todo!("Capability {:?} has not yet been implemented.", cap),
        })
    }

    result.join(" ")
}

pub fn info_refs<F, T>(
    filter: F,
) -> impl Filter<Extract = (warp::hyper::Response<warp::hyper::Body>,), Error = warp::Rejection>
       + Clone
       + Send
       + Sync
       + 'static
where
    F: Filter<Extract = (T,), Error = warp::Rejection> + Clone + Send + Sync + 'static,
    T: InfoRefs + Clone + Send + Sync + 'static,
{
    #[derive(Copy, Clone, Debug, Deserialize)]
    enum Service {
        #[serde(rename = "git-receive-pack")]
        GitReceivePack,
    }

    #[derive(Copy, Clone, Debug, Deserialize)]
    struct Request {
        service: Service,
    }

    filter
        .and(warp::query::<Request>())
        .and_then(|repo: T, _request| async move {
            let mut refs = repo.refs().await.unwrap();

            if refs.is_empty() {
                refs.push(Ref {
                    hash: [0u8; 20],
                    name: "capabilities^{}".to_owned(),
                })
            }

            let body = refs
                .into_iter()
                .enumerate()
                .map(|(i, reference)| {
                    let mut line = hex::encode(reference.hash) + " " + &reference.name;

                    if i == 0 {
                        line = line + "\0" + &build_capabilities(&repo.capabilities());
                    }

                    packet(line + "\n")
                })
                .collect::<Vec<_>>()
                .join("");

            Ok::<_, warp::Rejection>(
                Response::builder()
                    .header(
                        "Content-Type",
                        "application/x-git-receive-pack-advertisement",
                    )
                    .header("Cache-Control", "no-cache")
                    .body(warp::hyper::Body::from(
                        packet("# service=git-receive-pack\n")
                            + "0000"
                            + &packet("version 1")
                            + &body
                            + "0000",
                    ))
                    .unwrap(),
            )
        })
}
