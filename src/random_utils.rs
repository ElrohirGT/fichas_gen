use chrono::{prelude::*, LocalResult};
use rand::prelude::*;

const EMAILS: &'static [&'static str] = &[
    "tracto@uvg.edu.gt",
    "something@uvg.edu.gt",
    "palabra@uvg.edu.gt",
    "uba@uvg.edu.gt",
    "portal@uvg.edu.gt",
    "palanca@uvg.edu.gt",
    "hotel@uvg.edu.gt",
];

pub fn random_email<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    EMAILS.iter().choose(rng).unwrap().to_string()
}

const FOOD: &'static [&'static str] = &["Lácteos", "Maní", "Mariscos"];
pub fn random_food<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    FOOD.into_iter().choose(rng).unwrap().to_string()
}

const BONES: &'static [&'static str] = &["Clavícula", "Fémur", "Costilla", "Cráneo", "Tibia"];
pub fn random_bone<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    BONES.into_iter().choose(rng).unwrap().to_string()
}

const SURGERIES: &'static [&'static str] = &[
    "Cardíaca",
    "Hepática",
    "Extirpación de Glándula",
    "Reparación de Hernia",
];
pub fn random_surgery<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    SURGERIES.into_iter().choose(rng).unwrap().to_string()
}

const MEDICINES: &'static [&'static str] = &[
    "Abacavir",
    "Tabcin",
    "Abilify",
    "Absorbine",
    "Abstral",
    "L-glutamina",
    "L-Triyodotironinaver Liotironina",
    "Labetalol",
    "Labid",
    "Lac-Hydrinver Lactato de amonio tópico",
    "Lacosamida",
    "Lactato de amonio tópico",
    "Lactulosa",
    "Ladakamycinver Azacitidina Inyectable",
    "Lagevrio",
];

pub fn random_medicine<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    MEDICINES.into_iter().choose(rng).unwrap().to_string()
}

const SICKNESS: &'static [&'static str] = &[
    "Anotia",
    "Microtia",
    "Anoftalmía",
    "Microftalmía",
    "Craneosinostosis",
    "Atresia Pulmonar",
    "Espina Bífida",
    "Tronco Arterioso",
];
pub fn random_sickness<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    SICKNESS.into_iter().choose(rng).unwrap().to_string()
}

const FEMALE_SICKNESS: &'static [&'static str] = &[
    "Cáncer de mama",
    "Cáncer de cuello uterino",
    "Quistes ováricos",
    "Enfermedad Inflamatoria Pélvica",
];
pub fn random_female_sickness<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    FEMALE_SICKNESS.into_iter().choose(rng).unwrap().to_string()
}

const COMPANIES: &'static [&'static str] = &[
    "El Roble",
    "Yap",
    "BBVA Seguros",
    "Aseguradora General",
    "Seguros MAPFRE",
];
pub fn random_insurance_company<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    COMPANIES.into_iter().choose(rng).unwrap().to_string()
}

const INSURANCES: &'static [&'static str] = &["Carro", "Casa", "Motocicleta", "Vida"];
pub fn random_insurance<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    INSURANCES.into_iter().choose(rng).unwrap().to_string()
}

const RELATIONSHIPS: &'static [&'static str] =
    &["Abuela", "Abuelo", "Madre", "Padre", "Tío", "Tía"];
pub fn random_relationship<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    RELATIONSHIPS.into_iter().choose(rng).unwrap().to_string()
}

const LAST_NAMES: &'static [&'static str] = &[
    "Paz",
    "Donis",
    "Mancía",
    "Salazar",
    "Roldán",
    "Sagastume",
    "Arroyo",
    "Paniagua",
    "Pineda",
];
pub fn random_last_names<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    format!(
        "{} {}",
        LAST_NAMES.iter().choose(rng).unwrap(),
        LAST_NAMES.iter().choose(rng).unwrap()
    )
}

const NAMES: &'static [&'static str] = &[
    "Flavio",
    "Andre",
    "Gaby",
    "Fernando",
    "Gerardo",
    "Brandon",
    "Francisco",
    "Godinez",
    "Rodrigo",
    "Marta",
    "Pablo",
    "Paris",
    "Manolo",
    "Mario",
    "Juan",
    "Hal",
    "Tito",
];
pub fn random_names<R: rand::Rng + ?Sized>(rng: &mut R) -> String {
    format!(
        "{} {}",
        NAMES.iter().choose(rng).unwrap(),
        NAMES.iter().choose(rng).unwrap()
    )
}

pub fn random_date<R: rand::Rng + ?Sized>(rng: &mut R, start_year: i32, end_year: i32) -> String {
    let mut date = LocalResult::None;
    while let LocalResult::None | LocalResult::Ambiguous(_, _) = date {
        let year = rng.gen_range(start_year..end_year);
        let month = rng.gen_range(1..12);
        let day = rng.gen_range(1..27);

        date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0);
    }
    date.unwrap().format("%d/%m/%Y").to_string()
}
