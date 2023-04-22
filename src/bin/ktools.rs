use clap::Parser;

use klenkiven_toolkit::{args::Commands, commands::cf_ddns};
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[derive(Parser, Debug)]
#[command(name = "ktools")]
#[command(author = "KlenKiven <wzl709@outlook.com>")]
#[command(version = "0.1.0")]
#[command(about = "KTools is a toolkit collection that help you to build your own server.")]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    debug: u8,

    /// KTools' command
    #[command(subcommand)]
    command: Option<Commands>,
}

fn main() {
    let cli = Cli::parse();

    // You can see how many times a particular flag or argument occurred
    // Note, only flags can have multiple occurrences
    let log_level = match cli.debug {
        0 => Level::INFO,
        1 | 2 => Level::TRACE,
        _ => Level::INFO,
    };

    // Init Tracing Log
    let subscriber = FmtSubscriber::builder()
        // Set log level for subscriber
        .with_max_level(log_level)
        // Use a more compact, abbreviated log format
        .compact()
        // Display source code file paths and code line numbsers
        .with_file(false)
        .with_line_number(false)
        // Display the thread ID an event was recorded on
        .with_thread_ids(false)
        // Don't display the event's target (module path)
        .with_target(false)
        // Build the subscriber
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber);

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(commands) => {
            match commands {
                // If the commands is CfDdns, and use the
                Commands::CfDdns {
                    token,
                    zone_name,
                    record_name,
                    record_type,
                    ttl,
                    proxied,
                    force,
                } => {
                    // Format token as Bearer token
                    let token = format!("Bearer {}", token);
                    cf_ddns(&token, zone_name, record_name, record_type, ttl, proxied, force);
                }
            }
        }
        None => {}
    }
}
