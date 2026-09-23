use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::{Duration, Instant};

use apple_cf::cf::CFRunLoop;
use axuielement::ax_notification::AX_CREATED_NOTIFICATION;
use axuielement::{AXObserver, AXUIElement};

fn current_pid() -> i32 {
    i32::try_from(std::process::id()).expect("current pid fits in i32")
}

fn observer_holding(token: &Arc<()>) -> AXObserver {
    let callback_token = Arc::clone(token);
    AXObserver::new(current_pid(), move |_| {
        let _ = &callback_token;
    })
    .expect("observer")
}

#[test]
fn observer_type_id_is_non_zero() {
    assert!(AXObserver::type_id() > 0);
}

#[test]
fn observer_can_be_created_and_scheduled() {
    let observer = AXObserver::new(current_pid(), |_| {}).expect("observer");
    observer.schedule_on_current_run_loop();
    observer.schedule_on_current_run_loop();
    observer.unschedule_from_run_loop();
    observer.unschedule_from_run_loop();
}

#[test]
fn dropping_the_observer_releases_its_callback() {
    let token = Arc::new(());
    let observer = observer_holding(&token);
    assert_eq!(Arc::strong_count(&token), 2);
    observer.schedule_on_current_run_loop();
    drop(observer);
    assert_eq!(Arc::strong_count(&token), 1);

    let callback_token = Arc::clone(&token);
    let observer = AXObserver::new_with_info(current_pid(), move |_| {
        let _ = &callback_token;
    })
    .expect("observer with info");
    drop(observer);
    assert_eq!(Arc::strong_count(&token), 1);
}

#[test]
fn drop_from_another_thread_waits_for_the_owning_run_loop() {
    let token = Arc::new(());
    let running = Arc::new(AtomicBool::new(true));
    let (observer_tx, observer_rx) = mpsc::channel();
    let owner = {
        let token = Arc::clone(&token);
        let running = Arc::clone(&running);
        thread::spawn(move || {
            let observer = observer_holding(&token);
            drop(token);
            observer.schedule_on_current_run_loop();
            observer_tx.send(observer).expect("send observer");
            while running.load(Ordering::SeqCst) {
                let _ = CFRunLoop::run_in_default_mode(Duration::from_millis(20), false);
            }
        })
    };

    let observer = observer_rx.recv().expect("observer from owner thread");
    thread::sleep(Duration::from_millis(50));
    let started = Instant::now();
    drop(observer);
    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(Arc::strong_count(&token), 1);

    running.store(false, Ordering::SeqCst);
    owner.join().expect("owner thread");
}

#[test]
fn drop_after_the_owning_thread_exited() {
    let token = Arc::new(());
    let observer = {
        let token = Arc::clone(&token);
        thread::spawn(move || {
            let observer = observer_holding(&token);
            observer.schedule_on_current_run_loop();
            observer
        })
        .join()
        .expect("owner thread")
    };

    let started = Instant::now();
    drop(observer);
    assert!(started.elapsed() < Duration::from_secs(1));
    assert_eq!(Arc::strong_count(&token), 1);
}

#[test]
fn removing_an_unregistered_notification_is_an_error() {
    let app = AXUIElement::from_pid(current_pid()).expect("application element");
    let mut observer = AXObserver::new(current_pid(), |_| {}).expect("observer");
    assert!(observer
        .remove_notification(&app, AX_CREATED_NOTIFICATION)
        .is_err());
    assert!(observer.remove_notification(&app, "AX\0Created").is_err());
}
