// SPDX-License-Identifier: Apache-2.0

//! Manage command line arguments

use crate::diagnostic::DiagnosticCommand;
use crate::registry::RegistryCommand;
use crate::schema::SchemaCommand;
use clap::{Parser, Subcommand};

/// Command line arguments.
#[derive(Parser)]
#[command(
    author,
    version,
    about,
    long_about = None,
    subcommand_required = true,
    arg_required_else_help = true
)]
pub struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    /// Turn the quiet mode on (i.e., minimal output)
    #[arg(short, long)]
    pub quiet: bool,

    /// List of supported commands
    #[command(subcommand)]
    pub command: Option<Commands>,
}

/// Supported commands.
#[derive(Subcommand)]
pub enum Commands {
    /// Manage Semantic Convention Registry
    Registry(RegistryCommand),
    /// Manage Telemetry Schema
    Schema(SchemaCommand),
    /// Manage Diagnostic Messages
    Diagnostic(DiagnosticCommand),
}
