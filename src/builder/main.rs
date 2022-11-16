use std::io::Result;

use c64::{
    charset::{print_petscii, Charset},
    create_disk::PackageDisk,
};
use cbm::{
    disk::{directory::FileType, Id},
    Petscii,
};

fn package_disk1a() -> Result<()> {
    let mut disk =
        c64::create_disk::initialize_disk(Petscii::from_str("demo disk 1"), Id::from_bytes(b"1A"))?;

    disk.add_files(&[
        (
            "bin/test-autostart.prg",
            Petscii::from_str("autostart"),
            FileType::PRG,
        ),
        (
            "bin/test-load-charset.prg",
            Petscii::from_str("load-charset"),
            FileType::PRG,
        ),
        (
            "bin/test-charset.prg",
            Petscii::from_str("charset"),
            FileType::PRG,
        ),
        (
            "bin/test-rasterbar.prg",
            Petscii::from_str("rasterbar"),
            FileType::PRG,
        ),
        (
            "bin/test-controller.prg",
            Petscii::from_str("controller"),
            FileType::PRG,
        ),
        (
            "bin/test-sprite.prg",
            Petscii::from_str("sprite"),
            FileType::PRG,
        ),
        (
            "bin/test-sprite-duplication.prg",
            Petscii::from_str("sprite dup"),
            FileType::PRG,
        ),
        (
            "src/ahoy_art_deco.64c",
            Petscii::from_str("font"),
            FileType::SEQ,
        ),
        // Load program and the dummy program that will be loaded.
        (
            "bin/test-load-program.prg",
            Petscii::from_str("load program"),
            FileType::PRG,
        ),
        (
            "bin/test-dummy.prg",
            Petscii::from_str("dummy"),
            FileType::PRG,
        ),
    ])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    print_petscii(Charset::Lower, Petscii::from("disk 1a"));
    package_disk1a()
}

#[test]
fn test_image_to_standard_character_mode() {
    use c64::image_converter::{DefaultImageContainer, ImageConverter, StandardCharacterMode};
    use png;
    use std::fs::File;
    // The decoder is a build for reader and can be used to set various decoding options
    // via `Transformations`. The default output transformation is `Transformations::IDENTITY`.
    let decoder = png::Decoder::new(File::open("resources/test.png").unwrap());
    let mut reader = decoder.read_info().unwrap();
    // Allocate the output buffer.
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let info = reader.next_frame(&mut buf).unwrap();

    let image = DefaultImageContainer {
        width: info.width as usize,
        height: info.height as usize,

        buffer: buf.clone(),
        components_per_pixel: 4,
    };
    let converter = StandardCharacterMode::default();

    let text_image = converter.convert(&image);

    for chunk in text_image.characters.chunks(16) {
        print!("  .byte ");
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("${:2x}", a);
        }
        println!()
    }
    for chunk in text_image.foreground_colors.chunks(16) {
        print!("  .byte ");
        for (i, a) in chunk.iter().enumerate() {
            if i != 0 {
                print!(", ");
            }
            print!("{:?}", a);
        }
        println!()
    }
    println!("/* background {:?} */", text_image.background_color);
    unimplemented!()
}
