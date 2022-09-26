use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub(crate) version: VersionInfo,
    pub(crate) players: Players,
    pub(crate) favicon: String,
    pub(crate) description: TextComponent
}

#[derive(Serialize, Deserialize)]
pub struct VersionInfo {
    pub(crate) name: String,
    pub(crate) protocol: u32
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub(crate) max: u32,
    pub(crate) online: u32,
    pub(crate) sample: Vec<SamplePlayer>
}

#[derive(Serialize, Deserialize)]
pub struct TextComponent {
    pub(crate) text: String,
    pub(crate) color: String,
    pub(crate) underlined: bool,
    pub(crate) bold: bool,
    pub(crate) italic: bool,
    pub(crate) strikethrough: bool,
}

#[derive(Serialize, Deserialize)]
pub struct SamplePlayer {
    pub(crate) name: String,
    pub(crate) id: String
}
