use axuielement::{AXError, AXPoint, AXRange, AXRect, AXSize, AXUIElement, AXValue};

#[test]
fn scalar_and_geometry_values_round_trip() {
    let string = AXValue::from_string("hello").expect("string");
    let boolean = AXValue::from_bool(true);
    let integer = AXValue::from_i64(7);
    let float = AXValue::from_f64(3.5);
    let point = AXValue::from_point(AXPoint { x: 1.0, y: 2.0 }).expect("point");
    let size = AXValue::from_size(AXSize {
        width: 3.0,
        height: 4.0,
    })
    .expect("size");
    let rect = AXValue::from_rect(AXRect {
        origin: AXPoint { x: 5.0, y: 6.0 },
        size: AXSize {
            width: 7.0,
            height: 8.0,
        },
    })
    .expect("rect");
    let range = AXValue::from_range(AXRange {
        location: 9,
        length: 10,
    })
    .expect("range");
    let error = AXValue::from_error(AXError::CannotComplete).expect("error");
    let data = AXValue::from_data(&[1, 2, 3]).expect("data");

    assert_eq!(string.as_string().as_deref(), Some("hello"));
    assert_eq!(boolean.as_bool(), Some(true));
    assert_eq!(integer.as_i64(), Some(7));
    assert_eq!(float.as_f64(), Some(3.5));
    assert_eq!(point.as_point(), Some(AXPoint { x: 1.0, y: 2.0 }));
    assert_eq!(
        size.as_size(),
        Some(AXSize {
            width: 3.0,
            height: 4.0
        })
    );
    assert_eq!(
        rect.as_rect(),
        Some(AXRect {
            origin: AXPoint { x: 5.0, y: 6.0 },
            size: AXSize {
                width: 7.0,
                height: 8.0
            }
        })
    );
    assert_eq!(
        range.as_range(),
        Some(AXRange {
            location: 9,
            length: 10
        })
    );
    assert_eq!(error.as_error(), Some(AXError::CannotComplete));
    assert_eq!(data.as_data(), Some(vec![1, 2, 3]));
}

#[test]
fn aggregate_and_element_values_round_trip() {
    let text = AXValue::from_string("item").expect("text");
    let number = AXValue::from_i64(42);
    let array = AXValue::from_array(&[&text, &number]).expect("array");
    let dictionary = AXValue::from_dictionary(&[("answer", &number), ("label", &text)])
        .expect("dictionary result")
        .expect("dictionary");
    let pid = i32::try_from(std::process::id()).expect("current pid fits in i32");
    let app = AXUIElement::from_pid(pid).expect("element");
    let element = AXValue::from_element(&app).expect("element value");

    assert_eq!(array.as_array().expect("array values").len(), 2);
    let entries = dictionary.as_dictionary().expect("dictionary entries");
    assert_eq!(entries.len(), 2);
    assert_eq!(
        element.as_element().expect("element").pid().expect("pid"),
        pid
    );
}
