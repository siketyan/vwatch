mod hello;
mod list;

use crate::discord::interaction::{
    ApplicationCommandInteractionData, InteractionResponse, InteractionResponseType,
};
use crate::error::Error;

pub(crate) async fn handle_command(
    data: &ApplicationCommandInteractionData,
) -> Result<InteractionResponse, Error> {
    match data.name.as_str() {
        "hello" => Ok(hello::hello()),
        "list" => list::list(data).await,
        _ => Ok(InteractionResponse {
            ty: InteractionResponseType::ACKWithSource,
            data: None,
        }),
    }
}
