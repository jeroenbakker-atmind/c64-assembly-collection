use c64_colors::colors::Color;

use crate::{builder::demo::DemoBuilder, encoder::Encoder};

#[test]
fn encode_empty() {
    let demo = DemoBuilder::default();

    assert_eq!(2, demo.byte_size());
    let encoded_demo = demo.build();
    println!("{encoded_demo:?}");
    assert_eq!(2, encoded_demo.len());
}

#[test]
fn encode() {
    let mut demo = DemoBuilder::default();

    demo.new_frame()
        .set_palette4([Color::White, Color::Grey, Color::Black, Color::Purple])
        .fill_video_memory(0);

    assert_eq!(9, demo.byte_size());
    let encoded_demo = demo.build();
    println!("{encoded_demo:?}");
    assert_eq!(9, encoded_demo.len());
}
