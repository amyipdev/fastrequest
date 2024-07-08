// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::OpenOptions;
use std::io::Write;
use std::process::{Command, ExitStatus};

use chrono;

// Get path relative to the crate root.
// Taken from Rocket's rocket::fs::relative.
// Original under MIT License; github.com/rwf2/rocket
macro_rules! relative {
    ($path:expr) => {
        if cfg!(windows) {
            concat!(env!("CARGO_MANIFEST_DIR"), "\\", $path)
        } else {
            concat!(env!("CARGO_MANIFEST_DIR"), "/", $path)
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=NEVER_EXISTING_FILE");

    // install dependencies
    Command::new("npm").arg("i").status()?.eok()?;

    // Build Svelte
    Command::new("npm").arg("run").arg("build").status()?.eok()?;

    // Dynamically regenerate the Migrator
    // TODO: make a script for automatically generating the mRS files
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(relative!("src/migrator/mod.rs"))?;
    f.write_all(
        format!(
            "/* Auto-generated at {:?} */\nuse sea_orm_migration::prelude::*;\npub struct Migrator;\n",
            chrono::offset::Local::now()
        )
        .as_bytes(),
    )?;
    let files: Vec<String> = glob::glob(relative!("src/migrator/m*.rs"))?
        .filter_map(|e| match e {
            Ok(v) => match v.into_os_string().into_string() {
                Ok(s) => {
                    if !s.ends_with("/mod.rs") {
                        let v = s.rsplit_once("/").unwrap().1;
                        Some(v[..v.len() - 3].to_owned())
                    } else {
                        None
                    }
                }
                Err(e) => panic!("{:?} is an invalid path", e),
            },
            Err(e) => panic!("{:?}", e),
        })
        .collect();
    for k in &files {
        f.write_all(format!("mod {};\n", k).as_bytes())?;
    }
    f.write_all(b"#[async_trait::async_trait]\nimpl MigratorTrait for Migrator {\n    fn migrations() -> Vec<Box<dyn MigrationTrait>> {\n        vec![\n")?;
    for k in &files {
        f.write_all(format!("            Box::new({}::Migration),\n", k).as_bytes())?;
    }
    f.write_all(b"        ]\n    }\n}")?;
    f.flush()?;

    Command::new("cargo")
        .arg("run")
        .arg("--release")
        .current_dir(relative!("utils/migrator-entity-generator"))
        .status()?
        .eok()?;
    // TODO: generate entity using sea_orm_cli CRATE (NOT the command)
    std::fs::create_dir_all(relative!("src/entities"))?;
    sea_orm_cli::commands::generate::run_generate_command(
        sea_orm_cli::GenerateSubcommands::Entity {
            compact_format: false,
            expanded_format: false,
            include_hidden_tables: false,
            tables: vec![],
            ignore_tables: vec!["seaql_migrations".to_owned()],
            max_connections: 1,
            output_dir: relative!("src/entities").to_owned(),
            database_schema: "public".to_owned(),
            database_url: concat!(
                "sqlite://",
                env!("CARGO_MANIFEST_DIR"),
                "/.entity-gen-migr.tmpdb"
            )
            .to_owned(),
            with_serde: "none".to_owned(),
            serde_skip_deserializing_primary_key: false,
            serde_skip_hidden_column: false,
            with_copy_enums: false,
            date_time_crate: sea_orm_cli::DateTimeCrate::Chrono,
            lib: false,
            model_extra_derives: vec![],
            model_extra_attributes: vec![],
            enum_extra_derives: vec![],
            enum_extra_attributes: vec![],
            seaography: false,
        },
        false,
    )
    .await?;
    std::fs::remove_file(relative!(".entity-gen-migr.tmpdb"))?;

    Ok(())
}

// Stable override for exit_ok()
// Once exit_status_error (#84908) is stable, this can be removed
// and all .eok()s replaced with exit_ok()
trait ExitConversion {
    fn eok(&self) -> Result<(), ExitStatErr>;
}
#[derive(PartialEq, Eq, Clone, Copy, Debug)]
struct ExitStatErr(ExitStatus);
impl ExitConversion for ExitStatus {
    fn eok(&self) -> Result<(), ExitStatErr> {
        if self.success() {
            Ok(())
        } else {
            Err(ExitStatErr(*self))
        }
    }
}
impl std::fmt::Display for ExitStatErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "process exited unsuccesfully: {}", self.0)
    }
}
impl std::error::Error for ExitStatErr {}
