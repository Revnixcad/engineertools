use crate::translations::{EN_TRANSLATIONS, NL_TRANSLATIONS};

pub struct I18n {
    translations: &'static phf::Map<&'static str, &'static str>,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        #[cfg(debug_assertions)]
        {
            eprintln!("Initializing i18n for lang: {}", lang);
        }

        let translations = match lang {
            "nl" => &NL_TRANSLATIONS,
            _ => &EN_TRANSLATIONS,
        };

        Self { translations }
    }

    pub fn t(&self, key: &str) -> String {
        self.translations
            .get(key)
            .cloned()
            .unwrap_or_else(|| key)
            .to_string()
    }
}

impl Clone for I18n {
    fn clone(&self) -> Self {
        Self {
            translations: self.translations,
        }
    }
}
