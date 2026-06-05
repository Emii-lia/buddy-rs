use super::*;

#[test]
fn test_wrap_in_bubble() {
    let msg = "Hello";
    let buddy = "o/";
    let bubble = wrap_in_bubble(msg, buddy);
    
    assert!(bubble.contains("Hello"));
    assert!(bubble.contains("o/"));
    assert!(bubble.contains("│"));
    assert!(bubble.contains("─"));
}

#[test]
fn test_wrap_in_bubble_multiline() {
    let msg = "Hello\nWorld";
    let buddy = "o/";
    let bubble = wrap_in_bubble(msg, buddy);
    
    assert!(bubble.contains("Hello"));
    assert!(bubble.contains("World"));
    assert!(bubble.contains("o/"));
}

#[test]
fn test_wrap_in_box() {
    let msg = "Hello";
    let buddy = "o/";
    let boxed = wrap_in_box(msg, buddy);
    
    assert!(boxed.contains("Hello"));
    assert!(boxed.contains("o/"));
    assert!(boxed.contains("║"));
    assert!(boxed.contains("═"));
    assert!(boxed.contains("╔"));
    assert!(boxed.contains("╗"));
    assert!(boxed.contains("╚"));
    assert!(boxed.contains("╝"));
}
