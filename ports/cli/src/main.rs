mod cmds;
mod dtos;

use self::Opts::*;

use anyhow::Result;
use clap::Subcommand;
use classeq_ports_lib::{expose_runtime_arguments, CliLauncher, LogFormat};
use std::{path::PathBuf, str::FromStr};
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Subcommand, Debug)]
#[command(author, version, about, long_about = None)]
enum Opts {
    /// Input/Output commands
    Convert(cmds::convert::Arguments),

    /// Build the index database
    BuildDb(cmds::build_db::Arguments),

    /// Place sequences on the tree
    Place(cmds::place_sequences::Arguments),

    /// Describe the database
    DescribeDb(cmds::describe_db::Arguments),
}

fn main() -> Result<()> {
    let args = CliLauncher::<Opts>::parse();

    // ? -----------------------------------------------------------------------
    // ? Configure logger
    // ? -----------------------------------------------------------------------

    let log_level = args.log_level.unwrap_or("info".to_string());

    let (non_blocking, _guard) = match args.log_file {
        //
        // If no log file is provided, log to stderr
        //
        None => tracing_appender::non_blocking(std::io::stderr()),
        //
        // If a log file is provided, log to the file
        //
        Some(file) => {
            let mut log_file = PathBuf::from(file);

            let binding = log_file.to_owned();
            let parent_dir = binding
                .parent()
                .expect("Log file parent directory not found");

            match args.log_format {
                LogFormat::Jsonl => {
                    log_file.set_extension("jsonl");
                }
                LogFormat::Ansi => {
                    log_file.set_extension("log");
                }
            };

            if log_file.exists() {
                std::fs::remove_file(&log_file)?;
            }

            let file_name =
                log_file.file_name().expect("Log file name not found");

            let file_appender =
                tracing_appender::rolling::never(parent_dir, file_name);

            tracing_appender::non_blocking(file_appender)
        }
    };

    let tracing_config = tracing_subscriber::fmt()
        .event_format(
            fmt::format()
                .with_level(true)
                .with_target(false)
                .with_thread_ids(true)
                .with_file(false)
                .with_line_number(false),
        )
        .with_writer(non_blocking)
        .with_env_filter(EnvFilter::from_str(log_level.as_str()).unwrap());

    match args.log_format {
        LogFormat::Ansi => tracing_config.pretty().init(),
        LogFormat::Jsonl => tracing_config.json().init(),
    };

    // ? -----------------------------------------------------------------------
    // ? Get command line arguments
    // ? -----------------------------------------------------------------------

    expose_runtime_arguments();

    // ? -----------------------------------------------------------------------
    // ? Fire up the command
    // ? -----------------------------------------------------------------------

    match args.opts {
        Convert(io_args) => match io_args.convert {
            cmds::convert::Commands::Tree(tree_args) => {
                cmds::convert::serialize_tree_cmd(tree_args);
            }
            cmds::convert::Commands::Kmers(kmers_args) => {
                cmds::convert::get_kmers_cmd(kmers_args);
            }
            cmds::convert::Commands::Database(db_args) => {
                cmds::convert::convert_database_cmd(db_args)?;
            }
        },
        BuildDb(db_args) => {
            cmds::build_db::build_database_cmd(db_args, args.threads)?;
        }
        Place(place_args) => cmds::place_sequences::place_sequences_cmd(
            place_args,
            args.threads.unwrap_or(1),
        )?,
        DescribeDb(db_args) => {
            cmds::describe_db::describe_database_cmd(db_args)?;
        }
    }

    Ok(())
}
