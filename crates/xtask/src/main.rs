//! See <https://github.com/matklad/cargo-xtask/>.
//!
//! This binary defines various auxiliary build commands, which are not
//! expressible with just `cargo`.
//!
//! This binary is integrated into the `cargo` command line by using an alias in
//! `.cargo/config`.

// This crate is a CLI tool and can use stdout and stderr for logging.
#![allow(clippy::print_stdout)]
#![allow(clippy::print_stderr)]

mod history;
mod markdown_help;
mod validate;

#[cfg(not(tarpaulin_include))]
fn main() -> anyhow::Result<()> {
    let task = std::env::args().nth(1);

    match task {
        None => print_help(),
        Some(task) => match task.as_str() {
            "validate" => validate::run(),
            "history" => history::run(std::env::args().nth(2)),
            "markdown_help" => markdown_help::run(),
            "help" => print_help(),
            _ => {
                eprintln!("Unknown task: {}", task);
                print_help()
            }
        },
    }
}

/// Prints help message.
#[cfg(not(tarpaulin_include))]
pub fn print_help() -> anyhow::Result<()> {
    println!(
        "
Usage: Execute the command using `cargo xtask <task>`, e.g., `cargo xtask validate`.

Tasks:
  - history:        Run registry check on semconv models within back compatibility range.
                    Optionally provide a start semver e.g. `history 1.29.0`.
  - markdown_help   Generate markdown help for the weaver CLI.
  - validate:       Validate the entire structure of the weaver project.
"
    );
    Ok(())
}
