use std::env;
use std::string::FromUtf8Error;

#[derive(Debug)]
pub enum ResourceManagerError {
    IoError(std::io::Error),
    Utf8Error(FromUtf8Error),
}

pub struct ResourceManager {
    root: std::path::PathBuf,
}

impl ResourceManager {
    pub fn new() -> Self {
        let Ok(root) = env::current_dir() else {
            panic!("Can't get current directory");
        };

        ResourceManager {
            root
        }
    }

    pub fn load_raw_resource(&self, name: &str) -> Result<Vec<u8>, std::io::Error> {
        let path = self.root.join(name);
        if !path.exists() {
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "Not found"))?
        }

        // read data as bytes
        let data = std::fs::read(path)?;
        Ok(data)
    }

    pub fn load_shader_source(&self, name: &str) -> Result<String, ResourceManagerError> {
        let res = self.load_raw_resource(name);
        if res.is_err() {
            return Err(ResourceManagerError::IoError(res.err().unwrap()));
        }

        let data = res.unwrap();
        let res = String::from_utf8(data);
        if res.is_err() {
            return Err(ResourceManagerError::Utf8Error(res.err().unwrap()))
        }
        Ok(res.unwrap())
    }
}