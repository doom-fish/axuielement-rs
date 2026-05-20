use axuielement::async_api::AXNotificationStream;
use axuielement::AXUIElement;

#[test]
fn async_stream_can_subscribe_without_notifications() {
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let app = AXUIElement::from_pid(pid).expect("AX application handle");
    let stream = AXNotificationStream::subscribe_many(&app, &[], 4).expect("async stream");
    assert_eq!(stream.buffered_count(), 0);
    assert!(!stream.is_closed());
}
