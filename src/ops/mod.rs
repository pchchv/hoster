use blake3;
use iron::method;
use cidr::IpCidr;
use iron::mime::Mime;
use std::sync::RwLock;
use std::ffi::OsString;
use std::path::PathBuf;
use iron::headers::Encoding;
use std::sync::atomic::AtomicU64;
use std::collections::{HashMap, BTreeMap};


mod webdav;


type CacheT<Cnt> = HashMap<(blake3::Hash, Encoding), (Cnt, AtomicU64)>;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum WebDavLevel {
    No,
    MkColMoveOnly,
    All,
}

pub struct HttpHandler {
    pub hosted_directory: (String, PathBuf),
    pub follow_symlinks: bool,
    pub sandbox_symlinks: bool,
    pub generate_listings: bool,
    pub check_indices: bool,
    pub strip_extensions: bool,
    pub log: (bool, bool, bool),
    pub webdav: WebDavLevel,
    pub global_auth_data: Option<(String, Option<String>)>,
    pub path_auth_data: BTreeMap<String, Option<(String, Option<String>)>>,
    pub writes_temp_dir: Option<(String, PathBuf)>,
    pub encoded_temp_dir: Option<(String, PathBuf)>,
    pub proxies: BTreeMap<IpCidr, String>,
    pub proxy_redirs: BTreeMap<IpCidr, String>,
    pub mime_type_overrides: BTreeMap<OsString, Mime>,
    pub additional_headers: Vec<(String, Vec<u8>)>,
    pub cache_gen: RwLock<CacheT<Vec<u8>>>,
    pub cache_fs_files: RwLock<HashMap<String, blake3::Hash>>, // etag -> cache key
    pub cache_fs: RwLock<CacheT<(PathBuf, bool, u64)>>,
    pub cache_gen_size: AtomicU64,
    pub cache_fs_size: AtomicU64,
    pub encoded_filesystem_limit: u64,
    pub encoded_generated_limit: u64,
    pub allowed_methods: &'static [method::Method],
}