use axuielement::AXObserver;

#[test]
fn observer_type_id_is_non_zero() {
    assert!(AXObserver::type_id() > 0);
}

#[test]
fn observer_can_be_created_and_scheduled() {
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let observer = AXObserver::new(pid, |_| {}).expect("observer");
    observer.schedule_on_current_run_loop();
    observer.unschedule_from_run_loop();
}
