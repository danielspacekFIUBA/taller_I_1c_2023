use std::sync::{Mutex, MutexGuard};

struct ConfigItem {
    key: String,
    value: String,
}

static HASH_CONFIG: Mutex<Vec<ConfigItem>> = Mutex::new(vec![]);

fn access_config() -> Result<MutexGuard<'static, Vec<ConfigItem>>, String> {
    if let Ok(retorno) = HASH_CONFIG.lock() {
        return Ok(retorno);
    }
    Err("Error al lockear el config".to_string())
}

pub fn create_config() {
    //let filename = "nodo.conf".to_string();
    if let Ok(mut config) = access_config() {
        config.push(ConfigItem {
            key: "NOMBRE".to_string(),
            value: "Daniel".to_string(),
        });
    };
}

pub fn get_valor() -> Result<String, String> {
    let clave = "NOMBRE".to_string();
    let config = access_config()?;
    for item in config.iter() {
        if item.key == clave {
            let valor_clonado = item.value.clone();
            return Ok(valor_clonado);
        }
    }
    Err("No existe la clave".to_string())
}
