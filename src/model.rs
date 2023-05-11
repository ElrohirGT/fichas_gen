use enum_iterator::{all, Sequence};
use lipsum::lipsum;
use rand::distributions::Standard;
use rand::prelude::*;
use serde::Serialize;

use std::collections::HashSet;

use crate::random_utils::*;

#[derive(Debug, Serialize)]
pub struct Ficha {
    #[serde(rename(serialize = "Completion Time"))]
    pub completion_time: String,
    #[serde(rename(serialize = "Email"))]
    pub email: String,
    #[serde(rename(serialize = "¿Eres estudiante o colaborador?"))]
    pub student_or_colaborator: FichaRole,

    #[serde(rename(serialize = "Nombres"))]
    pub nombres: String,
    #[serde(rename(serialize = "Apellidos"))]
    pub apellidos: String,
    #[serde(rename(serialize = "Edad"))]
    pub edad: u8,
    #[serde(rename(serialize = "Género"))]
    pub sexo: Sexos,
    #[serde(rename(serialize = "Fecha de Nacimiento"))]
    pub fecha_de_nacimiento: String,
    #[serde(rename(serialize = "Carnet"))]
    pub carnet: u16,
    #[serde(rename(serialize = "Dirección de domicilio"))]
    pub direccion_domicilio: String,

    #[serde(rename(serialize = "Nombre del contacto No.1"))]
    pub nombre_contacto_1: String,
    #[serde(rename(serialize = "Parentesco de contacto No.1"))]
    pub parentesco_contacto_1: String,
    #[serde(rename(serialize = "Número de teléfono del contacto No.1"))]
    pub telefono_contacto_1: String,

    #[serde(rename(serialize = "Nombre del contacto No.2"))]
    pub nombre_contacto_2: String,
    #[serde(rename(serialize = "Parentesco de contacto No.2"))]
    pub parentesco_contacto_2: String,
    #[serde(rename(serialize = "Número de teléfono del contacto No.2"))]
    pub telefono_contacto_2: String,

    #[serde(rename(
        serialize = "¿Cuenta actualmente con cobertura del Instituto Guatemalteco de Seguridad Social (IGSS)?"
    ))]
    pub tiene_igss: YesNo,
    #[serde(rename(serialize = "¿Cuenta actualmente con cobertura de algún seguro privado?"))]
    pub tiene_seguro: YesNo,
    #[serde(rename(serialize = "¿Con cuál cuenta?"))]
    pub que_seguro_tiene: String,
    #[serde(rename(serialize = "Nombre de la Aseguradora"))]
    pub nombre_aseguradora: String,

    #[serde(rename(
        serialize = "¿Padece de alguna enfermedad del aparato reproductivo femenino?"
    ))]
    pub enfermedad_femenina: Option<YesNo>,
    #[serde(rename(serialize = "¿Cuál o cuáles enfermedades?"))]
    pub enfermedades_femeninas: String,

    #[serde(rename(serialize = "¿Padece de alguna enfermedad desde su nacimiento? "))]
    pub padece_desde_el_nacimiento: YesNo,
    #[serde(rename(serialize = "¿Cuál es esa enfermedad que tiene desde de su nacimiento?"))]
    pub enfermedades_desde_el_nacimiento: String,
    #[serde(rename(
        serialize = "¿Alguien de su familia cercana (abuelos, padres, tíos, hijos) padece de alguna enfermedad hereditaria?"
    ))]
    pub familiares_padecen_desde_el_nacimiento: YesNo,
    #[serde(rename(serialize = "¿Cuál o cuáles son esas enfermedades hereditarias?"))]
    pub enfermedades_del_familiar: String,
    #[serde(rename(
        serialize = "Mencione el parentesco de sus familiares cercanos en el orden que listo las enfermedades de la pregunta anterior."
    ))]
    pub parentesco_de_familiares: String,

    #[serde(rename(
        serialize = "¿Actualmente usted padece de alguna enfermedad y/o síndrome por el cual requiera control periódico con médico y tratamiento diario o continuo?"
    ))]
    pub enfermedad_control_periodico: YesNo,
    #[serde(rename(serialize = "Indique cuál o cuáles enfermedades padece"))]
    pub enfermedades_control_periodico: String,
    #[serde(rename(serialize = "¿Qué medicamento toma?"))]
    pub medicamentos_que_toma: String,

    #[serde(rename(serialize = "¿Ha sido hospitalizado alguna vez por enfermedad?"))]
    pub ha_sido_hospitalizado: YesNo,
    #[serde(rename(serialize = "Indique la razón de su hospitalización más cercana"))]
    pub razon_hospitalizacion_1: String,
    #[serde(rename(serialize = "Indique la fecha aproximada de esta hospitalización"))]
    pub fecha_hospitalizacion_1: String,
    #[serde(rename(serialize = "Indique la razón de su segunda hospitalización más cercana"))]
    pub razon_hospitalizacion_2: String,
    #[serde(rename(serialize = "Indique la fecha aproximada de esta hospitalización2"))]
    pub fecha_hospitalizacion_2: String,
    #[serde(rename(serialize = "Indique la razón de su tercera hospitalización más cercana"))]
    pub razon_hospitalizacion_3: String,
    #[serde(rename(serialize = "Indique la fecha aproximada de esta hospitalización3"))]
    pub fecha_hospitalizacion_3: String,

    #[serde(rename(serialize = "¿Lo han operado alguna vez?"))]
    pub lo_han_operado: YesNo,
    #[serde(rename(serialize = "¿Cuál es su cirugía más reciente?"))]
    pub cirugia_1: String,
    #[serde(rename(serialize = "¿En qué fecha se la realizó?"))]
    pub fecha_cirugia_1: String,
    #[serde(rename(serialize = "¿Cuál es su segunda cirugía más reciente?"))]
    pub cirugia_2: String,
    #[serde(rename(serialize = "¿En qué fecha se la realizó?2"))]
    pub fecha_cirugia_2: String,
    #[serde(rename(serialize = "¿Cuál es su tercera cirugía más reciente?"))]
    pub cirugia_3: String,
    #[serde(rename(serialize = "¿En qué fecha se la realizó?3"))]
    pub fecha_cirugia_3: String,

    #[serde(rename(serialize = "¿Se ha fracturado algún hueso o huesos?"))]
    pub se_ha_fracturado_huesos: YesNo,
    #[serde(rename(serialize = "Indique cuál es el hueso más reciente"))]
    pub fractura_1: String,
    #[serde(rename(serialize = "Indique aproximadamente en que fecha fue esa fractura"))]
    pub fecha_fractura_1: String,
    #[serde(rename(serialize = "Indique cuál es el segundo hueso más reciente"))]
    pub fractura_2: String,
    #[serde(rename(serialize = "Indique aproximadamente en que fecha fue esa fractura2"))]
    pub fecha_fractura_2: String,
    #[serde(rename(serialize = "Indique cuál es el tercer hueso más reciente"))]
    pub fractura_3: String,
    #[serde(rename(serialize = "Indique aproximadamente en que fecha fue esa fractura3"))]
    pub fecha_fractura_3: String,

    #[serde(rename(serialize = "¿Es usted alérgico a alguna comida?"))]
    pub es_alergico_comida: YesNo,
    #[serde(rename(serialize = "¿Cuáles comidas? "))]
    pub alergia_comidas: String,

    #[serde(rename(serialize = "¿Es usted alérgico a algún medicamento/fármaco?"))]
    pub es_alergico_medicamento: YesNo,
    #[serde(rename(serialize = "¿Cuáles fármacos/medicamentos? "))]
    pub alergia_medicamentos: String,

    #[serde(rename(serialize = "¿Es usted alérgico a cambios climáticos?"))]
    pub es_alergico_clima: YesNo,
    #[serde(rename(serialize = "¿Es usted alérgico a cambios climáticos?"))]
    pub alergias_clima: String,
}

#[derive(Debug, Serialize, Sequence, Hash, PartialEq, Eq)]
pub enum Climas {
    Seco,
    Humedo,
    Ventoso,
    Frío,
    Caluroso,
}

#[derive(Debug, Serialize)]
pub enum YesNo {
    Si,
    No,
}

#[derive(Debug, Serialize)]
pub enum FichaRole {
    Estudiante,
    Colaborador,
}

#[derive(Debug, Serialize)]
pub enum Sexos {
    Masculino,
    Femenino,
}

impl Distribution<YesNo> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> YesNo {
        match rng.gen_bool(0.5) {
            true => YesNo::Si,
            false => YesNo::No,
        }
    }
}

impl Distribution<FichaRole> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> FichaRole {
        match rng.gen_bool(0.5) {
            true => FichaRole::Estudiante,
            false => FichaRole::Colaborador,
        }
    }
}

impl Distribution<Sexos> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Sexos {
        match rng.gen_bool(0.5) {
            true => Sexos::Masculino,
            false => Sexos::Femenino,
        }
    }
}

impl Distribution<Climas> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Climas {
        all::<Climas>()
            .choose(rng)
            .expect("Couldn't choose a `Clima`")
    }
}

impl Distribution<Ficha> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Ficha {
        let email = random_email(rng);
        let completion_time = random_date(rng, 2023, 2024);
        let nombres = random_names(rng);

        let apellidos = random_last_names(rng);

        let student_or_colaborator = rng.gen();
        let edad = rng.gen_range(18..70);
        let sexo = rng.gen();
        let fecha_de_nacimiento = random_date(rng, 1950, 2004);
        let carnet = rng.gen_range(10000..65535);
        let direccion_domicilio = lipsum(rng.gen_range(5..10));

        let nombre_contacto_1 = random_names(rng);
        let parentesco_contacto_1 = random_relationship(rng);
        let telefono_contacto_1 = rng.gen_range(10000000..99999999).to_string();

        let nombre_contacto_2 = random_names(rng);
        let parentesco_contacto_2 = random_relationship(rng);
        let telefono_contacto_2 = rng.gen_range(10000000..99999999).to_string();

        let tiene_igss = rng.gen();
        let tiene_seguro = rng.gen();

        let (que_seguro_tiene, nombre_aseguradora) = match tiene_seguro {
            YesNo::No => (String::new(), String::new()),
            YesNo::Si => (random_insurance(rng), random_insurance_company(rng)),
        };

        let (enfermedad_femenina, enfermedades_femeninas) = match sexo {
            Sexos::Femenino => {
                if let YesNo::Si = rng.gen() {
                    (Some(YesNo::Si), random_female_sickness(rng))
                } else {
                    (Some(YesNo::No), String::new())
                }
            }
            Sexos::Masculino => (None, String::new()),
        };

        let padece_desde_el_nacimiento = rng.gen();
        let enfermedades_desde_el_nacimiento = match padece_desde_el_nacimiento {
            YesNo::Si => random_sickness(rng),
            YesNo::No => String::new(),
        };

        let familiares_padecen_desde_el_nacimiento = rng.gen();
        let (enfermedades_del_familiar, parentesco_de_familiares) =
            match familiares_padecen_desde_el_nacimiento {
                YesNo::Si => (
                    format!("{}; {}", random_sickness(rng), random_sickness(rng)),
                    random_relationship(rng),
                ),
                YesNo::No => (String::new(), String::new()),
            };

        let enfermedad_control_periodico = rng.gen();
        let (enfermedades_control_periodico, medicamentos_que_toma) =
            match enfermedad_control_periodico {
                YesNo::No => (String::new(), String::new()),
                YesNo::Si => (random_sickness(rng), random_medicine(rng)),
            };
        let ha_sido_hospitalizado = rng.gen();
        let mut razon_hospitalizacion_1 = String::new();
        let mut fecha_hospitalizacion_1 = String::new();
        let mut razon_hospitalizacion_2 = String::new();
        let mut fecha_hospitalizacion_2 = String::new();
        let mut razon_hospitalizacion_3 = String::new();
        let mut fecha_hospitalizacion_3 = String::new();
        if let YesNo::Si = ha_sido_hospitalizado {
            razon_hospitalizacion_1 = random_sickness(rng);
            fecha_hospitalizacion_1 = random_date(rng, 2006, 2019);

            if rng.gen_bool(0.5) {
                razon_hospitalizacion_2 = random_sickness(rng);
                fecha_hospitalizacion_2 = random_date(rng, 2006, 2019);
            }
            if rng.gen_bool(0.5) {
                razon_hospitalizacion_3 = random_sickness(rng);
                fecha_hospitalizacion_3 = random_date(rng, 2006, 2019);
            }
        }

        let lo_han_operado = rng.gen();
        let mut cirugia_1 = String::new();
        let mut fecha_cirugia_1 = String::new();
        let mut cirugia_2 = String::new();
        let mut fecha_cirugia_2 = String::new();
        let mut cirugia_3 = String::new();
        let mut fecha_cirugia_3 = String::new();

        if let YesNo::Si = lo_han_operado {
            cirugia_1 = random_surgery(rng);
            fecha_cirugia_1 = random_date(rng, 2006, 2019);

            if rng.gen_bool(0.5) {
                cirugia_2 = random_surgery(rng);
                fecha_cirugia_2 = random_date(rng, 2006, 2019);
            }
            if rng.gen_bool(0.5) {
                cirugia_3 = random_surgery(rng);
                fecha_cirugia_3 = random_date(rng, 2006, 2019);
            }
        }

        let se_ha_fracturado_huesos = rng.gen();
        let mut fractura_1 = String::new();
        let mut fecha_fractura_1 = String::new();
        let mut fractura_2 = String::new();
        let mut fecha_fractura_2 = String::new();
        let mut fractura_3 = String::new();
        let mut fecha_fractura_3 = String::new();

        if let YesNo::Si = se_ha_fracturado_huesos {
            fractura_1 = random_bone(rng);
            fecha_fractura_1 = random_date(rng, 2006, 2019);

            if rng.gen_bool(0.5) {
                fractura_2 = random_bone(rng);
                fecha_fractura_2 = random_date(rng, 2006, 2019);
            }
            if rng.gen_bool(0.5) {
                fractura_3 = random_bone(rng);
                fecha_fractura_3 = random_date(rng, 2006, 2019);
            }
        }

        let es_alergico_comida = rng.gen();
        let alergia_comidas = match es_alergico_comida {
            YesNo::No => String::new(),
            YesNo::Si => random_food(rng),
        };

        let es_alergico_medicamento = rng.gen();
        let alergia_medicamentos = match es_alergico_medicamento {
            YesNo::No => String::new(),
            YesNo::Si => random_medicine(rng),
        };

        let es_alergico_clima = rng.gen();
        let alergias_clima: String = match es_alergico_clima {
            YesNo::No => String::new(),
            YesNo::Si => {
                let mut set: HashSet<Climas> = HashSet::new();
                for _ in 1..rng.gen_range(2..4) {
                    set.insert(rng.gen());
                }
                let climates: Vec<String> = set.into_iter().map(|c| format!("{:?}", c)).collect();
                climates.join(", ")
            }
        };

        Ficha {
            completion_time,
            email,
            student_or_colaborator,
            nombres,
            apellidos,
            edad,
            sexo,
            fecha_de_nacimiento,
            carnet,
            direccion_domicilio,
            nombre_contacto_1,
            parentesco_contacto_1,
            telefono_contacto_1,
            nombre_contacto_2,
            parentesco_contacto_2,
            telefono_contacto_2,
            tiene_igss,
            tiene_seguro,
            que_seguro_tiene,
            nombre_aseguradora,
            enfermedad_femenina,
            enfermedades_femeninas,
            padece_desde_el_nacimiento,
            enfermedades_desde_el_nacimiento,
            familiares_padecen_desde_el_nacimiento,
            enfermedades_del_familiar,
            parentesco_de_familiares,
            enfermedad_control_periodico,
            enfermedades_control_periodico,
            medicamentos_que_toma,
            ha_sido_hospitalizado,
            razon_hospitalizacion_1,
            fecha_hospitalizacion_1,
            razon_hospitalizacion_2,
            fecha_hospitalizacion_2,
            razon_hospitalizacion_3,
            fecha_hospitalizacion_3,
            lo_han_operado,
            cirugia_1,
            fecha_cirugia_1,
            cirugia_2,
            fecha_cirugia_2,
            cirugia_3,
            fecha_cirugia_3,
            se_ha_fracturado_huesos,
            fractura_1,
            fecha_fractura_1,
            fractura_2,
            fecha_fractura_2,
            fractura_3,
            fecha_fractura_3,
            es_alergico_comida,
            alergia_comidas,
            es_alergico_medicamento,
            alergia_medicamentos,
            es_alergico_clima,
            alergias_clima,
        }
    }
}
