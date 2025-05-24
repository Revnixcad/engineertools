use phf::phf_map;

pub static EN_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "welcome" => "Welcome to Leptos - served from Spin!",
    "click-me" => "Click Me: { $count }",
    "not-found" => "Not Found",
};

pub static NL_TRANSLATIONS: phf::Map<&'static str, &'static str> = phf_map! {
    "welcome" => "Welkom bij Leptos - geserveerd vanaf Spin!",
    "click-me" => "Klik Mij: { $count }",
    "not-found" => "Niet Gevonden",
};
