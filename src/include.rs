use twitchchat::UserConfig;

fn get_env_var(key: &str) -> Result<String, String> {
    if let Ok(var) = std::env::var(key) {
        Ok(var)
    } else {
        Err(format!("please set `{}`", key))
    }
}

pub fn get_user_config() -> Result<UserConfig, String> {
    let name = get_env_var("TWITCH_USERNAME")?;
    let token = get_env_var("TWITCH_TOKEN")?;

    super::error_to_string(
        UserConfig::builder()
            .name(name)
            .token(token)
            .enable_all_capabilities()
            .build()
    )
}

pub fn channels_to_join() -> Result<Vec<String>, String> {
    Ok(
        get_env_var("TWITCH_CHANNELS")?
            .split(',')
            .map(ToString::to_string)
            .collect::<Vec<_>>()
    )
}
