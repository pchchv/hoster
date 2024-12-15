use blake3;
use std::collections::HashMap;
use std::sync::atomic::AtomicU64;
use iron::headers::Encoding;


mod webdav;


type CacheT<Cnt> = HashMap<(blake3::Hash, Encoding), (Cnt, AtomicU64)>;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum WebDavLevel {
    No,
    MkColMoveOnly,
    All,
}
