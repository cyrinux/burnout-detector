use hhmmss::Hhmmss;
use notify_rust::{Hint, Notification, Urgency};
use std::time::Duration;

use crate::helpers::*;
use crate::status::Status;
use crate::Args;

const MIN_NOTIFICATION_TIME: Duration = Duration::from_secs(15);
const MAX_RECOMMANDED_PAUSE_TIME: Duration = Duration::from_secs(600);

#[derive(Debug)]
pub struct Alert {
    /// App arguments
    pub args: Args,
    /// Notification urgency
    pub urgency: Urgency,
    /// Number of notification sent
    pub counter_sent: u64,
    /// Next notification time
    pub next_send_time: Duration,
    /// Active start at this time
    pub notification_delay_secs: Duration,
}

impl Alert {
    pub fn send_notification(&mut self, urgency: Urgency, status: &Status) -> bool {
        match status {
            Status::Active(start, elapsed) => self.notification_active(elapsed, start, urgency),
            Status::Idle(_, _) => false,
        }
    }

    fn notification_active(
        &mut self,
        elapsed: &Duration,
        start: &std::time::Instant,
        urgency: Urgency,
    ) -> bool {
        let should_send = !self.args.no_notify && elapsed >= &self.next_send_time;
        if !should_send {
            return false;
        }

        self.notification_delay_secs = self.notification_delay_secs.max(MIN_NOTIFICATION_TIME);

        let should_harass =
            *elapsed > Duration::from_secs(self.args.max_active_sessions * self.args.idle_timeout);

        if should_harass && self.notification_delay_secs >= MIN_NOTIFICATION_TIME * 2 {
            self.notification_delay_secs /= 2;
        }

        self.next_send_time = *elapsed + self.notification_delay_secs;

        let pause_time =
            Duration::from_secs(self.args.idle_timeout / 2).max(MAX_RECOMMANDED_PAUSE_TIME);

        self.counter_sent += 1;

        if self.args.debug {
            eprintln!(
                "Current session start at {:?}.\nA {}x is notification is send, the next should be in {}.",
                start,
                self.counter_sent,
                self.next_send_time.hhmmss()
            );
        }

        Notification::new()
                    .summary(&format!("Burnout detector ({}x)", self.counter_sent))
                    .body(&format!(
                        "You didn't take a break for {}\nYou should take a <b>{}</b> break and do some <b>gym</b> exercice!\n\n<b>{:?}</b>",
                       elapsed.hhmmss(),
                       pause_time.hhmmss(),
                       get_random_gymnastic(),
                    ))
                    .icon("media-playback-pause-symbolic")
                    .appname("burnout_detector")
                    .hint(Hint::Urgency(urgency))
                    .timeout(0)
                    .show().is_ok()
    }

    pub fn reset_notifications(&mut self) {
        self.reset_next_send_time();

        self.counter_sent = 0;
    }

    pub fn reset_next_send_time(&mut self) {
        self.notification_delay_secs =
            Duration::from_secs(self.args.idle_timeout * self.args.max_active_sessions);

        self.next_send_time = if self.args.waybar {
            self.notification_delay_secs
        } else {
            Duration::from_secs(0)
        };
    }
}
