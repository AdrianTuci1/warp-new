use crate::crypto::decrypt;
use crate::state::AppState;

pub fn load_encrypted_config(state: &AppState) -> anyhow::Result<Option<crate::config::VpsConfig>> {
    let path = state.config_path();
    if !path.exists() {
        return Ok(None);
    }
    let ciphertext = std::fs::read_to_string(path)?;
    let plaintext = decrypt(&state.pairing_key, &ciphertext)?;
    Ok(Some(serde_json::from_str(&plaintext)?))
}

pub fn save_encrypted_config(
    state: &AppState,
    config: &crate::config::VpsConfig,
) -> anyhow::Result<()> {
    let plaintext = serde_json::to_string(config)?;
    let ciphertext = crate::crypto::encrypt(&state.pairing_key, &plaintext)?;
    std::fs::write(state.config_path(), ciphertext)?;
    Ok(())
}

pub fn load_encrypted_secrets(state: &AppState) -> anyhow::Result<crate::state::SecretStore> {
    let path = state.secrets_path();
    if !path.exists() {
        return Ok(crate::state::SecretStore::default());
    }
    let ciphertext = std::fs::read_to_string(path)?;
    let plaintext = decrypt(&state.pairing_key, &ciphertext)?;
    Ok(serde_json::from_str(&plaintext)?)
}

pub fn save_encrypted_secrets(
    state: &AppState,
    secrets: &crate::state::SecretStore,
) -> anyhow::Result<()> {
    let plaintext = serde_json::to_string(secrets)?;
    let ciphertext = crate::crypto::encrypt(&state.pairing_key, &plaintext)?;
    std::fs::write(state.secrets_path(), ciphertext)?;
    Ok(())
}

pub fn update_config_from_encrypted_payload(
    state: &AppState,
    encrypted_payload: &str,
) -> anyhow::Result<crate::config::VpsConfig> {
    let plaintext = decrypt(&state.pairing_key, encrypted_payload)?;
    let config: crate::config::VpsConfig = serde_json::from_str(&plaintext)?;
    save_encrypted_config(state, &config)?;
    let mut guard = state.config.lock().unwrap();
    *guard = Some(config.clone());
    Ok(config)
}

pub fn update_secrets_from_encrypted_payload(
    state: &AppState,
    encrypted_payload: &str,
) -> anyhow::Result<crate::state::SecretStore> {
    let plaintext = decrypt(&state.pairing_key, encrypted_payload)?;
    let secrets: crate::state::SecretStore = serde_json::from_str(&plaintext)?;
    save_encrypted_secrets(state, &secrets)?;
    let mut guard = state.secrets.lock().unwrap();
    *guard = secrets.clone();
    Ok(secrets)
}
