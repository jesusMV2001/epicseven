// src/main.rs

mod data_models;
mod http_client;
mod database;

use std::error::Error;
use database::BuildSearchFilters; // Importamos la nueva estructura de filtros

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Parte 1: Hacer la petición HTTP y obtener el JSON ---
    let url = "https://krivpfvxi0.execute-api.us-west-2.amazonaws.com/dev/getBuilds";
    let request_body = "Apocalypse Ravi";

    let build_response = http_client::fetch_builds(url, request_body).await?;

    // --- Parte 2: Conectar a la base de datos y crear la tabla ---
    let db_path = "epicseven_builds.db";
    let mut conn = database::setup_database(db_path)?;

    // --- Parte 3: Insertar los datos en la base de datos ---
    database::insert_builds(&mut conn, &build_response.data)?;

    /*
    // --- Parte 4: Realizar una búsqueda avanzada ---

    // Ejemplo de filtros:
    // Buscar builds de Abyssal Yufine con set de robo de vida (set_vampire),
    // al menos 100 de eficacia, 3000 de ataque, 180 de velocidad y 20000 de vida.
    let my_filters = BuildSearchFilters {
        unit_name: Some("Abyssal Yufine".to_string()),
        required_set: Some("set_vampire".to_string()),
        min_eff: Some(100),
        min_atk: Some(3000),
        min_spd: Some(180),
        min_hp: Some(20000),
        ..Default::default() // Usamos ..Default::default() para dejar los otros campos como None
    };

    // Llamamos a la nueva función de búsqueda
    let found_builds = database::search_builds(&conn, my_filters)?;

    // Imprimir los resultados de la búsqueda
    if found_builds.is_empty() {
        println!("\nNo se encontraron builds que coincidan con los filtros.");
    } else {
        println!("\nBuilds encontradas que coinciden con los filtros:");
        for build in found_builds {
            // Imprimimos algunos detalles relevantes de la build encontrada
            println!("  Personaje: {}, GS: {}, SPD: {}, ATK: {}, HP: {}, EFF: {}, Sets: {:?}",
                     build.unit_name,
                     build.gs,
                     build.spd,
                     build.atk,
                     build.hp,
                     build.eff,
                     build.sets // Imprimimos el mapa de sets
            );
        }
    }
*/
    Ok(())
}
