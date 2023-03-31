use hhmmss::Hhmmss;
use notify_rust::Urgency;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::time::{Duration, Instant};

mod alert;

use crate::logic::alert::Alert;
use crate::Args;

/// Waybar output struct
#[derive(Serialize, Deserialize, Debug)]
struct WaybarOutput {
    text: String,
    class: String,
    tooltip: String,
}

/// Status of the app
#[derive(Debug)]
pub enum Status {
    Active(Instant, Duration),
    Idle(Instant, Duration),
}

/// Logic of the app
#[derive(Debug)]
pub struct Logic {
    /// App arguments
    args: Args,
    /// Status
    pub status: Status,
    /// Number of actual eyes display
    eyes_actual: Vec<String>,
    /// Waybar output
    waybar_output: WaybarOutput,
    /// Popup alert
    pub alert: Alert,
}

static STATUS_OK: &str = "ok";
static STATUS_WARNING: &str = "warning";
static STATUS_CRITICAL: &str = "critical";

impl Logic {
    pub fn new(args: &Args) -> Result<Logic, ()> {
        let next_send_time = if args.waybar {
            Duration::from_secs(args.idle_timeout * (args.max_active_sessions + 1))
        } else {
            Duration::from_secs(0)
        };

        let notification_delay_secs = if args.waybar {
            Duration::from_secs(args.idle_timeout * args.max_active_sessions)
        } else {
            Duration::from_secs(args.idle_timeout)
        };

        let alert = Alert {
            args: args.clone(),
            next_send_time,
            counter_sent: 0,
            notification_delay_secs,
            urgency: Urgency::Low,
            quiet: false,
        };

        let waybar_output = WaybarOutput {
            text: "".to_owned(),
            class: "ok".to_owned(),
            tooltip: "".to_owned(),
        };

        Ok(Logic {
            args: args.clone(),
            status: Status::Active(Instant::now(), Duration::from_secs(0)),
            alert,
            eyes_actual: Vec::new(),
            waybar_output,
        })
    }

    pub fn set_resumed(&mut self) {
        self.status = Status::Active(Instant::now(), Duration::from_secs(0));
        self.alert.reset_notifications();
    }

    pub fn set_idle(&mut self) {
        self.status = Status::Idle(Instant::now(), Duration::from_secs(0));
        self.alert.reset_notifications();
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.update()?;
        self.run_on_state()?;
        self.show_debug();
        self.show_waybar()?;
        Ok(())
    }

    fn run_on_state(&mut self) -> Result<(), Box<dyn Error>> {
        match self.status {
            Status::Active(_start, elapsed) => self.compute_active(elapsed)?,
            Status::Idle(_start, elapsed) => self.compute_idle(elapsed)?,
        }
        Ok(())
    }

    fn show_waybar(&mut self) -> Result<bool, Box<dyn Error>> {
        if !self.args.waybar {
            return Ok(false);
        }

        println!("{}", serde_json::to_string(&self.waybar_output)?);

        Ok(true)
    }

    fn update(&mut self) -> Result<(), Box<dyn Error>> {
        self.status = match self.status {
            Status::Active(start, _) => Status::Active(start, Instant::now() - start),
            Status::Idle(start, _) => Status::Idle(start, Instant::now() - start),
        };

        Ok(())
    }

    fn compute_active(&mut self, elapsed: Duration) -> Result<(), Box<dyn Error>> {
        let expected_number_of_eyes = self
            .args
            .max_active_sessions
            .min(elapsed.as_secs() / self.args.idle_timeout);

        self.eyes_actual = (0..expected_number_of_eyes)
            .map(|_| self.args.icon.to_string())
            .collect();

        let to_notify = if self.args.waybar {
            expected_number_of_eyes == self.args.max_active_sessions
        } else {
            elapsed.as_secs() >= self.args.idle_timeout
        };

        if to_notify {
            let urgency = if expected_number_of_eyes < self.args.max_active_sessions {
                Urgency::Normal
            } else {
                Urgency::Critical
            };

            self.alert.send_notification(urgency, &self.status);
        }

        self.waybar_output.class = if expected_number_of_eyes == 0 {
            STATUS_OK.to_string()
        } else if expected_number_of_eyes < self.args.max_active_sessions {
            STATUS_WARNING.to_string()
        } else {
            STATUS_CRITICAL.to_string()
        };

        self.waybar_output.tooltip = format!("You didn't take a break for {}", elapsed.hhmmss());
        self.waybar_output.text = self.eyes_actual.join(" ");

        Ok(())
    }

    fn compute_idle(&mut self, elapsed: Duration) -> Result<(), Box<dyn Error>> {
        let max_eyes = self.args.max_active_sessions;
        let max_seconds = max_eyes * self.args.idle_timeout;

        let new_eyes_to_remove = (elapsed.as_secs() * max_eyes) / max_seconds;
        let new_eyes = max_eyes - new_eyes_to_remove;

        self.eyes_actual = (0..new_eyes).map(|_| self.args.icon.to_string()).collect();

        self.waybar_output.class = STATUS_OK.into();
        self.waybar_output.tooltip = format!("You are idle since {}", elapsed.hhmmss());
        self.waybar_output.text = self.eyes_actual.join(" ");
        Ok(())
    }

    fn show_debug(&self) {
        if !self.args.debug {
            return;
        }
        match self.status {
            Status::Idle(_, elapsed) => {
                eprintln!("Is idle since {}", elapsed.hhmmss());
            }
            Status::Active(_, elapsed) => {
                eprintln!("Is active since {}", elapsed.hhmmss());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waybar_output_active_eye_increase_over_time() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;
        let start = Instant::now();
        let mut eyes: Vec<String> = vec![];

        assert_eq!(logic.waybar_output.class, STATUS_OK);

        logic.status = Status::Active(start, Duration::from_secs(0));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, "");

        logic.status = Status::Active(start, Duration::from_secs(60));
        assert_eq!(logic.run_on_state().is_ok(), true);
        eyes.push(args.icon.to_string());
        assert_eq!(logic.waybar_output.class, STATUS_WARNING);
        assert_eq!(logic.waybar_output.text, eyes.join(" "));

        logic.status = Status::Active(start, Duration::from_secs(120));
        assert_eq!(logic.run_on_state().is_ok(), true);
        eyes.push(args.icon.to_string());
        assert_eq!(logic.waybar_output.class, STATUS_WARNING);
        assert_eq!(logic.waybar_output.text, eyes.join(" "));

        logic.status = Status::Active(start, Duration::from_secs(180));
        assert_eq!(logic.run_on_state().is_ok(), true);
        eyes.push(args.icon.to_string());
        assert_eq!(logic.waybar_output.class, STATUS_CRITICAL);
        assert_eq!(logic.waybar_output.text, eyes.join(" "));

        logic.status = Status::Active(start, Duration::from_secs(240));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_CRITICAL);
        assert_eq!(logic.waybar_output.text, eyes.join(" "));
    }

    #[test]
    fn test_waybar_output_idle_eyes_decrease_over_time_loose_one_eye_after_60s() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;

        logic.eyes_actual = vec![args.icon.clone(), args.icon.clone(), args.icon.clone()];
        logic.status = Status::Idle(Instant::now(), Duration::from_secs(60));

        let expected_waybar = format!("{} {}", args.icon, args.icon);
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, expected_waybar);
    }

    #[test]
    fn test_waybar_output_during_idle_timeout_times_max_active_sessions_seconds_120s() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;

        logic.eyes_actual = vec![args.icon.clone(), args.icon.clone(), args.icon.clone()];
        logic.status = Status::Idle(Instant::now(), Duration::from_secs(120));

        let expected_waybar = format!("{}", args.icon);
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, expected_waybar);
    }

    #[test]
    fn test_with_waybar_output_during_idle_timeout_times_max_active_sessions_seconds_120s_then_active_60s(
    ) {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;

        logic.eyes_actual = vec![args.icon.clone(), args.icon.clone(), args.icon.clone()];
        logic.status = Status::Idle(Instant::now(), Duration::from_secs(120));

        let expected_waybar = format!("{}", args.icon);
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, expected_waybar);

        logic.status = Status::Active(Instant::now(), Duration::from_secs(60));
        let expected_waybar = format!("{}", args.icon);
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_WARNING);
        assert_eq!(logic.waybar_output.text, expected_waybar);
    }

    #[test]
    fn test_waybar_output_during_idle_timeout_that_we_dont_remove_too_fast_eyes() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;
        let start = Instant::now();
        logic.eyes_actual = vec![args.icon.clone(), args.icon.clone(), args.icon.clone()];

        let expected_waybar = format!("{} {}", args.icon, args.icon);
        logic.status = Status::Idle(start, Duration::from_secs(119));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, expected_waybar);

        let expected_waybar = format!("{}", args.icon);
        logic.status = Status::Idle(start, Duration::from_secs(120));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, expected_waybar);

        let expected_waybar = format!("{}", args.icon);
        logic.status = Status::Idle(start, Duration::from_secs(121));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, expected_waybar);
    }

    #[test]
    fn test_waybar_output_during_idle_timeout_no_more_eyes_at_max_sessions() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;
        let start = Instant::now();

        logic.eyes_actual = vec![args.icon.clone(), args.icon.clone(), args.icon.clone()];
        logic.status = Status::Idle(start, Duration::from_secs(180));

        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.waybar_output.class, STATUS_OK);
        assert_eq!(logic.waybar_output.text, "");
    }

    #[test]
    fn test_notifications_with_waybar() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };

        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;
        let start = Instant::now();

        // after 0s
        logic.status = Status::Active(start, Duration::from_secs(0));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(240));

        // after 60s
        logic.status = Status::Active(start, Duration::from_secs(60));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(240));

        // after 120s
        logic.status = Status::Active(start, Duration::from_secs(120));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(240));

        // after 180s
        logic.status = Status::Active(start, Duration::from_secs(180));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(240));

        // after 240s
        logic.status = Status::Active(start, Duration::from_secs(240));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(330));

        // after 242s
        logic.status = Status::Active(start, Duration::from_secs(242));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(330));

        // after 243s
        logic.status = Status::Active(start, Duration::from_secs(243));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(330));

        // after 330s
        logic.status = Status::Active(start, Duration::from_secs(330));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 2);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(375));

        // after 375s, this start to be more aggressive
        logic.status = Status::Active(start, Duration::from_secs(375));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 3);
        assert_eq!(logic.alert.next_send_time, Duration::from_millis(397500));

        // after 397.5s, this start to be more aggressive
        logic.status = Status::Active(start, Duration::from_millis(397500));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 4);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(420));
    }

    #[test]
    fn test_notifications_without_waybar() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: false,
            max_active_sessions: 3,
            no_notify: false,
            icon: "O".to_owned(),
        };
        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;
        let start = Instant::now();
        // after 0s
        logic.status = Status::Active(start, Duration::from_secs(0));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(0));

        // after 1s
        logic.status = Status::Active(start, Duration::from_secs(1));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(0));

        // after 60s
        logic.status = Status::Active(start, Duration::from_secs(60));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(120));

        // after 61s
        logic.status = Status::Active(start, Duration::from_secs(61));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(120));

        // after 62s
        logic.status = Status::Active(start, Duration::from_secs(62));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(120));

        // after 90s
        logic.status = Status::Active(start, Duration::from_secs(90));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 1);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(120));

        // after 120s
        logic.status = Status::Active(start, Duration::from_secs(120));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 2);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(180));

        // after 130s
        logic.status = Status::Active(start, Duration::from_secs(130));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 2);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(180));

        // after 180s
        logic.status = Status::Active(start, Duration::from_secs(180));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 3);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(240));

        // after 240s, we start the harassment from here
        logic.status = Status::Active(start, Duration::from_secs(240));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 4);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(270));

        // after 270s
        logic.status = Status::Active(start, Duration::from_secs(270));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 5);
        assert_eq!(logic.alert.next_send_time, Duration::from_secs(285));
    }

    #[test]
    fn test_notifications_with_notification_disabled() {
        let args = Args {
            debug: false,
            idle_timeout: 60,
            waybar: true,
            max_active_sessions: 3,
            no_notify: true,
            icon: "O".to_owned(),
        };
        let start = Instant::now();
        let mut logic = Logic::new(&args).unwrap();
        logic.alert.quiet = true;

        logic.status = Status::Active(start, Duration::from_secs(270));
        assert_eq!(logic.run_on_state().is_ok(), true);
        assert_eq!(logic.alert.counter_sent, 0);
    }
}
