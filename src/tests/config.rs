#[cfg(test)]
use crate::*;

#[test]
fn creation() {
    let _ = Config::builder()
        .add_source(config::File::with_name("default/config.toml"))
        .build()
        .unwrap();

    let _ = generate_config("default/config.toml");
}

#[test]
fn read() {
    let c = Config::builder()
        .add_source(config::File::with_name("default/config.toml"))
        .build()
        .unwrap();

    let mval: String = c.get("hostname").unwrap();
    let gval: String = get_value(&c, "hostname");

    assert_eq!(mval, "KRONOS");
    assert_eq!(gval, "KRONOS");
}

#[test]
fn write() {
    let c = Config::builder()
        .add_source(config::File::with_name("default/config.toml"))
        .build()
        .unwrap();

    let c = add_value(c, "test_val", "TEST");
    let val: String = get_value(&c, "test_val");

    assert_eq!(val, "TEST")
}
