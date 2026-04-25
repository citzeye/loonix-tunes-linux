pub fn delete_file(path: &str) -> Result<(), String> {
    trash::delete(path).map_err(|e| e.to_string())
}

pub struct FileService;

impl FileService {
    pub fn delete_file(&self, path: &str) -> Result<(), String> {
        delete_file(path)
    }
}

static FILE_SERVICE: FileService = FileService;

pub fn get_file_service() -> &'static FileService {
    &FILE_SERVICE
}