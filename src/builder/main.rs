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
