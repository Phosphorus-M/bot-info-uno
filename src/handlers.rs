use std::{time::Instant};

use chrono::Datelike;
use pyo3::{PyAny, Python, Py};
use serde_json::Value;

pub fn help_message(user_id: i32) -> String {
    println!("El usuario {user_id} ha solicitado AYUDA.");
    let content = std::fs::read_to_string("./assets/messages/help.txt").unwrap();
    content
}

pub fn get_emails(user_id: i32, message: String) -> String{
    println!("El usuario {user_id} ha solicitado MAILS.");
    let content = std::fs::read_to_string("./assets/mails_escuelas.json").unwrap();
    let Ok(json): Result<Value, _> = serde_json::from_str(content.as_str()) else {
        return "No se ha podido obtener los mails".to_string();
    };
    let mut result = String::new();
    let escuela = message.split(" ").nth(1);
    let Some(escuela) = escuela else {
        for item in json.as_array().unwrap() {
            result = result + &format!("\n<u>Escuela de {}</u>\n", item["escuela"]);
            if let Some(mails) = item["mails"].as_array() {
                for mail in mails {
                    result = result + &format!("<b>{}</b>: {}\n", mail["name"], mail["mail"]);
                }
            } else {
                result = result + "No hay mails para esta escuela\n";
            }
        }
        return result;
    };
    for item in json.as_array().unwrap() {
        if item["escuela"].as_str().unwrap() == escuela {
            result = result + &format!("\n<u>Escuela de {}</u>\n", item["escuela"]);
            if let Some(mails) = item["mails"].as_array() {
                for mail in mails {
                    result = result + &format!("<b>{}</b>: {}\n", mail["name"], mail["mail"]);
                }
            } else {
                result = result + "No hay mails para esta escuela\n";
            }
        }
    }

    result
}

pub fn get_useful_links(user_id: i32, message: String) -> String{
    if message.as_str() == "feriados" {
        println!("El usuario {user_id} ha solicitado el Calendario de Feriados.");
        let content = std::fs::read_to_string("./assets/calendario_feriados.json").unwrap();
        let Ok(json): Result<Value, _> = serde_json::from_str(content.as_str()) else {
            return "No se ha podido obtener los feriados".to_string();
        };
        let mut result = format!("Calendario Feriados {}:\n\n",chrono::Local::now().year());
        for mes in json.as_array().unwrap() {
            result = result + &format!("<b><u>{}</u></b>\n", mes["mes"]);
            if let Some(feriados) = mes["feriados"].as_array() {
                for feriado in feriados {
                    result = result + &format!("<u>{}:</u> {}\n", feriado["dia"], feriado["motivo"]);
                }
            } else {
                result = result + "No hay feriados este mes\n";
            }
        }
        result
    }
    else{
        println!("El usuario {user_id} ha solicitado el Calendario Académico.");
        let content = std::fs::read_to_string("./assets/messages/calendario-academico.txt").unwrap();
        content
    }

}

#[allow(unused_assignments)]
pub fn request_url_information(py: Python<'_>, user_id: i32, chat_id: i32, message: String, bot: Py<PyAny>) -> String{
    println!("El usuario {} ha solicitado información de {}", user_id, message);
    let mut url= "";
    let mut name = "";
    if message.as_str() == "siu" {
        url = "https://autogestion.uno.edu.ar/uno/";
        name = "siu guarani";
    } else if message.as_str() == "campus" {
        url = "http://campusvirtual.uno.edu.ar/moodle/";
        name = "campus";
    } else {
        unreachable!()
    }

    let args = (chat_id, format!("<i>Solicitando información a {} ...</i>", url),);
    if let Err(_) = bot.call_method1(py, "send_message", args) {
        println!("Error al enviar el mensaje");
    };

    fetch_message(url, name)
}

fn fetch_message(url: &str, name: &str) -> String {
    let (status, latency) = get_info(url.to_string());

    if status == "200".to_string() {
        return format!("El {name} ha respondido <b>exitosamente</b> con una latencia de <b>{latency}ms</b>")
    } else {
        return format!("<b>Falló</b> la solicitud al {name}. Al parecer está caído");
    }
}


pub fn get_info(url: String) -> (String, String) {
    let t0 = Instant::now();
    let r = reqwest::blocking::get(&url);
    let t1 = Instant::now();
    match r {
        Ok(r) => {
            (r.status().as_str().to_string(), (t1 - t0).as_millis().to_string())
        },
        Err(err) => {
            let Some(status) = err.status() else {
                return ("Timeout".to_string(), (t1 - t0).as_millis().to_string());
            };
            (status.as_str().to_string(), (t1 - t0).as_millis().to_string())

        },
    }
}