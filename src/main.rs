mod capture;
mod edge_detection;
mod ui;
mod wayland_handlers;

use capture::capture_screen;
use wayland_client::Connection;
use wayland_handlers::WaylandApp;

fn main() {
    let conn = Connection::connect_to_env().expect("Failed to connect to Wayland");

    let screenshot = match capture_screen(&conn) {
        Ok(s) => s,
        Err(_) => std::process::exit(1),
    };

    let (mut app, mut event_queue) = WaylandApp::new(&conn, screenshot);
    let qh = event_queue.handle();

    app.create_surface(&qh);

    while !app.should_exit() {
        event_queue.blocking_dispatch(&mut app).unwrap();
    }
}
