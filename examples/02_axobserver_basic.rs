//! Create an `AXObserver`, optionally register one notification, then unschedule it.

use axuielement::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let mut observer = AXObserver::new(pid, |event| {
        println!(
            "notification={} element={:?}",
            event.notification, event.element
        );
    })?;
    println!("observer_type_id={}", AXObserver::type_id());

    if let Some(app) = AXUIElement::from_pid(pid) {
        if let Err(error) =
            observer.add_notification(&app, axuielement::ax_notification::AX_CREATED_NOTIFICATION)
        {
            eprintln!("notification registration skipped: {error}");
        }
    }

    observer.schedule_on_current_run_loop();
    observer.unschedule_from_run_loop();
    Ok(())
}
