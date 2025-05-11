// src/database.rs

use rusqlite::{params, Connection, ToSql};
use crate::data_models::Build;
use serde_json;
use std::collections::HashMap;
use std::error::Error;

// Función para conectar a la base de datos y crear la tabla si no existe
pub fn setup_database(db_path: &str) -> Result<Connection, Box<dyn Error>> {
    println!("Conectando a la base de datos SQLite en: {}", db_path);
    let conn = Connection::open(db_path)?;
    println!("Conexión a la base de datos establecida.");

    conn.execute(
        "CREATE TABLE IF NOT EXISTS builds (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            artifact_code TEXT,
            atk INTEGER,
            chc INTEGER,
            chd INTEGER,
            create_date TEXT,
            def INTEGER,
            eff INTEGER,
            efr INTEGER,
            gs INTEGER,
            hp INTEGER,
            sets TEXT, -- Almacenamos los sets como texto JSON
            spd INTEGER,
            unit_code TEXT NOT NULL,
            unit_name TEXT NOT NULL
        )",
        [],
    )?;
    println!("Tabla 'builds' verificada/creada.");

    Ok(conn)
}

// Función para insertar una lista de builds en la base de datos
pub fn insert_builds(conn: &mut Connection, builds: &[Build]) -> Result<(), Box<dyn Error>> {
    println!("Insertando {} builds en la base de datos...", builds.len());

    let tx = conn.transaction()?;

    {
        let mut stmt = tx.prepare(
            "INSERT INTO builds (artifact_code, atk, chc, chd, create_date, def, eff, efr, gs, hp, sets, spd, unit_code, unit_name)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        )?;

        let mut inserted_count = 0;
        for build in builds {
            let sets_json = serde_json::to_string(&build.sets)?;

            // Usamos params! macro para pasar los parámetros de forma segura
            stmt.execute(params![
                build.artifact_code, // Option<String> se maneja automáticamente
                build.atk,
                build.chc,
                build.chd,
                build.create_date,
                build.def,
                build.eff,
                build.efr,
                build.gs,
                build.hp,
                sets_json, // Insertamos el JSON de sets como TEXT
                build.spd,
                build.unit_code,
                build.unit_name,
            ])?;
            inserted_count += 1;
        }
        println!("{} builds preparadas para inserción.", inserted_count);
    }

    tx.commit()?;
    println!("Inserciones confirmadas.");

    Ok(())
}

// Función para consultar builds con un filtro de GS (ejemplo anterior)
pub fn query_builds_by_gs(conn: &Connection, min_gs: i32) -> Result<Vec<(String, i32, i32, i32)>, Box<dyn Error>> {
    println!("\nConsultando builds con GS > {}...", min_gs);
    let mut query_stmt = conn.prepare("SELECT unit_name, gs, spd, atk FROM builds WHERE gs > ?1 LIMIT 10")?;

    let build_iter = query_stmt.query_map(params![min_gs], |row| {
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;

    let mut results = Vec::new();
    for build_result in build_iter {
        results.push(build_result?);
    }

    println!("Consulta completada. Encontrados {} resultados.", results.len());
    Ok(results)
}


// --- Nueva función para consultar builds por sets y stats ---

// Definimos una estructura para los parámetros de búsqueda
// Usamos Option<> para que los filtros sean opcionales
#[derive(Debug)] // Añadimos Debug para poder imprimir los filtros
pub struct BuildSearchFilters {
    pub unit_name: Option<String>,
    pub required_set: Option<String>, // Nombre del set requerido (ej: "set_vampire")
    pub min_atk: Option<i32>,
    pub min_hp: Option<i32>,
    pub min_def: Option<i32>,
    pub min_spd: Option<i32>,
    pub min_chc: Option<i32>,
    pub min_chd: Option<i32>,
    pub min_eff: Option<i32>,
    pub min_efr: Option<i32>,
    pub min_gs: Option<i32>,
    // Podrías añadir más filtros aquí si los necesitas
}

// Implementación por defecto para BuildSearchFilters
impl Default for BuildSearchFilters {
    fn default() -> Self {
        BuildSearchFilters {
            unit_name: None,
            required_set: None,
            min_atk: None,
            min_hp: None,
            min_def: None,
            min_spd: None,
            min_chc: None,
            min_chd: None,
            min_eff: None,
            min_efr: None,
            min_gs: None,
        }
    }
}


// Función para consultar builds basándose en filtros de sets y stats
pub fn search_builds(conn: &Connection, filters: BuildSearchFilters) -> Result<Vec<Build>, Box<dyn Error>> {
    println!("\nBuscando builds con filtros: {:?}", filters);

    // Construimos la consulta SQL dinámicamente basándonos en los filtros proporcionados
    let mut query = "SELECT * FROM builds WHERE 1=1".to_string(); // 1=1 es un truco para facilitar añadir cláusulas AND

    // Vector para almacenar los parámetros de la consulta
    let mut boxed_params: Vec<Box<dyn ToSql>> = Vec::new();

    // Añadir filtro por nombre de personaje
    if let Some(unit_name) = filters.unit_name {
        query.push_str(" AND unit_name = ?");
        boxed_params.push(Box::new(unit_name));
    }

    // Añadir filtro por set requerido
    if let Some(required_set) = filters.required_set {
        query.push_str(" AND json_extract(sets, ? || '$') IS NOT NULL AND json_extract(sets, ? || '$') > 0");
        boxed_params.push(Box::new(format!("$.{}", required_set)));
        boxed_params.push(Box::new(format!("$.{}", required_set)));
    }

    // Añadir filtros por stats mínimos
    if let Some(min_atk) = filters.min_atk { query.push_str(" AND atk >= ?"); boxed_params.push(Box::new(min_atk)); }
    if let Some(min_hp) = filters.min_hp { query.push_str(" AND hp >= ?"); boxed_params.push(Box::new(min_hp)); }
    if let Some(min_def) = filters.min_def { query.push_str(" AND def >= ?"); boxed_params.push(Box::new(min_def)); }
    if let Some(min_spd) = filters.min_spd { query.push_str(" AND spd >= ?"); boxed_params.push(Box::new(min_spd)); }
    if let Some(min_chc) = filters.min_chc { query.push_str(" AND chc >= ?"); boxed_params.push(Box::new(min_chc)); }
    if let Some(min_chd) = filters.min_chd { query.push_str(" AND chd >= ?"); boxed_params.push(Box::new(min_chd)); }
    if let Some(min_eff) = filters.min_eff { query.push_str(" AND eff >= ?"); boxed_params.push(Box::new(min_eff)); }
    if let Some(min_efr) = filters.min_efr { query.push_str(" AND efr >= ?"); boxed_params.push(Box::new(min_efr)); }
    if let Some(min_gs) = filters.min_gs { query.push_str(" AND gs >= ?"); boxed_params.push(Box::new(min_gs)); }

    // Opcional: Limitar el número de resultados para no saturar
    query.push_str(" LIMIT 50"); // Limita a 50 resultados

    println!("Consulta SQL generada: {}", query);

    // Preparamos la sentencia con la consulta construida
    let mut stmt = conn.prepare(&query)?;

    // Aquí está la corrección: Creamos un slice de referencias a dyn ToSql
    // a partir del vector de Box<dyn ToSql>.
    let params_slice: Vec<&dyn ToSql> = boxed_params.iter().map(|b| b.as_ref() as &dyn ToSql).collect();

    // Ejecutamos la consulta y mapeamos las filas a structs Build
    let build_iter = stmt.query_map(
        params_slice.as_slice(), // Pasamos el slice de referencias
        |row| {
            // Mapeamos cada fila del resultado a una struct Build
            // Necesitamos deserializar el campo 'sets' de nuevo desde TEXT a HashMap
            let sets_json: String = row.get(11)?; // El índice 11 corresponde a la columna 'sets'
            let sets: HashMap<String, i32> = serde_json::from_str(&sets_json).unwrap_or_default(); // Deserializamos, usamos un valor por defecto si falla

            Ok(Build {
                artifact_code: row.get(1)?,
                atk: row.get(2)?,
                chc: row.get(3)?,
                chd: row.get(4)?,
                create_date: row.get(5)?,
                def: row.get(6)?,
                eff: row.get(7)?,
                efr: row.get(8)?,
                gs: row.get(9)?,
                hp: row.get(10)?,
                sets, // Usamos el HashMap deserializado
                spd: row.get(12)?,
                unit_code: row.get(13)?,
                unit_name: row.get(14)?,
            })
        },
    )?;

    let mut results = Vec::new();
    for build_result in build_iter {
        results.push(build_result?); // Recopilamos los resultados en un vector
    }

    println!("Búsqueda completada. Encontrados {} resultados.", results.len());
    Ok(results) // Devolvemos el vector de structs Build
}
