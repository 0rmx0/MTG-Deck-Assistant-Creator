use crate::settings::AiProvider;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn secrets_dir() -> Result<std::path::PathBuf, String> {
    let dir = crate::settings::Settings::config_dir().join("secrets");
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).map_err(|e| e.to_string())?;
    Ok(dir)
}

fn provider_file(provider: AiProvider) -> String {
    let key = match provider {
        AiProvider::Mistral => "mistral",
        AiProvider::OpenAi => "openai",
        AiProvider::Anthropic => "anthropic",
        AiProvider::Google => "google",
    };
    format!("{key}.key")
}

pub fn store_secret(provider: AiProvider, key: &str) -> Result<(), String> {
    let path = secrets_dir()?.join(provider_file(provider));
    fs::write(&path, key).map_err(|e| e.to_string())?;
    #[cfg(unix)]
    fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_secret(provider: AiProvider) -> Option<String> {
    let path = secrets_dir().ok()?.join(provider_file(provider));
    fs::read_to_string(path).ok().and_then(|s| {
        let s = s.trim().to_string();
        if s.is_empty() { None } else { Some(s) }
    })
}

pub fn delete_secret(provider: AiProvider) -> Result<(), String> {
    let path = secrets_dir()?.join(provider_file(provider));
    match fs::remove_file(&path) {
        Ok(()) => Ok(()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(e) => Err(e.to_string()),
    }
}
