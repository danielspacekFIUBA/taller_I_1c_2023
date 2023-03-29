use ajedrez::{self, resultado_ajedrez};

#[test]
fn integration_test_all() {
    resultado_test_01();
    resultado_test_02();
    resultado_test_03();
    resultado_test_05();
    resultado_test_09();
}

#[test]
fn resultado_test_01() {
    let args: Vec<String> = vec!["target".to_string(), "src/test_files/test1.txt".to_string()];
    let resultado_result = resultado_ajedrez(args);
    assert!(resultado_result.is_ok());

    let resultado = resultado_result.unwrap();
    assert_eq!(resultado, "B".to_string())
}

#[test]
fn resultado_test_02() {
    let args: Vec<String> = vec!["target".to_string(), "src/test_files/test2.txt".to_string()];
    let resultado_result = resultado_ajedrez(args);
    assert!(resultado_result.is_ok());

    let resultado = resultado_result.unwrap();
    assert_eq!(resultado, "P".to_string())
}

#[test]
fn resultado_test_03() {
    let args: Vec<String> = vec!["target".to_string(), "src/test_files/test3.txt".to_string()];
    let resultado_result = resultado_ajedrez(args);
    assert!(resultado_result.is_ok());

    let resultado = resultado_result.unwrap();
    assert_eq!(resultado, "P".to_string())
}

#[test]
fn resultado_test_05() {
    let args: Vec<String> = vec!["target".to_string(), "src/test_files/test5.txt".to_string()];
    let resultado_result = resultado_ajedrez(args);
    assert!(resultado_result.is_ok());

    let resultado = resultado_result.unwrap();
    assert_eq!(resultado, "E".to_string())
}

#[test]
fn resultado_test_09() {
    let args: Vec<String> = vec!["target".to_string(), "src/test_files/test9.txt".to_string()];
    let resultado_result = resultado_ajedrez(args);
    assert!(resultado_result.is_err());
}
