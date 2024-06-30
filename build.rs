// SPDX-License-Identifier: AGPL-3.0-or-later

use std::fs::OpenOptions;
use std::io::Write;
use std::process::Command;

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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo::rerun-if-changed=NEVER_EXISTING_FILE");

    // install dependencies
    Command::new("npm").arg("i").status()?;

    // Build Svelte
    Command::new("npm").arg("run").arg("build").status()?;

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

    Ok(())
}
