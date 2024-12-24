use anyhow::Result;

use std::{
    fs::{copy, create_dir_all, read_dir, remove_dir, remove_dir_all, remove_file, rename, File},
    path::Path,
};

#[derive(Debug)]
pub enum AsyaCommand {
    Remove(String, bool),
    Create(String),
    Copy(String, String),
    Rename(String, String),
    Move(String, String),
    Find(String, String),
}

fn find_recursive(path: &str, name: &str) -> Result<Vec<String>> {
    let mut found = Vec::new();
    let path = Path::new(path);

    for entry in read_dir(&path)? {
        let entry = entry?;
        let metadata = entry.metadata()?;
        let path = entry.path().into_os_string().into_string().unwrap();

        if metadata.is_dir() {
            found.append(&mut find_recursive(&path, name)?);
        } else if metadata.is_file() && path.contains(name) {
            found.push(path);
        }
    }

    Ok(found)
}

fn is_likely_directory(path: &Path) -> bool {
    path.to_str()
        .map(|p| p.ends_with('/') || p.ends_with('\\'))
        .unwrap_or(false)
}

fn is_likely_file(path: &Path) -> bool {
    path.extension().is_some()
}

pub fn command_process(cmd: AsyaCommand) -> Result<()> {
    match cmd {
        AsyaCommand::Remove(path, force) => {
            log::info!("Removing {} force {}", path, force);
            let path = Path::new(&path);

            if !path.exists() {
                return Ok(());
            }

            if path.is_file() {
                remove_file(path)?;
            } else if path.is_dir() {
                if force {
                    remove_dir_all(path)?;
                } else {
                    remove_dir(path)?;
                }
            }

            Ok(())
        }
        AsyaCommand::Create(path) => {
            log::info!("Creating {}", path);
            let path = Path::new(&path);

            if path.exists() {
                return Ok(());
            }

            if is_likely_directory(path) {
                create_dir_all(path)?;
            } else if is_likely_file(path) {
                if let Some(prefix) = path.parent() {
                    create_dir_all(prefix)?;
                }

                File::create(path)?;
            }

            Ok(())
        }
        AsyaCommand::Copy(from, to) => {
            log::info!("Copying {} to {}", from, to);
            copy(&from, &to)?;
            Ok(())
        }
        AsyaCommand::Rename(from, to) | AsyaCommand::Move(from, to) => {
            log::info!("Renaming {} to {}", from, to);
            rename(&from, &to)?;
            Ok(())
        }
        AsyaCommand::Find(hint, name) => {
            for path in find_recursive(&hint, &name)? {
                println!("{}", path);
            }

            Ok(())
        }
    }
}
