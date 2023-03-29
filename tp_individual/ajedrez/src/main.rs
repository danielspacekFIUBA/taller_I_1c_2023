use std::env;

use ::ajedrez::resultado_ajedrez;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    match resultado_ajedrez(args) {
        Ok(resultado) => {
            println!("{}", resultado);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}