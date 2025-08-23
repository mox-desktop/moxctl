use clap::Parser;
use rayon::prelude::*;
use std::{fs, process::Command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn find_mox_binaries() -> anyhow::Result<Vec<(String, String)>> {
    use std::collections::HashMap;

    let path = std::env::var("PATH")?;

    let mut unique_binaries: HashMap<String, String> = HashMap::new();

    let found_binaries: Vec<(String, String)> = path
        .split(':')
        .par_bridge()
        .flat_map_iter(|dir_path| {
            fs::read_dir(dir_path).ok().into_iter().flat_map(|dir| {
                dir.filter_map(Result::ok).filter_map(|entry| {
                    let file_name = entry.file_name();
                    let file_name = file_name.to_string_lossy();
                    let path = entry.path().display().to_string();

                    if !entry.path().is_file() {
                        return None;
                    }

                    if file_name.starts_with("mox") && file_name.ends_with("ctl") {
                        let subcommand = file_name
                            .strip_prefix("mox")
                            .and_then(|s| s.strip_suffix("ctl"))?
                            .to_string();

                        if subcommand.is_empty() {
                            return Some(("".to_string(), path));
                        }

                        Some((subcommand, path))
                    } else {
                        None
                    }
                })
            })
        })
        .collect();

    for (subcommand, binary_path) in found_binaries {
        unique_binaries.entry(subcommand).or_insert(binary_path);
    }

    Ok(unique_binaries.into_iter().collect())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let binaries = Box::leak(Box::new(find_mox_binaries()?));

    if binaries.is_empty() {
        eprintln!("No mox*ctl binaries found in PATH");
        return Ok(());
    }

    let mut mox = clap::Command::new("mox")
        .version("1.0.0")
        .about("Dynamic CLI wrapper for mox*ctl binaries")
        .arg_required_else_help(true);

    for (subcommand, _path) in binaries.iter() {
        if subcommand.is_empty() {
            mox = mox.subcommand(
                clap::Command::new("ctl").about("Execute moxctl").arg(
                    clap::Arg::new("args")
                        .help("Arguments to pass to moxctl")
                        .num_args(0..)
                        .allow_hyphen_values(true)
                        .trailing_var_arg(true),
                ),
            );
        } else {
            mox = mox.subcommand(
                clap::Command::new(subcommand.as_str())
                    .about(&format!("Execute mox{}ctl", subcommand))
                    .arg(
                        clap::Arg::new("args")
                            .help(&format!("Arguments to pass to mox{}ctl", subcommand))
                            .num_args(0..)
                            .allow_hyphen_values(true)
                            .trailing_var_arg(true),
                    ),
            );
        }
    }

    let matches = mox.get_matches();

    if let Some((subcmd, sub_matches)) = matches.subcommand() {
        let binary_path = if subcmd == "ctl" {
            binaries
                .iter()
                .find(|(subcommand, _)| subcommand.is_empty())
                .map(|(_, path)| path)
        } else {
            binaries
                .iter()
                .find(|(subcommand, _)| subcommand == subcmd)
                .map(|(_, path)| path)
        };

        if let Some(path) = binary_path {
            let args: Vec<&str> = sub_matches
                .get_many::<String>("args")
                .unwrap_or_default()
                .map(|s| s.as_str())
                .collect();

            let status = Command::new(path).args(&args).status()?;

            std::process::exit(status.code().unwrap_or(1));
        } else {
            eprintln!("Binary not found for subcommand: {}", subcmd);
            std::process::exit(1);
        }
    }

    Ok(())
}
