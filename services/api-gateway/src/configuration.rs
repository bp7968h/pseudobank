

#[derive(Debug, serde::Deserialize)]
pub struct Settings {
    pub gateway: GatewaySettings,
}

#[derive(Debug, serde::Deserialize)]
pub struct GatewaySettings {
    pub host: String,
    pub port: u16,
}

pub fn get_settings() -> Result<Settings, config::ConfigError> {
    let base_path = std::env::current_dir().expect("Failed to determine current directory");
    let config_dir = base_path.join("config");

    let environment: Environment = std::env::var("APP_ENV")
        .unwrap_or_else(|_| "local".into())
        .try_into()
        .expect("Failed to parse APP_ENV");
    let environment_filename = format!("{}.json", environment.as_str());

    let settings = config::Config::builder()
        .add_source(config::File::from(config_dir.join("base.json")))
        .add_source(config::File::from(config_dir.join(environment_filename)))
        .build()?;

    settings.try_deserialize::<Settings>()
}

pub(crate) enum Environment {
    Local,
    Production,
}

impl TryFrom<String> for Environment {
    type Error = String;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        match s.to_lowercase().as_str() {
            "local" => Ok(Self::Local),
            "production" | "prod" => Ok(Self::Production),
            other => Err(format!(
                "{} is not supported environment. Use either `loca` or `production` or `prod`",
                other
            ))
        }
    }
}

impl Environment {
    pub fn as_str(&self) -> &'static str {
        match self {
            Environment::Local => "local",
            Environment::Production => "production",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_environment_try_from() {
        assert!(matches!(Environment::try_from("local".to_string()), Ok(Environment::Local)));
        assert!(matches!(Environment::try_from("production".to_string()), Ok(Environment::Production)));
        assert!(matches!(Environment::try_from("prod".to_string()), Ok(Environment::Production)));
        
        assert!(Environment::try_from("invalid".to_string()).is_err());
    }
    
    #[test]
    fn test_environment_as_str() {
        assert_eq!(Environment::Local.as_str(), "local");
        assert_eq!(Environment::Production.as_str(), "production");
    }
}