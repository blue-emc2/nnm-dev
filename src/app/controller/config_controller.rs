use crate::app::{
    config::{Config, ConfigMessage},
    file::File,
    history::History,
};

pub struct ConfigController;

impl ConfigController {
    pub fn create(&self) -> Result<ConfigMessage, std::io::Error> {
        let config = Config::new();
        let config_file_path = config.file_path();

        if config_file_path.exists() {
            return Ok(ConfigMessage::ExistsConfig);
        }
        config.save()?;

        let history = History::new();
        history.save()?;

        Ok(ConfigMessage::Success(
            config_file_path.into_os_string().into_string().unwrap(),
        ))
    }
}
