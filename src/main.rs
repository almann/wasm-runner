use anyhow::{Error, Result};
use indoc::indoc;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use which::which;

fn help<S: AsRef<str>>(name: S) {
    eprintln!(
        indoc! {"
        USAGE:
            {} <RUNTIME-CMD> <WASM-PROGRAM> [<ARGUMENT>...]
        "},
        name.as_ref()
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    // note that we don't use something clap to keep the processing as pass through as possible
    match args.len() {
        1 => {
            help(args[0].as_str());
            return Err(Error::msg("Need to specify WASM runtime command"));
        }
        2 => {
            help(args[0].as_str());
            return Err(Error::msg("Need to specify WASM program file"));
        }
        _ => {
            let runtime = args[1].as_str();
            let program = args[2].as_str();
            let app_args: Vec<&str> = (&args[3..]).iter().map(String::as_str).collect();

            // look for the runtime in our search path
            let runtime_path = if let Ok(path) = which(runtime) {
                path
            } else {
                // get the "home" directory
                let home = match (env::var("HOME"), env::var("USERPROFILE")) {
                    (Ok(dir), _) => dir,
                    (_, Ok(dir)) => dir,
                    (_, _) => {
                        return Err(Error::msg(
                            "Could not find user home directory to autodetect WASM runtime",
                        ))
                    }
                };
                let mut path = PathBuf::from(home);

                match runtime {
                    "wasmer" => path.push(".wasmer"),
                    "wasmtime" => path.push(".wasmtime"),
                    _ => {
                        return Err(Error::msg(format!(
                            "Could not autodetect WASM runtime for '{}'",
                            runtime
                        )))
                    }
                }
                path.push("bin");
                path.push(runtime);

                if let Ok(path) = which(path.as_path()) {
                    path
                } else {
                    return Err(Error::msg(format!(
                        "Could not find WASM runtime for '{:?}'",
                        path.as_os_str()
                    )));
                }
            };

            // construct our wrapped command
            // TODO this is known to work with wasmtime and wasmer--probably should be smarter about it.
            let runtime_args: Vec<&str> =
                ["run", program, "--"].into_iter().chain(app_args).collect();

            Command::new(runtime_path)
                .args(runtime_args)
                .spawn()
                .expect("Could not start runtime")
                .wait()?;
        }
    }

    Ok(())
}
