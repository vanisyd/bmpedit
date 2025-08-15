pub mod bitmap;

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;

pub static PATH_RESOURCES: &str = "res";

pub trait Resource: Sized {
    fn from_file(path: impl AsRef<Path>) -> Result<Self, String> {
        let f = File::open(path)
            .map_err(|err| format!("Failed to load resource: {:?}", err))?;
        let mut reader = BufReader::new(f);

        let res = Self::load_res(&mut reader);
        
        Ok(res)
    }

    fn load_res<R: Sized + Read>(reader: &mut BufReader<R>) -> Self;
}