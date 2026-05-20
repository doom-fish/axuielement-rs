use std::future::poll_fn;
use std::time::{Duration, Instant};

use axuielement::async_api::AXNotificationStream;
use axuielement::ax_notification::AX_MAIN_WINDOW_CHANGED_NOTIFICATION;
use axuielement::{is_process_trusted, AXUIElement};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if !is_process_trusted() {
        println!(
            "Accessibility permission not granted — skipping async notification stream example."
        );
        return Ok(());
    }

    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let app = AXUIElement::from_pid(pid)
        .ok_or_else(|| std::io::Error::other("missing AX application handle"))?;
    let stream =
        match AXNotificationStream::subscribe_many(&app, &[AX_MAIN_WINDOW_CHANGED_NOTIFICATION], 8)
        {
            Ok(stream) => stream,
            Err(error) => {
                println!("Could not subscribe to current-app notifications: {error}");
                return Ok(());
            }
        };

    println!("Waiting up to 2 s for an AX notification on the current app…");

    let event = pollster::block_on(async {
        let start = Instant::now();
        loop {
            if let Some(event) = stream.try_next() {
                return Some(event);
            }
            if start.elapsed() >= Duration::from_secs(2) {
                return None;
            }
            poll_fn(|cx| {
                cx.waker().wake_by_ref();
                std::task::Poll::Ready(())
            })
            .await;
        }
    });

    match event {
        Some(event) => println!(
            "notification={} element={:?}",
            event.notification, event.element
        ),
        None => println!("No notification arrived within the timeout — that's fine."),
    }

    println!("Stream dropped cleanly.");
    Ok(())
}
