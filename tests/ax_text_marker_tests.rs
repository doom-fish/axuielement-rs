use axuielement::{AXTextMarker, AXTextMarkerRange};

#[test]
fn text_marker_round_trip() {
    let marker = AXTextMarker::from_bytes(&[1, 2, 3, 4]).expect("marker");
    assert_eq!(marker.len(), 4);
    assert_eq!(marker.bytes(), vec![1, 2, 3, 4]);
}

#[test]
fn text_marker_range_round_trip() {
    let start = AXTextMarker::from_bytes(&[1, 2]).expect("start");
    let end = AXTextMarker::from_bytes(&[3, 4]).expect("end");
    let range = AXTextMarkerRange::new(&start, &end).expect("range");
    assert_eq!(range.start_marker().bytes(), vec![1, 2]);
    assert_eq!(range.end_marker().bytes(), vec![3, 4]);
}
