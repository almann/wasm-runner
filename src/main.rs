use anyhow::{Error, Result};
use serde_json::Value;
use std::env;
use std::path::PathBuf;
use std::process::Command;
use which::which;

fn help<S: AsRef<str>>(name: S) {
    eprintln!(
        "USAGE:\n    {} <RUNTIME-CMD> <WASM-PROGRAM> [<ARGUMENT>...]",
        name.as_ref()
    );
}

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let verbose = env::var("WASM_RUNNER_VERBOSE").is_ok();

    if verbose {
        eprintln!("WASM runner arguments {:?}", args);
    }

    let runtime_add_args: Vec<String> = if let Ok(args_text) = env::var("WASM_RUNNER_RT_ARGS") {
        let args_value = serde_json::from_str(args_text.as_str())?;
        if let Value::Array(args_list) = args_value {
            args_list
                .into_iter()
                .map(|v| {
                    let res: Result<String> = if let Value::String(text) = v {
                        Ok(text)
                    } else {
                        Err(Error::msg(format!(
                            "WASM invalid runtime argument: {:?}",
                            v
                        )))
                    };
                    res
                })
                .collect::<Result<Vec<String>>>()?
        } else {
            return Err(Error::msg(format!(
                "WASM runner invalid additional runtime arguments {:?}",
                args_text
            )));
        }
    } else {
        vec![]
    };
    if !runtime_add_args.is_empty() && verbose {
        eprintln!("WASM runner additional runtime args {:?}", runtime_add_args);
    }

    // note that we don't use something like clap to keep the processing as pass through as possible
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
                if verbose {
                    eprintln!("WASM runtime found at {:?}", path);
                }
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
                            "Could not auto-detect WASM runtime for {:?}",
                            runtime
                        )))
                    }
                }
                path.push("bin");
                path.push(runtime);

                if let Ok(path) = which(path.as_path()) {
                    if verbose {
                        eprintln!("WASM runtime found at (auto-detect): {:?}", path);
                    }
                    path
                } else {
                    return Err(Error::msg(format!(
                        "Could not find WASM runtime for {:?}",
                        path.as_os_str()
                    )));
                }
            };

            // construct our wrapped command
            // TODO this is known to work with wasmtime and wasmer--probably should be smarter about it.
            let runtime_args: Vec<&str> = vec!["run", program]
                .into_iter()
                .chain(runtime_add_args.iter().map(String::as_str))
                .chain(vec!["--"])
                .chain(app_args)
                .collect();

            if verbose {
                eprintln!("WASM runtime arguments {:?}", runtime_args);
            }

            Command::new(runtime_path)
                .args(runtime_args)
                .spawn()
                .expect("Could not start runtime")
                .wait()?;
        }
    }

    Ok(())
}
