/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::{
    fs::File,
    io::Write,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};

pub mod gen_csharp;
pub use gen_csharp::{CSharpWrapper, Config};

use super::super::interface::ComponentInterface;

/// Generates C# bindings for the given ComponentInterface, in the given output directory.
pub fn write_bindings(
    config: &Config,
    ci: &ComponentInterface,
    out_dir: &Path,
    _try_format_code: bool,
) -> Result<()> {
    let mut cs_file = PathBuf::from(out_dir);
    cs_file.push(format!("{}.cs", ci.namespace()));
    let mut f = File::create(&cs_file).context("Failed to create .py file for bindings")?;
    write!(f, "{}", generate_csharp_bindings(config, &ci)?)?;

    // TODO: format code

    Ok(())
}

/// Genrates C# bindings
pub fn generate_csharp_bindings(config: &Config, ci: &ComponentInterface) -> Result<String> {
    use askama::Template;
    CSharpWrapper::new(config, &ci)
        .render()
        .map_err(|_| anyhow::anyhow!("failed to render C# bindings"))
}
