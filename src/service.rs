use anyhow::{bail, Result};
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};

use super::commands::{command_process, AsyaCommand};
use super::prompt::PROMPT_STRING;

#[derive(Debug)]
pub struct AsyaService {
    client: Client,
    model: Model,
}

impl AsyaService {
    pub fn new() -> Result<Self> {
        Ok(AsyaService {
            client: Client::new(None, None, None, None)?,
            model: Model::OpenMistral7b,
        })
    }

    pub fn execute(&self, message: String) -> Result<()> {
        let messages = vec![ChatMessage {
            role: ChatMessageRole::User,
            content: PROMPT_STRING.to_string() + &message,
            tool_calls: None,
        }];
        let options = ChatParams {
            temperature: 0.0,
            random_seed: Some(rand::random::<u32>()),
            ..Default::default()
        };
        let response = self
            .client
            .chat(self.model.clone(), messages, Some(options))?;

        match response.choices.first() {
            Some(message) => {
                let content = message.message.content.clone();
                let cmd = AsyaCommand::try_from(content)?;
                command_process(cmd)?;
            }
            None => bail!("No response from the model"),
        }

        Ok(())
    }
}
