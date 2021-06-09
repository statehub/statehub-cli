use std::borrow::Cow;

pub(crate) fn last_delimiter(path: &str, sep: char) -> Cow<'_, str> {
    let mut pieces = path.rsplit(sep);
    match pieces.next() {
        Some(p) => p.into(),
        None => path.into(),
    }
}
