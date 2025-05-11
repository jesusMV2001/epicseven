// src/data_models.rs

use serde::Deserialize; // Necesario para deserializar JSON
use std::collections::HashMap; // Necesario para el tipo HashMap

// Definimos una estructura que coincide con la estructura principal del JSON
// El atributo #[derive(Deserialize)] permite a serde_json convertir JSON a esta struct
#[derive(Debug, Deserialize)]
pub struct BuildResponse {
    pub data: Vec<Build>, // El campo "data" es un vector de structs Build
}

// Definimos una estructura para cada build individual dentro del vector "data"
// Hacemos los campos públicos (pub) para poder acceder a ellos desde otros módulos
#[derive(Debug, Deserialize)]
pub struct Build {
    #[serde(rename = "artifactCode")] // Usamos rename si el nombre del campo en JSON es diferente al de Rust
    pub artifact_code: Option<String>, // Usamos Option<String> porque podría ser nulo
    pub atk: i32,
    pub chc: i32,
    pub chd: i32,
    #[serde(rename = "createDate")]
    pub create_date: String,
    pub def: i32,
    pub eff: i32,
    pub efr: i32,
    pub gs: i32,
    pub hp: i32,
    pub sets: HashMap<String, i32>, // Los sets son un mapa de nombre del set a cantidad
    pub spd: i32,
    #[serde(rename = "unitCode")]
    pub unit_code: String,
    #[serde(rename = "unitName")]
    pub unit_name: String,
}

// Nota: No hay función main en este archivo. Solo definiciones de structs.
