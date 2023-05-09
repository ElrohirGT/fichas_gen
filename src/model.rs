use chrono::prelude::*;
use rand::{distributions::Standard, prelude::Distribution};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Ficha {
    pub completion_time: String,
    pub email: String,
    pub student_or_colaborator: FichaRole,

    pub nombres: String,
    pub apellidos: String,
    pub edad: u8,
    pub sexo: String,
    pub fecha_de_nacimiento: String,
    pub carnet: usize,
    pub direccion_domicilio: String,

    pub nombre_contacto_1: String,
    pub parentesco_contacto_1: String,
    pub telefono_contacto_1: String,

    pub nombre_contacto_2: String,
    pub parentesco_contacto_2: String,
    pub telefono_contacto_2: String,

    pub tiene_igss: YesNo,
    pub tiene_seguro: YesNo,
    pub que_seguro_tiene: String,
    pub nombre_aseguradora: String,

    pub enfermedad_femenina: YesNo,
    pub enfermedades_femeninas: String,

    pub padece_desde_el_nacimiento: YesNo,
    pub enfermedades_desde_el_nacimiento: String,
    pub familiares_padecen_desde_el_nacimiento: YesNo,
    pub enfermedades_del_familiar: String,
    pub parentesco_de_familiares: String,

    pub enfermedad_control_periodico: YesNo,
    pub enfermedades_control_periodico: String,
    pub medicamentos_que_toma: String,

    pub ha_sido_hospitalizado: YesNo,
    pub razon_hospitalizacion_1: String,
    pub fecha_hospitalizacion_1: String,
    pub razon_hospitalizacion_2: String,
    pub fecha_hospitalizacion_2: String,
    pub razon_hospitalizacion_3: String,
    pub fecha_hospitalizacion_3: String,

    pub lo_han_operado: YesNo,
    pub cirugia_1: String,
    pub fecha_cirugia_1: String,
    pub cirugia_2: String,
    pub fecha_cirugia_2: String,
    pub cirugia_3: String,
    pub fecha_cirugia_3: String,

    pub se_ha_fracturado_huesos: YesNo,
    pub fractura_1: String,
    pub fecha_fractura_1: String,
    pub fractura_2: String,
    pub fecha_fractura_2: String,
    pub fractura_3: String,
    pub fecha_fractura_3: String,

    pub es_alergico_comida: YesNo,
    pub alergia_comidas: String,

    pub es_alergico_medicamento: YesNo,
    pub alergia_medicamentos: String,

    pub es_alergico_clima: YesNo,
    pub alergias_clima: Vec<Climas>,
}

#[derive(Debug, Serialize)]
pub enum Climas {
    Seco,
    Humedo,
    Ventoso,
    Frío,
    Caluroso,
}

#[derive(Debug, Serialize)]
pub enum YesNo {
    YES,
    NO,
}

#[derive(Debug, Serialize)]
pub enum FichaRole {
    Estudiante,
    Colaborador,
}

impl Distribution<Ficha> for Standard {
    fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> Ficha {
        todo!()
    }
}
