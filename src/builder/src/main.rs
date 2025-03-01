mod applications;

use std::{io::Result, path::Path};

use applications::{
    engine::engine_application, intro::intro_application, set_black_border::set_black_border_application,
};
use c64::create_disk::PackageDisk;
use cbm::{
    disk::{directory::FileType, Id},
    Petscii,
};

#[allow(dead_code)]
fn package_disk1a() -> Result<()> {
    let mut disk = c64::create_disk::initialize_disk(
        Path::new("demo-disk1.D64"),
        Petscii::from_str("demo disk 1"),
        Id::from_bytes(b"1A"),
    )?;

    disk.add_files(&[
        ("bin/autostart.prg", Petscii::from_str("autostart"), FileType::PRG),
        ("bin/load-charset.prg", Petscii::from_str("load-charset"), FileType::PRG),
        ("bin/charset.prg", Petscii::from_str("charset"), FileType::PRG),
        ("bin/rasterbar.prg", Petscii::from_str("rasterbar"), FileType::PRG),
        ("bin/controller.prg", Petscii::from_str("controller"), FileType::PRG),
        ("bin/sprite.prg", Petscii::from_str("sprite"), FileType::PRG),
        (
            "bin/sprite-duplication.prg",
            Petscii::from_str("sprite dup"),
            FileType::PRG,
        ),
        ("src/ahoy_art_deco.64c", Petscii::from_str("font"), FileType::SEQ),
        // Load program and the dummy program that will be loaded.
        ("bin/load-program.prg", Petscii::from_str("load program"), FileType::PRG),
        ("bin/dummy.prg", Petscii::from_str("dummy"), FileType::PRG),
        // text mode
        (
            "bin/standard-text.prg",
            Petscii::from_str("standard text"),
            FileType::PRG,
        ),
        (
            "bin/standard-text-cs.prg",
            Petscii::from_str("std txt cs"),
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
    let mut disk = c64::create_disk::initialize_disk(
        Path::new("development.D64"),
        Petscii::from_str("development"),
        Id::from_bytes(b"FYR"),
    )?;

    disk.add_bytes(&intro_application().unwrap(), Petscii::from_str("intro"), FileType::PRG)?;

    disk.add_bytes(
        &engine_application().unwrap(),
        Petscii::from_str("engine"),
        FileType::PRG,
    )?;
    disk.add_bytes(
        &set_black_border_application(),
        Petscii::from_str("set black border"),
        FileType::PRG,
    )?;

    Ok(())
}

fn main() -> std::io::Result<()> {
    //package_disk1a()?;
    package_dev()
}
