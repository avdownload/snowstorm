use serde::Serialize;

#[derive(Debug, Serialize, serde::Deserialize)]
pub struct Song {
    pub location: String,
    pub metadata: Metadata,
}
#[derive(Debug, Serialize, serde::Deserialize)]
pub struct Metadata {
    /// You must have a name, album and artist. Put them as sensible defaults.
    pub name: String,
    pub album: String,
    pub artist: String,
}
