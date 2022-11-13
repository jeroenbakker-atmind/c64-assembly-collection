use std::io::Result;

use c64::{
    charset::{find_best_petschii, print_petscii, Charset},
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
fn test_image_to_petscii() {
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
    // Grab the bytes of the image.
    let bytes = &buf[..info.buffer_size()];

    let height = reader.info().height;
    let width = reader.info().width;
    println!("{}, {}", width, height);
    for y in 0..height / 8 {
        let mut petscii_chars = Vec::new();
        for x in 0..width / 8 {
            let mut bits = Vec::new();
            for iy in 0..8 {
                for ix in 0..8 {
                    let xo = ix + x * 8;
                    let yo = iy + y * 8;
                    let byte = bytes[(yo * width + xo) as usize];
                    let bit = if byte > 127 { true } else { false };
                    //println!("{},{}+{},{}={},{} {}", x, y, ix, iy, xo, yo, bit);
                    bits.push(bit);
                }
            }
            let best_match = find_best_petschii(&bits);
            petscii_chars.push(best_match);
        }
        println!("\"{}\"", Petscii::from_bytes(&petscii_chars));
    }
    unimplemented!()
}
