use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

use cbm::{
    disk::{directory::FileType, file::FileOps, Disk, Id, D64},
    Petscii,
};

pub fn do_packaging() -> std::io::Result<()> {
    let geometry = D64::geometry(false);
    let path = Path::new("demo-disk1.D64");
    File::create(path)?;
    let mut disk = D64::create(path, geometry, false)?;
    let name = Petscii::from_str("demo disk 1");
    let id = Id::from_bytes(b"1A");
    disk.write_format(&name, &id)?;

    add_files_to_disk(&mut disk)?;

    Ok(())
}

fn add_files_to_disk(disk: &mut D64) -> Result<()> {
    for (src_name, dst_name, file_type) in [
        ("bin/test-load-charset.prg", "load-charset", FileType::PRG),
        ("bin/test-charset.prg", "charset", FileType::PRG),
        ("bin/test-rasterbar.prg", "rasterbar", FileType::PRG),
        ("bin/test-controller.prg", "controller", FileType::PRG),
        ("bin/test-sprite.prg", "sprite", FileType::PRG),
        (
            "bin/test-sprite-duplication.prg",
            "sprite dup",
            FileType::PRG,
        ),
        ("src/ahoy_art_deco.64c", "font", FileType::SEQ),
    ] {
        add_file_to_disk(
            disk,
            Path::new(src_name),
            Petscii::from_str(dst_name),
            file_type,
        )?;
    }

    Ok(())
}

fn add_file_to_disk(
    disk: &mut D64,
    src_filename: &Path,
    dst_filename: Petscii,
    file_type: FileType,
) -> Result<()> {
    let file = disk.create_file(&dst_filename, file_type, cbm::disk::file::Scheme::Linear)?;

    let mut fs_file = std::fs::File::open(src_filename)?;
    let mut content = Vec::new();
    fs_file.read_to_end(&mut content)?;

    file.writer()?.write(&content)?;

    Ok(())
}
