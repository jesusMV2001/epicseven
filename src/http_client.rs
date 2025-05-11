// src/http_client.rs

use reqwest::Client; // Importamos el cliente reqwest
use std::error::Error; // Para manejar errores
use crate::data_models::BuildResponse; // Importamos la estructura BuildResponse de nuestro módulo data_models

// Función para hacer la petición HTTP POST y devolver el resultado parseado
// Es pública (pub) para poder llamarla desde main.rs
// Es asíncrona (async) y devuelve un Result que contiene BuildResponse o un Box<dyn Error>
pub async fn fetch_builds(url: &str, body: &str) -> Result<BuildResponse, Box<dyn Error>> {
    let client = Client::new();

    println!("Realizando petición POST a: {}", url);
    println!("Cuerpo de la petición: {}", body);

    let response = client.post(url)
        .header("Content-Type", "text/plain")
        .body(body.to_string()) // Convertimos &str a String para el cuerpo
        .send()
        .await?; // Esperamos la respuesta y manejamos errores

    // Verificamos si la petición fue exitosa
    if response.status().is_success() {
        println!("Petición exitosa. Código de estado: {}", response.status());

        let body_text = response.text().await?;
        // println!("Cuerpo de la respuesta (primeras 200 chars):\n{}", &body_text[..std::cmp::min(body_text.len(), 200)]); // Opcional: imprimir parte del cuerpo

        // Parseamos el cuerpo JSON a nuestra estructura BuildResponse
        let build_response: BuildResponse = serde_json::from_str(&body_text)?;
        println!("JSON parseado exitosamente. Encontradas {} builds.", build_response.data.len());

        Ok(build_response) // Devolvemos la estructura parseada
    } else {
        // Si hay error, obtenemos el cuerpo del error y lo incluimos en el mensaje de error
        let error_body = response.text().await?;
        Err(format!("Error en la petición. Código de estado: Cuerpo: {}", error_body).into()) // Devolvemos un error con información
    }
}

// Nota: No hay función main en este archivo. Solo la función de petición HTTP.
