use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

use cbm::{
    disk::{file::FileOps, Disk, Id, D64},
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

    for (src_name, dst_name) in [
        ("bin/test-charset.prg", "charset"),
        ("bin/test-rasterbar.prg", "rasterbar"),
        ("bin/test-controller.prg", "controller"),
        ("bin/test-sprite.prg", "sprite"),
        ("bin/test-sprite-duplication.prg", "sprite dup"),
    ] {
        add_program_file(&mut disk, Path::new(src_name), Petscii::from_str(dst_name))?;
    }
    Ok(())
}

fn add_program_file(disk: &mut D64, src_filename: &Path, dst_filename: Petscii) -> Result<()> {
    let file = disk.create_file(
        &dst_filename,
        cbm::disk::directory::FileType::PRG,
        cbm::disk::file::Scheme::Linear,
    )?;

    let mut fs_file = std::fs::File::open(src_filename)?;
    let mut content = Vec::new();
    fs_file.read_to_end(&mut content)?;

    file.writer()?.write(&content)?;

    Ok(())
}
