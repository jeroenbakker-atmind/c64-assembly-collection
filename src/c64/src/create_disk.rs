use std::{
    fs::File,
    io::{Read, Result},
    path::Path,
};

use cbm::{
    disk::{directory::FileType, file::FileOps, Disk, Id, D64},
    Petscii,
};

pub fn initialize_disk(path: &Path, name: Petscii, id: Id) -> std::io::Result<D64> {
    let geometry = D64::geometry(false);
    File::create(path)?;
    let mut disk = D64::create(path, geometry, false)?;
    disk.write_format(&name, &id)?;

    Ok(disk)
}

type AddFile = (&'static str, Petscii, FileType);

pub trait PackageDisk {
    fn add_files(&mut self, files: &[AddFile]) -> Result<()>;
    fn add_file(&mut self, file: &AddFile) -> Result<()>;
    fn add_bytes(&mut self, bytes: &[u8], petscii: Petscii, file_type: FileType) -> Result<()>;
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
        self.add_bytes(&content, file.1.clone(), file.2)
    }

    fn add_bytes(&mut self, bytes: &[u8], petscii: Petscii, file_type: FileType) -> Result<()> {
        let d64_file = self.create_file(&petscii, file_type, cbm::disk::file::Scheme::Linear)?;
        d64_file.writer()?.write(bytes)?;
        Ok(())
    }
}
