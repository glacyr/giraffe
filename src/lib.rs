mod capability;
mod filter_ext;
mod info_refs;
mod proto;
mod receive_pack;

pub struct Ref {
    pub hash: [u8; 20],
    pub name: String,
}

pub use capability::Capability;
pub use filter_ext::FilterExt;
pub use info_refs::{info_refs, InfoRefs};
pub use proto::{CommandStatus, Multiplexed, Status};
pub use receive_pack::{receive_pack, ReceivePack};
