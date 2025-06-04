use std::borrow::Cow;
use std::collections::HashMap;

#[derive(PartialEq, Eq, Clone)]
pub struct I18n {
    lang: String,
    translations: HashMap<&'static str, &'static str>,
}

impl I18n {
    pub fn new(lang: &str) -> Self {
        let translations = match lang {
            "nl" => nl_translations(),
            _ => en_translations(),
        };
        Self {
            lang: lang.to_string(),
            translations,
        }
    }

    pub fn t<'a>(&'a self, key: &'a str) -> Cow<'a, str> {
        match self.translations.get(key) {
            Some(&val) => Cow::Borrowed(val),
            None => Cow::Borrowed(key),
        }
    }
}

fn en_translations() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("welcome", "Welcome!"),
        ("sum_calculator", "Sum Calculator"),
        // ...add more keys
    ])
}

fn nl_translations() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("welcome", "Welkom!"),
        ("sum_calculator", "Som Calculator"),
        // ...add more keys
    ])
}

// write a test for the i18n module
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i18n_en() {
        let i18n = I18n::new("en");
        assert_eq!(i18n.t("welcome"), "Welcome!");
        assert_eq!(i18n.t("sum_calculator"), "Sum Calculator");
        assert_eq!(i18n.t("non_existent_key"), "non_existent_key");
    }

    #[test]
    fn test_i18n_nl() {
        let i18n = I18n::new("nl");
        assert_eq!(i18n.t("welcome"), "Welkom!");
        assert_eq!(i18n.t("sum_calculator"), "Som Calculator");
        assert_eq!(i18n.t("non_existent_key"), "non_existent_key");
    }
}
