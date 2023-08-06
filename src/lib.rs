use std::process;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;
use winit::dpi::{LogicalPosition, PhysicalSize};
use winit::error::{ExternalError, OsError};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

/// Types of errors that could occur during the [`idle`] function workflow.
#[derive(Debug)]
pub enum IdleError {
    /// Failure in [`EventLoop::primary_monitor`].
    CouldNotAcquireMonitor,
    /// Failure in [`Window::new`].
    CouldNotCreateWindow(OsError),
    /// Failure in [`Window::set_cursor_position`].
    CouldNotMoveCursor(ExternalError),
}

impl std::fmt::Display for IdleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IdleError::CouldNotAcquireMonitor => write!(f, "Could not acquire monitor handle."),
            IdleError::CouldNotCreateWindow(os_error) => {
                write!(f, "Error while creating window: {}", os_error)
            }
            IdleError::CouldNotMoveCursor(external_error) => {
                write!(f, "Could not move mouse cursor: {}", external_error)
            }
        }
    }
}

impl From<OsError> for IdleError {
    fn from(value: OsError) -> Self {
        Self::CouldNotCreateWindow(value)
    }
}

impl From<ExternalError> for IdleError {
    fn from(value: ExternalError) -> Self {
        Self::CouldNotMoveCursor(value)
    }
}

const MIN_DELAY: u8 = 2;
const MAX_DELAY: u8 = 30;

/// ### Moves the mouse cursor randomly around the screen.
///
/// The cursor will move to a random location on the user's monitor on a random delay interval
/// every [`MIN_DELAY`]-[`MAX_DELAY`] seconds. This will occur repeatedly and endlessly until the user closes the program window.
///
/// This function will cause the process to exit if the [`Window::set_cursor_position`] invocation throws an error, since this is irrecoverable
/// and we want the user to immediately know if the main function logic is failing.
pub fn idle() -> Result<(), IdleError> {
    let event_loop = EventLoop::default();
    let monitor = event_loop
        .primary_monitor()
        .ok_or(IdleError::CouldNotAcquireMonitor)?;
    let PhysicalSize {
        width: max_x,
        height: max_y,
    } = monitor.size();
    let window = Window::new(&event_loop)?;
    window.set_title("grater");
    let start_time = std::time::Instant::now();

    // Create separate thread to handle mouse moving
    thread::spawn(move || {
        let mut rng = rand::thread_rng();
        println!("grater is running...");
        loop {
            let rand_x = rng.gen_range(0..max_x);
            let rand_y = rng.gen_range(0..max_y);
            if let Err(e) = window.set_cursor_position(LogicalPosition::new(rand_x, rand_y)) {
                eprintln!("grater crashed: {}", IdleError::CouldNotMoveCursor(e));
                eprintln!(
                    "exiting; grater ran for {}",
                    format_time_duration(Instant::now() - start_time)
                );
                process::exit(1);
            }
            let rand_delay = rng.gen_range(MIN_DELAY..MAX_DELAY);
            thread::sleep(std::time::Duration::from_secs(rand_delay as u64));
        }
    });

    // Main application thread will host a window that, when closed, will print out the elapsed runtime of the app before returning (and thus ending the program).
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        if let Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } = event
        {
            println!(
                "exiting; grater ran for {}",
                format_time_duration(Instant::now() - start_time)
            );
            *control_flow = ControlFlow::Exit
        }
    });
}

/// Generates a "HH:MM:SS" formatted [`String`] for a given [`Duration`].
fn format_time_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let seconds = total_seconds % 60;
    let minutes = (total_seconds / 60) % 60;
    let hours = total_seconds / 3600;
    format!("{:02}:{:02}:{:02}", hours, minutes, seconds)
}
