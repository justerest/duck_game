use macroquad::prelude::*;

pub struct AssetsServer {
    base_path: String,
}

impl AssetsServer {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.into(),
        }
    }

    pub async fn load_string(&self, path: &str) -> Result<String, FileError> {
        load_string(&self.join(path)).await
    }

    fn join(&self, path: &str) -> String {
        [self.base_path.as_str(), path].join("/")
    }

    pub async fn load_texture(&self, path: &str) -> Result<Texture2D, FileError> {
        load_texture(&self.join(path)).await
    }
}
