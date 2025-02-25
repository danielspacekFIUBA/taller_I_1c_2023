use crate::configuration::UsarConfiguracion;

mod compartir;
mod configuration;

fn main() {
    //let s1 = String::from("hola");
    //let mut v = Vec::new();
    //v.push(s1);
    ////println!("{:?}", v);
    //let s2 = v.swap_remove(0);
    //println!("{}", s2);

    let config1 = UsarConfiguracion::new();
    println!("Valor leido: {:?}", config1);

    compartir::create_config();
    let _resultado = match compartir::get_valor() {
        Ok(valor) => println!("El valor es: {:?}", valor),
        Err(error) => println!("ERROR: {:?}", error),
    };

    let config2 = UsarConfiguracion::new();
    println!("Valor leido: {:?}", config2);
}
