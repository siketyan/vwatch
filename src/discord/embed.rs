use serde::Serialize;

#[derive(Serialize)]
pub(crate) enum EmbedType {
    #[serde(rename = "rich")]
    Rich,

    #[serde(rename = "image")]
    Image,

    #[serde(rename = "video")]
    Video,

    #[serde(rename = "gifv")]
    GifV,

    #[serde(rename = "article")]
    Article,

    #[serde(rename = "link")]
    Link,
}

#[derive(Serialize)]
pub(crate) struct EmbedFooter {
    pub(crate) text: Option<String>,
    pub(crate) icon_url: Option<String>,
    pub(crate) proxy_icon_url: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct EmbedThumbnail {
    pub(crate) url: Option<String>,
    pub(crate) proxy_url: Option<String>,
    pub(crate) height: Option<u32>,
    pub(crate) width: Option<u32>,
}

#[derive(Serialize)]
pub(crate) struct EmbedAuthor {
    pub(crate) name: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) icon_url: Option<String>,
    pub(crate) proxy_icon_url: Option<String>,
}

#[derive(Serialize)]
pub(crate) struct Embed {
    #[serde(rename = "type")]
    pub(crate) ty: Option<EmbedType>,
    pub(crate) title: Option<String>,
    pub(crate) description: Option<String>,
    pub(crate) url: Option<String>,
    pub(crate) color: Option<u32>,
    pub(crate) footer: Option<EmbedFooter>,
    pub(crate) thumbnail: Option<EmbedThumbnail>,
    pub(crate) author: Option<EmbedAuthor>,
}
