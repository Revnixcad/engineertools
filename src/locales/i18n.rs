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
        ("website_for_engineers", "A website for engineers!"),
        ("welcome_to_engineertools", "Welcome to engineertools.nl"),
        ("menu_home", "Home"),
        ("menu_basics", "Basic calculations"),
        // home text
        ("home_welcome", "Welcome to engineertools.nl"),
        ("home_description", "This is a website for engineers!"),
        ("home_resources", "You can use this website to find useful tools and resources for your engineering projects."),   
        // basic page
        ("basic_page_title", "Basic Calculations"),
        ("basic_page_description", "This page contains basic calculations for engineers."),
        // sum calculator
        ("sum_calculator", "Sum calculator"),
        ("calculate_sum", "Calculate Sum"),
        ("sum_result", "The sum is: "),
        // rectangular area calculator
        ("rec_area_calculator", "Rectangular area calculator"),
        ("rec_area_calculate", "Calculate rectangle area"),
        ("rec_area_result", "The area of rectangle is: "),
        ("rec_area_area", "Area"),
        ("rec_area_breedte", "Width"),
        ("rec_area_hoogte",  "Height"), 
        //circle area calculator
        ("circle_area_calculator", "Circle area calculator"),
        ("circle_area_radius", "Radius"),
        ("circle_area_calculate", "Calculate circle area"),
        ("circle_area_result", "The area of circle is: "),
        // other keys
        ("copy_to_clipboard", "Copy to clipboard"),
        // footer
        ("copyright", "Copyright 2025 engineertools.nl"),
        // ...add more keys
    ])
}

fn nl_translations() -> HashMap<&'static str, &'static str> {
    HashMap::from([
        ("website_for_engineers", "Een website voor engineers!"),
        ("welcome_to_engineertools", "Welkom bij engineertools.nl"),
        ("menu_home", "Home"),
        ("menu_basics", "Basis berekeningen"),
        // home text
        ("home_welcome", "Welkom bij engineertools.nl"),
        ("home_description", "Dit is een website voor engineers!"),
        ("home_resources", "Je kunt deze website gebruiken om nuttige tools en bronnen te vinden voor je engineering projecten."),
        // basic page
        ("basic_page_title", "Basis Berekeningen"),
        ("basic_page_description", "Deze pagina bevat basis berekeningen voor engineers."),
        // sum calculator
        ("sum_calculator", "Som Calculator"),
        ("calculate_sum", "Bereken som"),
        ("sum_result", "De som is: "),
        // rectangular area calculator
        ("rec_area_calculator", "Rechthoek oppervlakte calculator"),
        ("rec_area_calculate", "Bereken oppervlakte"),
        ("rec_area_result", "De oppervlakte van rechthoek is: "),
        ("rec_area_area", "Oppervlakte"),
        ("rec_area_breedte", "Breedte"),
        ("rec_area_hoogte", "Hoogte"), 
        // circle area calculator
        ("circle_area_calculator", "Cirkel oppervlakte calculator"),
        ("circle_area_radius", "Straal"),
        ("circle_area_calculate", "Bereken oppervlakte"),
        ("circle_area_result", "De oppervlakte van de cirkel is: "),
        // other keys
        ("copy_to_clipboard", "Kopieer naar klembord"),
        // footer
        ("copyright", "Auteursrecht 2025 engineertools.nl"),
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
