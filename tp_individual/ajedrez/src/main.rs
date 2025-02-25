use std::env;

use ::ajedrez::resultado_ajedrez;

// fn main() {
//     let app = Application::builder()
//         .application_id("com.github.ajedrez")
//         .build();
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    match resultado_ajedrez(args) {
        Ok(resultado) => {
            println!("{}", resultado);
        }
        Err(e) => {
            println!("{}", e);
        }
    }
}
