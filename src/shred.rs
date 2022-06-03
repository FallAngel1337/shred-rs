use std::{fs::{self, File}, io::{Write, Seek, BufWriter}, path::PathBuf};
use std::path::Path;
use rand::{Rng, RngCore, distributions::Alphanumeric};

use crate::{ShredAgrs, ShredConf};

pub fn shred(conf: ShredAgrs) -> crate::IoResult<()> {
    let conf = ShredConf::new(conf);

    for file in conf.files.iter().map(Path::new)
    {
        overwrite_file(file, conf.clone())?;
    }

    Ok(())
}

fn overwrite_file(path: &Path, conf: ShredConf) -> crate::IoResult<()> {
    let file = File::create(path)?;
    let mut buf = vec![0u8; conf.bytes];
    if !conf.zero {
        rand::thread_rng().fill_bytes(&mut buf);
    }

    let mut file = BufWriter::new(file);

    for _ in 0..conf.rounds {
        file.write_all(&buf)?;
        file.rewind()?;
    }

    if conf.delete {
        fs::remove_file(rename_file(path)?.unwrap())?;
    }

    Ok(())
}

fn rename_file(path: &Path) -> crate::IoResult<Option<PathBuf>> {
    let len = path.file_name().unwrap().len();
    let dir_path = path.parent().unwrap();
    let file = File::open(&dir_path)?;

    let mut rng = rand::thread_rng();
    let mut path = PathBuf::from(path);

    for _ in 0..len {
        let name = (0..len).map(|_| rng.sample(Alphanumeric) as char).collect::<String>();
        let new_path = dir_path.join(&name);

        if fs::metadata(&new_path).is_ok() {
            continue;
        }

        fs::rename(&path, &new_path).unwrap();
        path = new_path;
        file.sync_all()?;
    }

    Ok(Some(path))
}