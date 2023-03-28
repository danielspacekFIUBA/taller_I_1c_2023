use crate::errores::AjedrezError;

pub fn parse_args(args: Vec<String>) -> Result<String, AjedrezError> {
    if args.len() < 2 {
        Err(AjedrezError::NoArgumentError)
    } else {
        Ok(args[1].to_string())
    }
}
