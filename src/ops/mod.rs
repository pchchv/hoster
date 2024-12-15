mod webdav;


#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum WebDavLevel {
    No,
    MkColMoveOnly,
    All,
}
