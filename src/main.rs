use std::{process::exit, time::Duration};

use clap::Parser;
use env_logger::Env;
use lock_guard::LockWatcher;
use log::{debug, info};

/// Shut down your computer automaticly if a lock is running and its unplugged.
#[derive(Debug, clap::Parser, Clone)]
#[command(name = "lock-guard", about)]
struct Args {
    /// The battery to read from. Must exist under /sys/class/power_supply/ and have status
    ///
    /// Setting this to DISABLED will cause the program to immidiatly exit.
    #[arg(short, long, default_value_t = String::from("BAT1"))]
    battery: String,

    /// The processes to watch. This should be your lock process. E.g hyprlock
    #[arg(
        short,
        long,
        value_delimiter = ',',
        default_value = "swaylock,hyprlock"
    )]
    lock_processes: Vec<String>,

    /// The time the program waits before it refreshes.
    #[arg(long, default_value_t = 0.5)]
    poll_rate: f32,
    /// The delay between unplug and shutdown
    #[arg(short, long, default_value_t = 10.0)]
    delay: f32,
}

fn main() {
    env_logger::init_from_env(Env::new().filter("LOG_LEVEL"));

    let args = Args::parse();
    if args.battery == "DISABLED" {
        info!("--battery is set to DISABLED.");
        info!("This causes the program to instantly stop.");
        exit(0);
    }

    debug!("{args:#?}");
    let watcher = LockWatcher {
        lock_process_names: args.lock_processes,
        polling_rate: Duration::from_secs_f32(args.poll_rate),
        time_until_shutdown: Duration::from_secs_f32(args.delay),
        battery_id: args.battery,
    };

    watcher.loop_forever()
}
