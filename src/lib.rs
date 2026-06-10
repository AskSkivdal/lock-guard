use std::{
    ffi::OsStr,
    fs::File,
    io::Read,
    path::PathBuf,
    process::Command,
    thread,
    time::{Duration, Instant},
};

use log::{debug, error, info, warn};
use sysinfo::{ProcessRefreshKind, RefreshKind, System};

pub enum PowerState {
    /// Plugged in
    Charging,
    /// Not plugged in
    Discharging,
}

impl PowerState {
    /// Checks the status for a battery. If the battery could not be checked, then it returns PowerState::Discharging.
    ///
    /// Supported statuses:
    ///  - "Charging" - Plugged in.
    ///  - "Not Charging" - Plugged in, but not charging.
    ///  - "Discharging" - Not plugged in.
    fn current(battery_name: &String) -> Self {
        let battery_path = PathBuf::from(format!("/sys/class/power_supply/{battery_name}/status"));

        let mut battery_status = format!("Failed to read from {battery_path:?}");
        {
            let _ = File::open(battery_path).and_then(|mut x| {
                battery_status.clear();
                x.read_to_string(&mut battery_status)
            });
        }

        let trimmed_status = battery_status.trim().to_lowercase();
        let trimmed_status = trimmed_status.as_str();

        match trimmed_status {
            "charging" => PowerState::Charging,
            "full" => PowerState::Charging,
            "not charging" => PowerState::Charging,
            "discharging" => PowerState::Discharging,
            _ => {
                warn!("Powerstate: {trimmed_status} unknown. Returning Discharging");
                PowerState::Discharging
            }
        }
    }
}

pub struct LockWatcher {
    pub lock_process_names: Vec<String>,
    pub battery_id: String,
    pub polling_rate: Duration,
    pub time_until_shutdown: Duration,
}

impl LockWatcher {
    /// Check if any of the lock processes are running
    fn system_locked(&self) -> bool {
        let system = System::new_with_specifics(
            RefreshKind::nothing().with_processes(ProcessRefreshKind::everything()),
        );

        for process_name in &self.lock_process_names {
            if system
                .processes_by_name(OsStr::new(process_name))
                .next()
                .is_some()
            {
                debug!("{process_name} is running");
                return true;
            }
        }

        false
    }

    /// Get the current power state
    fn get_power_state(&self) -> PowerState {
        PowerState::current(&self.battery_id)
    }

    pub fn loop_forever(&self) -> ! {
        let mut past_power_state: PowerState = self.get_power_state();
        let mut started_discharging_at: Option<Instant> = None;

        loop {
            thread::sleep(self.polling_rate);

            if !self.system_locked() {
                past_power_state = self.get_power_state();
                continue;
            }

            match (&past_power_state, self.get_power_state()) {
                // NOOP
                (PowerState::Charging, PowerState::Charging) => {
                    started_discharging_at = None;
                }
                // Device unplugged
                (PowerState::Charging, PowerState::Discharging) => {
                    started_discharging_at = Some(Instant::now());
                    past_power_state = PowerState::Discharging;
                    info!("Disconnected")
                }
                // Device plugged in
                (PowerState::Discharging, PowerState::Charging) => {
                    started_discharging_at = None;
                    past_power_state = PowerState::Charging;
                    info!("Plugged in")
                }
                // Noop
                (PowerState::Discharging, PowerState::Discharging) => {}
            }

            if let Some(discharging_since) = started_discharging_at {
                let elapsed = discharging_since.elapsed();
                let until_shutdown = self.time_until_shutdown.as_secs_f32() - elapsed.as_secs_f32();
                info!("Time until shutdown: {:.2} seconds", until_shutdown);

                if elapsed > self.time_until_shutdown {
                    info!("Shutting down");

                    match Command::new("shutdown").arg("now").output() {
                        Ok(_) => {}
                        Err(v) => error!("Could not shut down: {:?}", v),
                    }

                    thread::sleep(Duration::from_secs_f32(2.0));
                }
            }
        }
    }
}
