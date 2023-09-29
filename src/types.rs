use crate::error::ButlerError;
pub type ButlerResult<T> = Result<T, ButlerError>;

#[derive(Clone, Debug)]
pub enum QueryType {
    Keywords(String),
    KeywordList(Vec<String>),
    VideoLink(String),
    PlaylistLink(String),
}