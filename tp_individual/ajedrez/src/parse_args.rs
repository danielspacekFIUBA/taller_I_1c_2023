use crate::errores::AjedrezError;

/// Parsea los argumentos del main
pub fn parse_args(args: Vec<String>) -> Result<String, AjedrezError> {
    if args.len() < 2 {
        Err(AjedrezError::NoArgument)
    } else {
        Ok(args[1].to_string())
    }
}
