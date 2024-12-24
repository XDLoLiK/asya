use anyhow::{bail, Result};

use super::commands::AsyaCommand;

fn command_size(cmd: &str) -> Result<usize> {
    match cmd {
        "remove" => Ok(3),
        "create" => Ok(2),
        "copy" => Ok(3),
        "rename" => Ok(3),
        "move" => Ok(3),
        "find" => Ok(3),
        other => bail!("Unknown command {:?}", other),
    }
}

impl TryFrom<String> for AsyaCommand {
    type Error = anyhow::Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let tokens: Vec<&str> = value.split_whitespace().collect();

        if tokens.is_empty() {
            bail!("Empty string");
        }

        let cmd = tokens[0].to_lowercase();
        let cmd_size = command_size(&cmd)?;

        if tokens.len() != cmd_size {
            bail!("Invalid args number for command {:?}", cmd);
        }

        match cmd.as_str() {
            "remove" => {
                let path = tokens[1];
                let force = tokens[2].parse::<bool>()?;
                Ok(AsyaCommand::Remove(path.to_string(), force))
            }
            "create" => {
                let path = tokens[1];
                Ok(AsyaCommand::Create(path.to_string()))
            }
            "copy" => {
                let from = tokens[1];
                let to = tokens[2];
                Ok(AsyaCommand::Copy(from.to_string(), to.to_string()))
            }
            "rename" => {
                let from = tokens[1];
                let to = tokens[2];
                Ok(AsyaCommand::Rename(from.to_string(), to.to_string()))
            }
            "move" => {
                let from = tokens[1];
                let to = tokens[2];
                Ok(AsyaCommand::Move(from.to_string(), to.to_string()))
            }
            "find" => {
                let hint = tokens[1];
                let name = tokens[2];
                Ok(AsyaCommand::Find(hint.to_string(), name.to_string()))
            }
            other => bail!("Unknown command: {:?}", other),
        }
    }
}
