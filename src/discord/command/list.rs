use schetube::fetch_upcoming_videos;

use crate::discord::embed::{Embed, EmbedAuthor, EmbedFooter, EmbedThumbnail, EmbedType};
use crate::discord::interaction::{
    ApplicationCommandInteractionData, ApplicationCommandInteractionDataOptionValue,
    InteractionApplicationCommandCallbackData, InteractionResponse, InteractionResponseType,
};
use crate::error::Error;

pub(crate) async fn list(
    data: &ApplicationCommandInteractionData,
) -> Result<InteractionResponse, Error> {
    Ok(match &data.options {
        Some(options) => {
            let channel_id = options
                .iter()
                .find(|o| o.name == "channel_id")
                .and_then(|o| o.value.as_ref())
                .ok_or_else(|| Error::InvalidPayload("option not found".to_string()))?;

            let channel_id = match channel_id {
                ApplicationCommandInteractionDataOptionValue::String(str) => Some(str),
                _ => None,
            }
            .ok_or_else(|| Error::InvalidPayload("option value is invalid".to_string()))?;

            let (channel, videos) = fetch_upcoming_videos(channel_id)
                .await
                .map_err(|_| Error::InvalidPayload("fetch failed".to_string()))?;

            InteractionResponse {
                ty: InteractionResponseType::ChannelMessage,
                data: Some(InteractionApplicationCommandCallbackData {
                    content: "Fetched:".to_string(),
                    embeds: Some(
                        videos
                            .iter()
                            .map(|v| Embed {
                                ty: Some(EmbedType::Link),
                                title: Some(v.title.clone()),
                                description: None,
                                url: Some(format!("https://youtu.be/{}", v.id)),
                                color: None,
                                footer: Some(EmbedFooter {
                                    text: Some("VWatch".to_string()),
                                    icon_url: None,
                                    proxy_icon_url: None,
                                }),
                                thumbnail: Some(EmbedThumbnail {
                                    url: Some(v.thumbnail_url.clone()),
                                    proxy_url: None,
                                    height: None,
                                    width: None,
                                }),
                                author: Some(EmbedAuthor {
                                    name: Some(channel.name.clone()),
                                    url: Some(channel.url.clone()),
                                    icon_url: Some(channel.avatar_url.clone()),
                                    proxy_icon_url: None,
                                }),
                            })
                            .collect(),
                    ),
                }),
            }
        }
        _ => InteractionResponse {
            ty: InteractionResponseType::ChannelMessageWithSource,
            data: Some(InteractionApplicationCommandCallbackData {
                content: "Hello, world!".to_string(),
                embeds: None,
            }),
        },
    })
}
