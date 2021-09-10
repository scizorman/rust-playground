use std::borrow::Cow;
use std::path::{Path, PathBuf};

use rand::{self, Rng};
use rocket::http::uri::fmt;
use rocket::request::FromParam;

/// Table to retrieve base62 values from.
const BASE62: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

/// A _probably_ unique paste ID.
#[derive(UriDisplayPath)]
pub struct PasteID<'a>(Cow<'a, str>);

impl PasteID<'_> {
    /// Generate a _probably_ unique ID with `size` characters. For readability,
    /// the characters used are from the sets [0-9], [A-Z], [a-z]. The
    /// probability of a collision depends on the value of `size` and the number
    /// of IDs generated thus far.
    pub fn new(size: usize) -> PasteID<'static> {
        let mut id = String::with_capacity(size);
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            id.push(BASE62[rng.gen::<usize>() % 62] as char);
        }

        PasteID(Cow::Owned(id))
    }

    pub fn file_path(&self) -> PathBuf {
        let root = concat!(env!("CARGO_MANIFEST_DIR"), "/", "upload");
        Path::new(root).join(self.0.as_ref())
    }
}

/// Returns an instance of `PasteID` if the path segment is a valid ID.
/// Otherwise returns the invalid ID as the `Err` value.
impl<'a> FromParam<'a> for PasteID<'a> {
    type Error = &'a str;

    fn from_param(param: &'a str) -> Result<Self, Self::Error> {
        param
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
            .then(|| PasteID(param.into()))
            .ok_or(param)
    }
}

impl<'a> fmt::FromUriParam<fmt::Path, &'a str> for PasteID<'_> {
    type Target = PasteID<'a>;

    fn from_uri_param(param: &'a str) -> Self::Target {
        PasteID(param.into())
    }
}
