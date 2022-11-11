use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

use cbm::{
    disk::{directory::FileType, file::FileOps, Disk, Id, D64},
    Petscii,
};

pub fn initialize_disk(name: Petscii, id: Id) -> std::io::Result<D64> {
    let geometry = D64::geometry(false);
    let path = Path::new("demo-disk1.D64");
    File::create(path)?;
    let mut disk = D64::create(path, geometry, false)?;
    disk.write_format(&name, &id)?;

    Ok(disk)
}

type AddFile = (&'static str, Petscii, FileType);

pub trait PackageDisk {
    fn add_files(&mut self, files: &[AddFile]) -> Result<()>;
    fn add_file(&mut self, file: &AddFile) -> Result<()>;
}

impl PackageDisk for D64 {
    fn add_files(&mut self, files: &[AddFile]) -> Result<()> {
        for add_file in files {
            self.add_file(add_file)?;
        }
        Ok(())
    }

    fn add_file(&mut self, file: &AddFile) -> Result<()> {
        let mut fs_file = std::fs::File::open(Path::new(file.0))?;
        let mut content = Vec::new();
        fs_file.read_to_end(&mut content)?;

        let d64_file = self.create_file(&file.1, file.2, cbm::disk::file::Scheme::Linear)?;
        d64_file.writer()?.write(&content)?;

        Ok(())
    }
}
