use serde::{Deserialize, Serialize};

/// Top-level site configuration loaded from `contents/config.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SiteConfig {
    pub port: u16,
    pub env: String,
}

impl Default for SiteConfig {
    fn default() -> Self {
        Self {
            port: 3000,
            env: "development".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_config_has_expected_values() {
        let cfg = SiteConfig::default();
        assert_eq!(cfg.port, 3000);
        assert_eq!(cfg.env, "development");
    }

    #[test]
    fn config_deserialises_from_yaml() {
        let yaml = "port: 8080\nenv: production\n";
        let cfg: SiteConfig = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(cfg.port, 8080);
        assert_eq!(cfg.env, "production");
    }
}
