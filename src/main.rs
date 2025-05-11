mod data_models;
mod http_client;
mod database;

use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // --- Parte 1: Hacer la petición HTTP y obtener el JSON ---
    let url = "https://krivpfvxi0.execute-api.us-west-2.amazonaws.com/dev/getBuilds";
    let request_body = "Abyssal Yufine";

    // hacemos la peticion
    let build_response = http_client::fetch_builds(url, request_body).await?;

    // --- Parte 2: Conectar a la base de datos y crear la tabla ---
    let db_path = "epicseven_builds.db";
    let mut conn = database::setup_database(db_path)?;

    // --- Parte 3: Insertar los datos en la base de datos ---
    database::insert_builds(&mut conn, &build_response.data)?;

    // --- Parte 4: Ejemplo de consulta a la base de datos ---
    let min_gs_filter = 450;
    let builds_found = database::query_builds_by_gs(&conn, min_gs_filter)?;

    // Imprimir los resultados de la consulta
    if builds_found.is_empty() {
        println!("No se encontraron builds con GS > {}.", min_gs_filter);
    } else {
        println!("\nResultados de la consulta (GS > {}):", min_gs_filter);
        // Iteramos sobre el vector de tuplas devuelto por la consulta e imprimimos la información.
        for (unit_name, gs, spd, atk) in builds_found {
            println!("  Personaje: {}, GS: {}, SPD: {}, ATK: {}", unit_name, gs, spd, atk);
        }
    }

    // Si todo ha ido bien, devolvemos Ok(())
    Ok(())
}
