use serde::{Deserialize, Serialize};

/// A URL link shown on the home page.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SocialLink {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub primaire: bool,
    #[serde(rename = "imgUrl", default)]
    pub img_url: String,
}

/// A single entry in the career / education timeline.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HistoryEntry {
    pub title: String,
    /// Optional English translation of the title.
    #[serde(rename = "titleEn", default)]
    pub title_en: Option<String>,
    pub lieux: String,
    pub date: String,
    pub weight: u32,
    #[serde(rename = "imgUrl", default)]
    pub img_url: String,
    #[serde(rename = "icoUrl", default)]
    pub ico_url: String,
    #[serde(default)]
    pub description: String,
    /// Optional English translation of the description.
    #[serde(rename = "descriptionEn", default)]
    pub description_en: Option<String>,
    #[serde(default)]
    pub url: Vec<SocialLink>,
}

/// Configuration for the home page, loaded from `contents/home.yaml`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct HomeConfig {
    pub name: String,
    pub presentation: String,
    /// Optional English translation of the presentation text.
    #[serde(rename = "presentationEn", default)]
    pub presentation_en: Option<String>,
    #[serde(rename = "shortDescription")]
    pub short_description: String,
    /// Optional English translation of the short description.
    #[serde(rename = "shortDescriptionEn", default)]
    pub short_description_en: Option<String>,
    #[serde(rename = "coverTitle", default)]
    pub cover_title: Vec<String>,
    /// Optional English cover titles.
    #[serde(rename = "coverTitleEn", default)]
    pub cover_title_en: Option<Vec<String>>,
    #[serde(rename = "cvUrl", default)]
    pub cv_url: String,
    #[serde(rename = "contactEmail", default)]
    pub contact_email: Option<String>,
    #[serde(rename = "contactLocation", default)]
    pub contact_location: Option<String>,
    #[serde(rename = "contactAvailability", default)]
    pub contact_availability: Option<String>,
    #[serde(rename = "contactAvailabilityEn", default)]
    pub contact_availability_en: Option<String>,
    #[serde(rename = "currentWork", default)]
    pub current_work: Option<String>,
    #[serde(default)]
    pub url: Vec<SocialLink>,
    #[serde(default)]
    pub history: Vec<HistoryEntry>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn social_link_defaults() {
        let yaml = "name: GitHub\nurl: https://github.com\n";
        let link: SocialLink = serde_yaml::from_str(yaml).unwrap();
        assert_eq!(link.name, "GitHub");
        assert!(!link.primaire);
        assert!(link.img_url.is_empty());
    }

    #[test]
    fn home_config_round_trips() {
        let config = HomeConfig {
            name: "Max".to_string(),
            presentation: "Hello".to_string(),
            presentation_en: Some("Hello EN".to_string()),
            short_description: "Dev".to_string(),
            short_description_en: None,
            cover_title: vec!["Hello".to_string()],
            cover_title_en: None,
            cv_url: "cv.pdf".to_string(),
            contact_email: Some("contact@example.dev".to_string()),
            contact_location: Some("Nouvelle-Aquitaine, France".to_string()),
            current_work: Some(
                "Ingénieur Socle de fabrication / Couche d'échange à la Macif".to_string(),
            ),
            contact_availability: Some("Disponible".to_string()),
            contact_availability_en: Some("Available".to_string()),
            url: vec![],
            history: vec![],
        };
        let yaml = serde_yaml::to_string(&config).unwrap();
        let parsed: HomeConfig = serde_yaml::from_str(&yaml).unwrap();
        assert_eq!(config, parsed);
    }

    #[test]
    fn history_entry_bilingual_defaults() {
        let yaml = "title: T\nlieux: L\ndate: 2020\nweight: 1\n";
        let entry: HistoryEntry = serde_yaml::from_str(yaml).unwrap();
        assert!(entry.title_en.is_none());
        assert!(entry.description_en.is_none());
    }

    #[test]
    fn history_entry_url_defaults_to_empty_vec() {
        let yaml = "title: T\nlieux: L\ndate: 2020\nweight: 1\n";
        let entry: HistoryEntry = serde_yaml::from_str(yaml).unwrap();
        assert!(entry.url.is_empty());
    }
}
