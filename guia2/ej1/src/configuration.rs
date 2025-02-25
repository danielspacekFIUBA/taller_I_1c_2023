use crate::compartir;

#[derive(Debug)]
pub struct UsarConfiguracion {
    pub valor_leido: String,
}

const VALOR_DEFAULT: &str = "VALOR DEFAULT";

impl UsarConfiguracion {
    pub fn new() -> UsarConfiguracion {
        if let Ok(valor_leido) = compartir::get_valor() {
            UsarConfiguracion { valor_leido }
        } else {
            UsarConfiguracion {
                valor_leido: VALOR_DEFAULT.to_string(),
            }
        }
    }
}

/*
use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
    sync::{Arc, Mutex},
};

use std::sync;

struct Configuration {
    filename: String,
    Items: Vec<ConfigItem>,
}

pub struct ConfigItem {
    pub key: String,
    pub value: String,
}

const COMMENT_CHAR: char = '#';
const KEY_VALUE_SEPARATOR: char = '=';

impl Configuration {

    static GLOBAL_CONFIG: Arc<Mutex<HashMap<String, String>>> =
    Arc::new(Mutex::new(HashMap::new()));

    pub fn new(filename: String) -> Result<(), String> {
        if let Ok(file) = File::open(filename) {
            let buf = BufReader::new(file);
            let lineas = Configuration::parsear_archivo(buf)?;

            if lineas.len() == 0 {
                return Err("Archivo vacío".to_string());
            }


            for linea in lineas {
                if !linea.starts_with(COMMENT_CHAR) {
                    match linea.split_once(KEY_VALUE_SEPARATOR) {
                        Some((key, value)) => {
                            let item = ConfigItem {
                                key: key.to_string(),
                                value: value.to_string(),
                            };
                            if let Ok(mut lista) = GLOBAL_CONFIG.lock() {
                                lista.insert(item.key, item.value);
                            }
                        }
                        None => {}
                    }
                }
            }
        }
        Ok(())
    }

    fn parsear_archivo(buf: BufReader<File>) -> Result<Vec<String>, String> {
        let buf_lineas = buf.lines();
        let mut hay_errores = false;
        let lineas: Vec<String> = buf_lineas
            .map(|l| {
                if let Ok(linea) = l {
                    linea
                } else {
                    hay_errores = false;
                    "".to_string()
                }
            })
            .collect();
        if hay_errores {
            return Err("Error de archivo conf".to_string());
        }
        Ok(lineas)
    }

    pub fn get(key: String) -> Result<String, String> {
        let mut retorno = "".to_string();
        if let Ok(lista) = GLOBAL_CONFIG.lock() {
            if let Some(valor) = lista.get(&key) {
                retorno = valor.to_string();
            } else {
                return Err("Clave inexistente".to_string());
            }
        } else {
            return Err("Error en la configurafción".to_string());
        }
        return Ok(retorno);
    }
}
*/
