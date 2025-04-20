use std::{
    env,
    fs::{self, File},
    io::Write,
    path::PathBuf,
};

use clorinde::{
    Error,
    config::{Config, Package},
};
use postgres::{Client, tls};

#[allow(clippy::result_large_err)]
fn main() -> Result<(), Error> {
    let queries_path = "./db/queries";
    let schema_path = "./db/schema.sql";

    let cfg = Config {
        destination: PathBuf::from("generated_db_crate"),
        queries: PathBuf::from(queries_path),
        package: Package {
            name: String::from("db"),
            ..Default::default()
        },
        ..Default::default()
    };

    println!("cargo:rerun-if-changed={queries_path}");
    println!("cargo:rerun-if-changed={schema_path}");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    clorinde::gen_live(
        &mut Client::connect(&db_url, tls::NoTls).expect("Failed to connect to DB"),
        cfg,
    )?;

    // Fix warnings in generated code
    let generated_lib = PathBuf::from("./generated_db_crate/src/lib.rs");
    let mut content =
        fs::read_to_string(&generated_lib).expect("Failed to read from generated lib");
    content = content.replace("mod utils;", "#[allow(dead_code)]\nmod utils;");
    content = content.replace(
        "pub(crate) use utils::slice_iter;",
        "#[allow(unused_imports)]\npub(crate) use utils::slice_iter;",
    );
    File::create(&generated_lib)
        .expect("Failed to open generated lib.rs file")
        .write_all(content.as_bytes())
        .expect("Failed to write to generated lib.rs file");

    Ok(())
}
