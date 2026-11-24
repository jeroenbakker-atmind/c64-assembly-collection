use std::{
    collections::HashSet,
    fs::File,
    io::{Error, ErrorKind, Read, Result},
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

/// One contiguous chain of raw 256-byte sectors to master onto the D64.
///
/// This type lives in `c64` so the disk writer does not need to depend on the
/// encoder crate. Track is 1-based; sector is the physical D64 sector number
/// (0-based), matching the planner and the loader's resource table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MasteredRange {
    /// Physical `(track, sector)` for each entry in `sectors`, in load order.
    pub locations: Vec<(u8, u8)>,
    pub sectors: Vec<[u8; 256]>,
}

/// Write raw resource sectors and reserve them in the BAM.
///
/// This must run before any DOS PRGs are added. `create_file()` only allocates
/// free blocks, so reserving first keeps PRG chains out of the resource
/// regions.
pub fn write_master_ranges(disk: &mut D64, ranges: &[MasteredRange]) -> Result<()> {
    let format = disk.disk_format()?;
    let mut seen = HashSet::new();
    let mut locations = Vec::new();

    for (range_index, range) in ranges.iter().enumerate() {
        if range.locations.len() != range.sectors.len() {
            return Err(Error::new(
                ErrorKind::InvalidInput,
                format!(
                    "mastered range {range_index} has {} locations but {} sectors",
                    range.locations.len(),
                    range.sectors.len()
                ),
            ));
        }

        for &(track, sector) in &range.locations {
            if track < format.first_track || track > format.last_track {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("mastered range {range_index} uses invalid track {track}"),
                ));
            }
            if format.is_reserved_track(track) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("mastered range {range_index} uses reserved track {track}"),
                ));
            }
            if sector >= format.sectors_in_track(track) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("mastered range {range_index} uses invalid sector {track}/{sector}"),
                ));
            }
            if !seen.insert((track, sector)) {
                return Err(Error::new(
                    ErrorKind::InvalidInput,
                    format!("mastered sector {track}/{sector} appears more than once"),
                ));
            }
            locations.push((track, sector));
        }
    }

    for range in ranges {
        for (&(track, sector), payload) in range.locations.iter().zip(&range.sectors) {
            let mut location = format.first_directory_location();
            location.0 = track;
            location.1 = sector;
            disk.blocks_ref_mut().sector_mut(location)?.copy_from_slice(payload);
        }
    }

    {
        let bam_ref = disk.bam()?;
        let mut bam = bam_ref.borrow_mut();
        for &(track, sector) in &locations {
            let mut location = format.first_directory_location();
            location.0 = track;
            location.1 = sector;
            bam.allocate(location)?;
        }
        bam.flush()?;
    }
    disk.blocks_ref_mut().flush()?;

    Ok(())
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

    fn add_bytes(&mut self, bytes: &[u8], filename: Petscii, file_type: FileType) -> Result<()> {
        let d64_file = self.create_file(&filename, file_type, cbm::disk::file::Scheme::Linear)?;
        d64_file.writer()?.write(bytes)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicUsize, Ordering},
    };

    use cbm::disk::Id;

    use super::*;

    static TEST_DISK_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TestDisk {
        path: PathBuf,
    }

    impl TestDisk {
        fn new() -> (Self, D64) {
            let index = TEST_DISK_COUNTER.fetch_add(1, Ordering::Relaxed);
            let path = std::env::temp_dir().join(format!("c64-master-{}-{index}.D64", std::process::id()));
            let _ = fs::remove_file(&path);
            let disk = initialize_disk(&path, Petscii::from_str("master test"), Id::from_bytes(b"MT"))
                .expect("failed to create test disk");
            (Self { path }, disk)
        }
    }

    impl Drop for TestDisk {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    fn assert_sector(disk: &D64, track: u8, sector: u8, expected: &[u8; 256]) {
        let mut location = disk.disk_format().unwrap().first_directory_location();
        location.0 = track;
        location.1 = sector;
        let actual = disk.read_sector(location).unwrap();
        assert_eq!(&actual[..], &expected[..], "sector {track}/{sector}");
    }

    #[test]
    fn master_ranges_write_reserve_and_survive_later_prgs() {
        let (_guard, mut disk) = TestDisk::new();
        let locations = vec![(1, 1), (1, 8), (2, 3)];
        let payloads = [[0xA5u8; 256], [0x5Au8; 256], {
            let mut sector = [0u8; 256];
            for (index, byte) in sector.iter_mut().enumerate() {
                *byte = index as u8;
            }
            sector
        }];
        let ranges = vec![MasteredRange {
            locations: locations.clone(),
            sectors: payloads.to_vec(),
        }];

        let free_before = disk.bam().unwrap().borrow().blocks_free();
        write_master_ranges(&mut disk, &ranges).unwrap();
        let free_after = disk.bam().unwrap().borrow().blocks_free();
        assert_eq!(free_before - free_after, locations.len());

        {
            let bam_ref = disk.bam().unwrap();
            let bam = bam_ref.borrow();
            for &(track, sector) in &locations {
                let entry = bam.entry(track).unwrap();
                assert_eq!(
                    entry.sector_map() & (1u64 << sector),
                    0,
                    "sector {track}/{sector} should be allocated"
                );
            }
        }

        for ((track, sector), payload) in locations.iter().zip(&payloads) {
            assert_sector(&disk, *track, *sector, payload);
        }

        disk.add_bytes(
            &[0x00, 0x08, 0x60, 0xA0, 0x00, 0xFF],
            Petscii::from_str("after master"),
            FileType::PRG,
        )
        .unwrap();

        for ((track, sector), payload) in locations.iter().zip(&payloads) {
            assert_sector(&disk, *track, *sector, payload);
        }
        let errors = disk.validate().unwrap();
        assert!(
            errors
                .iter()
                .all(|error| matches!(error, cbm::disk::ValidationError::SectorMisallocated(_))),
            "unexpected validation errors: {errors:?}"
        );
    }

    #[test]
    fn master_ranges_reject_reserved_duplicate_and_invalid_locations() {
        let (_guard, mut disk) = TestDisk::new();
        let payload = [0u8; 256];

        let reserved = MasteredRange {
            locations: vec![(18, 1)],
            sectors: vec![payload],
        };
        assert!(write_master_ranges(&mut disk, &[reserved]).is_err());

        let invalid_sector = MasteredRange {
            locations: vec![(1, 21)],
            sectors: vec![payload],
        };
        assert!(write_master_ranges(&mut disk, &[invalid_sector]).is_err());

        let duplicate = MasteredRange {
            locations: vec![(1, 1), (1, 1)],
            sectors: vec![payload, payload],
        };
        assert!(write_master_ranges(&mut disk, &[duplicate]).is_err());

        let mismatched = MasteredRange {
            locations: vec![(1, 1)],
            sectors: vec![],
        };
        assert!(write_master_ranges(&mut disk, &[mismatched]).is_err());
    }
}
