// SPDX-License-Identifier: AGPL-3.0-or-later

use std::env::VarError;

pub fn pexi(s: &str) -> bool {
    std::path::Path::new(s).exists()
}

pub fn erxit(s: &str) -> ! {
    log::error!("{}", s);
    std::process::exit(1);
}

#[allow(dead_code)]
pub fn erxits(s: String) -> ! {
    erxit(&s)
}

pub fn uw_nofile(r: Result<String, VarError>) -> String {
    if let Ok(s) = r {
        s
    } else {
        erxit(
            "attempted to access non-existent file\ncheck configuration and environment variables",
        );
    }
}
