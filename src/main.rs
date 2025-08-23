use clap::Parser;
use rayon::prelude::*;
use std::{fs, path::Path, process::Command};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {}

fn find_paths() -> anyhow::Result<Vec<String>> {
    let path = std::env::var("PATH")?;
    Ok(path
        .split(':')
        .par_bridge()
        .flat_map_iter(|p| {
            fs::read_dir(p).ok().into_iter().flat_map(|dir| {
                dir.filter_map(Result::ok)
                    .filter(|entry| {
                        entry.file_name().to_string_lossy().starts_with("mox")
                            && entry.file_name().to_string_lossy().ends_with("ctl")
                    })
                    .map(|entry| entry.path().display().to_string())
            })
        })
        .collect())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut mox = clap::Command::new("mox");

    let paths = find_paths()?;
    for path in &paths {
        let file_name = Path::new(&path).file_name().unwrap().to_string_lossy();

        let subcommand = file_name
            .strip_prefix("mox")
            .and_then(|s| s.strip_suffix("ctl"))
            .unwrap_or("")
            .to_string();

        let subcommand_static: &'static str = Box::leak(subcommand.into_boxed_str());
        mox = mox.subcommand(
            clap::Command::new(subcommand_static).arg(
                clap::Arg::new("args")
                    .num_args(0..)
                    .allow_hyphen_values(true),
            ),
        );
    }

    let matches = mox.clone().get_matches();

    if let Some((subcmd, _sub_matches)) = matches.subcommand() {
        if let Some(path) = paths
            .iter()
            .find(|p| p.ends_with(&format!("mox{}ctl", subcmd)))
        {
            let args: Vec<String> = std::env::args().skip(2).collect();
            println!("{subcmd}");

            Command::new(path).args(&args).spawn()?.wait()?;
        }
    } else {
        mox.print_help()?;
    }

    Ok(())
}
