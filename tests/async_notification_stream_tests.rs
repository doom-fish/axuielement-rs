use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use axuielement::async_api::AXNotificationStream;
use axuielement::AXUIElement;

fn current_app() -> AXUIElement {
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    AXUIElement::from_pid(pid).expect("AX application handle")
}

#[test]
fn async_stream_can_subscribe_without_notifications() {
    let app = current_app();
    let stream = AXNotificationStream::subscribe_many(&app, &[], 4).expect("async stream");
    assert_eq!(stream.buffered_count(), 0);
    assert!(!stream.is_closed());
}

#[test]
fn async_stream_rejects_zero_capacity() {
    let app = current_app();
    assert!(AXNotificationStream::subscribe_many(&app, &[], 0).is_err());
}

#[test]
fn dropping_a_just_created_stream_never_hangs() {
    let (done_tx, done_rx) = mpsc::channel();
    thread::spawn(move || {
        let app = current_app();
        for _ in 0..200 {
            let stream = AXNotificationStream::subscribe_many(&app, &[], 1).expect("async stream");
            drop(stream);
        }
        let _ = done_tx.send(());
    });
    done_rx
        .recv_timeout(Duration::from_secs(60))
        .expect("dropping a just-created stream hung");
}
