use clap::Parser;

mod alert;
mod helpers;
mod logic;
mod status;
mod wayland;
use wayland::Wayland;

/// Command line parameters
#[derive(Parser, Debug, Clone)]
#[clap(author, version, about)]
pub struct Args {
    /// Debug param
    #[arg(short, long, default_value_t = false)]
    debug: bool,
    /// Change the waybar eye icon
    #[arg(long, default_value = "")]
    icon: String,
    /// Idle timeout in seconds
    #[arg(short, long, default_value_t = 600)]
    idle_timeout: u64,
    /// The maximum of active session before being notify
    #[arg(short, long, default_value_t = 3)]
    max_active_sessions: u64,
    /// Disable the notification
    #[arg(short, long, default_value_t = false)]
    no_notify: bool,
    /// Enable waybar module output
    #[arg(short, long, default_value_t = false)]
    waybar: bool,
}

fn main() {
    let args = Args::parse();
    if args.debug {
        eprintln!("Params: {args:?}");
    }

    if !args.waybar && args.no_notify {
        eprintln!("You cannot disable waybar and notification in the same time");
        std::process::exit(1);
    } else if !args.no_notify && args.waybar {
        eprintln!("Waybar and notification mode");
    } else if args.no_notify && args.waybar {
        eprintln!("Waybar only mode");
    } else {
        eprintln!("Notification only mode");
    }

    Wayland::new(&args)
        .expect("Can't initialize the application")
        .run();
}
