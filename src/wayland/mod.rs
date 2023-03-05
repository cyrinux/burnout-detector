use chan::chan_select;
use std::error::Error;
use std::thread;
use std::time::Duration;
use wayland_client::EventQueue;
use wayland_client::{
    protocol::wl_registry, protocol::wl_seat, protocol::wl_seat::WlSeat, Connection, Dispatch,
    QueueHandle,
};
use wayland_protocols_plasma::idle::client::org_kde_kwin_idle::OrgKdeKwinIdle;
use wayland_protocols_plasma::idle::client::{org_kde_kwin_idle, org_kde_kwin_idle_timeout};

use crate::logic::Logic;
use crate::Args;

/// State of the app
#[derive(Debug)]
pub struct Wayland {
    /// Wayland seat
    seat: Option<WlSeat>,
    /// KDE idle
    kde_idle: Option<OrgKdeKwinIdle>,
    /// Logic struct
    logic: Logic,
    /// Idle detection timeout
    timeout: u32,
}

/// Wayland events
#[derive(Debug)]
pub struct Events {
    conn: Connection,
    event_queue: EventQueue<Wayland>,
}

impl Events {
    pub fn new() -> Result<Events, Box<dyn Error>> {
        let conn = Connection::connect_to_env().expect("Can't connect to wayland");
        let display = conn.display();
        let event_queue = conn.new_event_queue();
        let queue_handle = event_queue.handle();
        let _registry = display.get_registry(&queue_handle, ());

        Ok(Events { conn, event_queue })
    }
}

impl Wayland {
    pub fn new(args: &Args) -> Result<Wayland, ()> {
        let logic = Logic::new(args).expect("Can't initialize logic core");
        let timeout = if args.idle_timeout < 60 {
            // if idle_timeout less than 60s,
            // make wayland timeout 1s less
            (args.idle_timeout as u32 - 1) * 1000
        } else {
            // or use 60s as timeout detection
            60 * 1000
        };

        Ok(Wayland {
            seat: None,
            kde_idle: None,
            logic,
            timeout,
        })
    }

    pub fn run(mut self) -> ! {
        let mut events = Events::new().expect("Can't initialize events");

        let tick_dispatch = chan::tick(Duration::from_millis(500));
        let tick_compute = chan::tick(Duration::from_millis(1000));
        loop {
            chan_select! {
                default => {
                    thread::sleep(Duration::from_millis(100));
                },
                tick_dispatch.recv() => {
                    events.event_queue.dispatch_pending(&mut self).expect("Can't dispatch wayland events");
                    // flush and read events
                    events.conn.flush().is_ok().then(|| {
                        events.conn.prepare_read().expect("Can't prepare wayland read").read()
                    });
                },
                tick_compute.recv() => {
                    self.logic.run().expect("Can't run the main idle analyze job");
                },
            }
        }
    }
}

#[allow(clippy::identity_op)]
impl Dispatch<wl_registry::WlRegistry, ()> for Wayland {
    fn event(
        wayland: &mut Self,
        registry: &wl_registry::WlRegistry,
        event: wl_registry::Event,
        _: &(),
        _: &Connection,
        qh: &QueueHandle<Wayland>,
    ) {
        if let wl_registry::Event::Global {
            name,
            interface,
            version,
        } = event
        {
            match &interface[..] {
                "wl_seat" => {
                    wayland.seat =
                        Some(registry.bind::<wl_seat::WlSeat, _, _>(name, version, qh, ()));
                    wayland.kde_idle.as_ref().and_then(|kde_idle| {
                        wayland
                            .seat
                            .as_ref()
                            .map(|seat| kde_idle.get_idle_timeout(seat, wayland.timeout, qh, ()))
                    });
                }
                "org_kde_kwin_idle" => {
                    wayland.kde_idle =
                        Some(registry.bind::<org_kde_kwin_idle::OrgKdeKwinIdle, _, _>(
                            name,
                            version,
                            qh,
                            (),
                        ));
                    wayland.seat.as_ref().and_then(|seat| {
                        wayland.kde_idle.as_ref().map(|kde_idle| {
                            kde_idle.get_idle_timeout(seat, wayland.timeout, qh, ())
                        })
                    });
                }
                _ => {}
            }
        }
    }
}

impl Dispatch<org_kde_kwin_idle_timeout::OrgKdeKwinIdleTimeout, ()> for Wayland {
    fn event(
        wayland: &mut Self,
        _: &org_kde_kwin_idle_timeout::OrgKdeKwinIdleTimeout,
        event: org_kde_kwin_idle_timeout::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
        match event {
            org_kde_kwin_idle_timeout::Event::Resumed => {
                wayland.logic.set_resumed();
            }
            org_kde_kwin_idle_timeout::Event::Idle => {
                wayland.logic.set_idle();
            }
            _ => {}
        }
    }
}

impl Dispatch<wl_seat::WlSeat, ()> for Wayland {
    fn event(
        _: &mut Self,
        _: &wl_seat::WlSeat,
        _: wl_seat::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}

impl Dispatch<org_kde_kwin_idle::OrgKdeKwinIdle, ()> for Wayland {
    fn event(
        _: &mut Self,
        _: &org_kde_kwin_idle::OrgKdeKwinIdle,
        _: org_kde_kwin_idle::Event,
        _: &(),
        _: &Connection,
        _: &QueueHandle<Self>,
    ) {
    }
}
