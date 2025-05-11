use rusqlite::Connection;
use std::error::Error;
use crate::data_models::Build;
use serde_json;

// Función para conectar a la base de datos y crear la tabla si no existe
// Es pública (pub) y devuelve un Result con la conexión o un Box<dyn Error>
pub fn setup_database(db_path: &str) -> Result<Connection, Box<dyn Error>> {
    println!("Conectando a la base de datos SQLite en: {}", db_path);
    let conn = Connection::open(db_path)?; // Abre o crea el archivo de base de datos
    println!("Conexión a la base de datos establecida.");

    // Crea la tabla 'builds' si no existe
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

    Ok(conn) // Devolvemos la conexión
}

// Función para insertar una lista de builds en la base de datos
// Es pública (pub) y toma una referencia mutable a la conexión y un slice de Builds
pub fn insert_builds(conn: &mut Connection, builds: &[Build]) -> Result<(), Box<dyn Error>> {
    println!("Insertando {} builds en la base de datos...", builds.len());

    // Iniciamos una transacción para que todas las inserciones sean atómicas
    // Si alguna falla, ninguna se guarda.
    let tx = conn.transaction()?;

    { // Bloque para limitar el scope de la sentencia preparada
        let mut stmt = tx.prepare(
            "INSERT INTO builds (artifact_code, atk, chc, chd, create_date, def, eff, efr, gs, hp, sets, spd, unit_code, unit_name)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14)",
        )?;

        let mut inserted_count = 0;
        for build in builds {
            // Convertimos el mapa de sets a una cadena JSON para almacenarlo
            let sets_json = serde_json::to_string(&build.sets)?;

            stmt.execute([
                build.artifact_code.as_deref(),
                Some(&build.atk.to_string()),
                Some(&build.chc.to_string()),
                Some(&build.chd.to_string()),
                Some(&build.create_date),
                Some(&build.def.to_string()),
                Some(&build.eff.to_string()),
                Some(&build.efr.to_string()),
                Some(&build.gs.to_string()),
                Some(&build.hp.to_string()),
                Some(&sets_json),
                Some(&build.spd.to_string()),
                Some(&build.unit_code),
                Some(&build.unit_name),
            ])?;
            inserted_count += 1;
        }
        println!("{} builds preparadas para inserción.", inserted_count);
    } // La sentencia preparada se destruye aquí

    tx.commit()?; // Confirmamos la transacción
    println!("Inserciones confirmadas.");

    Ok(())
}

// Función para consultar builds con un filtro de GS
// Es pública (pub) y toma una referencia inmutable a la conexión y el valor mínimo de GS
// Devuelve un Result con un vector de tuplas o un Box<dyn Error>
pub fn query_builds_by_gs(conn: &Connection, min_gs: i32) -> Result<Vec<(String, i32, i32, i32)>, Box<dyn Error>> {
    println!("\nConsultando builds con GS > {}...", min_gs);
    let mut query_stmt = conn.prepare("SELECT unit_name, gs, spd, atk FROM builds WHERE gs > ?1 LIMIT 10")?; // Limitamos a 10 resultados

    let build_iter = query_stmt.query_map([min_gs], |row| {
        // Mapeamos cada fila del resultado a una tupla
        Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
    })?;

    let mut results = Vec::new();
    for build_result in build_iter {
        results.push(build_result?); // Recopilamos los resultados en un vector
    }

    println!("Consulta completada. Encontrados {} resultados.", results.len());
    Ok(results) // Devolvemos el vector de resultados
}

// Nota: Este archivo contiene funciones relacionadas con la base de datos.
