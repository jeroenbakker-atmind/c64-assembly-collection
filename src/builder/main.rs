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
            "bin/autostart.prg",
            Petscii::from_str("autostart"),
            FileType::PRG,
        ),
        (
            "bin/load-charset.prg",
            Petscii::from_str("load-charset"),
            FileType::PRG,
        ),
        (
            "bin/charset.prg",
            Petscii::from_str("charset"),
            FileType::PRG,
        ),
        (
            "bin/rasterbar.prg",
            Petscii::from_str("rasterbar"),
            FileType::PRG,
        ),
        (
            "bin/controller.prg",
            Petscii::from_str("controller"),
            FileType::PRG,
        ),
        (
            "bin/sprite.prg",
            Petscii::from_str("sprite"),
            FileType::PRG,
        ),
        (
            "bin/sprite-duplication.prg",
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
            "bin/load-program.prg",
            Petscii::from_str("load program"),
            FileType::PRG,
        ),
        (
            "bin/dummy.prg",
            Petscii::from_str("dummy"),
            FileType::PRG,
        ),
        // text mode
        (
            "bin/standard-text.prg",
            Petscii::from_str("standard text"),
            FileType::PRG,
        ),
        // bitmap mode
        (
            "bin/standard-bitmap.prg",
            Petscii::from_str("standard bitmap"),
            FileType::PRG,
        ),
    ])?;
    Ok(())
}

fn package_dev() -> Result<()> {
    let mut disk =
        c64::create_disk::initialize_disk(Petscii::from_str("development"), Id::from_bytes(b"FYR"))?;

    disk.add_files(&[
        (
            "bin/standard-bitmap2.prg",
            Petscii::from_str("bitmap"),
            FileType::PRG,
        ),
    ])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    print_petscii(Charset::Lower, Petscii::from("disk 1a"));
    //package_disk1a()?;
    package_dev()
}
