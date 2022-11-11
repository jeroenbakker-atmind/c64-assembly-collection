use std::io::Result;

use c64::create_disk::PackageDisk;
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
    ])?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    package_disk1a()
}
