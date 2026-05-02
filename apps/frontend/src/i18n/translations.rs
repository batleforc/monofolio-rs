/// Supported UI languages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Fr,
    En,
}

impl Language {
    /// BCP-47 language tag.
    #[allow(dead_code)]
    pub fn as_str(self) -> &'static str {
        match self {
            Language::Fr => "fr",
            Language::En => "en",
        }
    }

    /// Short label displayed in the current-language indicator.
    #[allow(dead_code)]
    pub fn label(self) -> &'static str {
        match self {
            Language::Fr => "FR",
            Language::En => "EN",
        }
    }

    /// Label for the toggle button (shows the *other* language).
    pub fn toggle_label(self) -> &'static str {
        match self {
            Language::Fr => "EN",
            Language::En => "FR",
        }
    }
}

/// Static UI strings for one language.
pub struct Translations {
    pub nav_home: &'static str,
    pub nav_about: &'static str,
    pub nav_blog: &'static str,
    pub nav_docs: &'static str,
    pub nav_contact: &'static str,
    pub nav_projects: &'static str,
    pub nav_technologies: &'static str,
    pub hero_greeting: &'static str,
    pub hero_cta_about: &'static str,
    pub about_title: &'static str,
    pub timeline_title: &'static str,
    pub home_footer_contact_cta: &'static str,
    pub loading: &'static str,
    pub error_loading: &'static str,
    pub error_retry: &'static str,
}

pub static FR: Translations = Translations {
    nav_home: "Accueil",
    nav_about: "À propos",
    nav_blog: "Blog",
    nav_docs: "Docs",
    nav_contact: "Contact",
    nav_projects: "Projets",
    nav_technologies: "Carte techno",
    hero_greeting: "Bonjour 👋",
    hero_cta_about: "More about me",
    about_title: "À propos de moi",
    timeline_title: "Mon Parcours",
    home_footer_contact_cta: "Me contacter",
    loading: "Chargement…",
    error_loading: "Impossible de charger les données",
    error_retry: "Réessayer",
};

pub static EN: Translations = Translations {
    nav_home: "Home",
    nav_about: "About",
    nav_blog: "Blog",
    nav_docs: "Docs",
    nav_contact: "Contact",
    nav_projects: "Projects",
    nav_technologies: "Tech map",
    hero_greeting: "Hello 👋",
    hero_cta_about: "More about me",
    about_title: "About me",
    timeline_title: "My Journey",
    home_footer_contact_cta: "Contact me",
    loading: "Loading…",
    error_loading: "Failed to load data",
    error_retry: "Retry",
};
