#![allow(dead_code)]
use directories;
use open;
use std::collections::HashMap;
use std::error::Error;
use std::fmt::Display;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::ops::{Index, IndexMut};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
pub struct Document {
    pathbuf: PathBuf,
    create_policy: Create,
}

#[derive(Debug, Clone, Copy)]
pub enum Mode {
    Read,
    Replace,
    Append,
    ReadReplace,
    ReadAppend,
}

impl Mode {
    pub fn readable(&self) -> bool {
        match self {
            Self::Read | Self::ReadReplace | Self::ReadAppend => true,
            _ => false,
        }
    }
    pub fn writable(&self) -> bool {
        match self {
            Self::Replace | Self::Append | Self::ReadAppend | Self::ReadReplace => true,
            _ => false,
        }
    }
    pub fn appendable(&self) -> bool {
        match self {
            Self::Append | Self::ReadAppend => true,
            _ => false,
        }
    }
}

pub enum Folder<'a> {
    User(User<'a>),
    Project((Project, &'a str, &'a str, &'a str)),
}

fn join_all(path: &Path, subdirs: &[&str]) -> PathBuf {
    let mut pathbuf = path.to_path_buf();
    for subdir in subdirs {
        pathbuf.push(subdir);
    }
    pathbuf
}

impl<'a> Folder<'a> {
    fn into_pathbuf_result(&self, filename: &str) -> Result<PathBuf, DocumentError> {
        match self {
            Folder::User(subdir) => match subdir {
                User::Pictures(subdirs) => {
                    if let Some(dir) = directories::UserDirs::new() {
                        if let Some(path) = dir.picture_dir() {
                            let mut pathbuf = join_all(path, subdirs);
                            pathbuf = pathbuf.join(filename);
                            Ok(pathbuf)
                        } else {
                            Err(DocumentError::PicturesDirNotFound)?
                        }
                    } else {
                        Err(DocumentError::UserDirsNotFound)?
                    }
                }
                User::Videos(subdirs) => {
                    if let Some(dir) = directories::UserDirs::new() {
                        if let Some(path) = dir.video_dir() {
                            let mut pathbuf = join_all(path, subdirs);
                            pathbuf = pathbuf.join(filename);
                            Ok(pathbuf)
                        } else {
                            Err(DocumentError::VideosDirNotFound)?
                        }
                    } else {
                        Err(DocumentError::UserDirsNotFound)?
                    }
                }
                User::Downloads(subdirs) => {
                    if let Some(dir) = directories::UserDirs::new() {
                        if let Some(path) = dir.download_dir() {
                            let mut pathbuf = join_all(path, subdirs);
                            pathbuf = pathbuf.join(filename);
                            Ok(pathbuf)
                        } else {
                            Err(DocumentError::DownloadsDirNotFound)?
                        }
                    } else {
                        Err(DocumentError::UserDirsNotFound)?
                    }
                }
                User::Documents(subdirs) => {
                    if let Some(dir) = directories::UserDirs::new() {
                        if let Some(path) = dir.document_dir() {
                            let mut pathbuf = join_all(path, subdirs);
                            pathbuf = pathbuf.join(filename);
                            Ok(pathbuf)
                        } else {
                            Err(DocumentError::DocumentsDirNotFound)?
                        }
                    } else {
                        Err(DocumentError::UserDirsNotFound)?
                    }
                }
                User::Home(subdirs) => {
                    if let Some(dir) = directories::UserDirs::new() {
                        let path = dir.home_dir();
                        let mut pathbuf = join_all(path, subdirs);
                        pathbuf = pathbuf.join(filename);
                        Ok(pathbuf)
                    } else {
                        Err(DocumentError::UserDirsNotFound)?
                    }
                }
            },
            Folder::Project((subdir, qualifier, organization, application)) => match subdir {
                Project::Data => {
                    if let Some(dir) =
                        directories::ProjectDirs::from(qualifier, organization, application)
                    {
                        let mut pathbuf = PathBuf::from(dir.data_dir());
                        pathbuf = pathbuf.join(filename);
                        Ok(pathbuf)
                    } else {
                        Err(DocumentError::ProjectDirsNotFound)?
                    }
                }
                Project::Config => {
                    if let Some(dir) =
                        directories::ProjectDirs::from(qualifier, organization, application)
                    {
                        let mut pathbuf = PathBuf::from(dir.config_dir());
                        pathbuf = pathbuf.join(filename);
                        Ok(pathbuf)
                    } else {
                        Err(DocumentError::ProjectDirsNotFound)?
                    }
                }
            },
        }
    }
}

pub enum User<'a> {
    Documents(&'a [&'a str]),
    Pictures(&'a [&'a str]),
    Videos(&'a [&'a str]),
    Downloads(&'a [&'a str]),
    Home(&'a [&'a str]),
}

pub enum Project {
    Config,
    Data,
}

impl<'a> Project {
    /// The app ID should have the reverse-DNS format of "com.example.App", where "com" is the qualifier, "example" is the organization and "App" is the application
    pub fn with_id(
        self,
        qualifier: &'a str,
        organization: &'a str,
        application: &'a str,
    ) -> (Self, &'a str, &'a str, &'a str) {
        (self, qualifier, organization, application)
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Create {
    No,
    OnlyIfNotExists,
    AutoRenameIfExists,
}

#[derive(Debug)]
pub enum DocumentError {
    UserDirsNotFound,
    PicturesDirNotFound,
    VideosDirNotFound,
    DownloadsDirNotFound,
    DocumentsDirNotFound,
    ProjectDirsNotFound,
    FileNotFound(String),
    CouldNotCreateFile(String),
    CouldNotCreateParentFolder(String),
    CouldNotLaunchFile(String),
    CouldNotOpenFile(String),
    FileNotWritable(String),
    FileNotOpen(String),
}

impl Display for DocumentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg: String = match self {
            Self::UserDirsNotFound => "User directories not found".to_string(),
            Self::PicturesDirNotFound => "Pictures directory not found".to_string(),
            Self::VideosDirNotFound => "Videos directory not found".to_string(),
            Self::DownloadsDirNotFound => "Downloads directory not found".to_string(),
            Self::FileNotFound(file_path) => "File not found: ".to_string() + file_path,
            Self::CouldNotCreateFile(file_path) => {
                "Could not create file: ".to_string() + file_path
            }
            Self::CouldNotCreateParentFolder(parent_folder_path) => {
                "Could not create parent folder: ".to_string() + parent_folder_path
            }
            Self::CouldNotLaunchFile(file_path) => {
                "Could not launch file with default app: ".to_string() + file_path
            }
            Self::ProjectDirsNotFound => "Project directories not found".to_string(),
            Self::CouldNotOpenFile(file_path) => "Could not open file: ".to_string() + file_path,
            Self::DocumentsDirNotFound => "Documents directory not found".to_string(),
            Self::FileNotWritable(file_path) => "File not writable: ".to_string() + file_path,
            Self::FileNotOpen(file_path) => "File not open: ".to_string() + file_path,
        };
        f.pad(msg.as_str())
    }
}

impl Error for DocumentError {
    fn description(&self) -> &str {
        "Document error"
    }
}

impl Document {
    pub fn at(location: Folder, filename: &str, create: Create) -> Result<Self, Box<dyn Error>> {
        let pathbuf = location.into_pathbuf_result(filename)?;
        Ok(Self {
            pathbuf,
            create_policy: create,
        })
    }
    fn open_file(&mut self, permissions: Mode) -> Result<File, Box<dyn Error>> {
        let filename = self
            .pathbuf
            .clone()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
            .split(".")
            .collect::<Vec<&str>>()[0]
            .to_string();
        let extension = self
            .pathbuf
            .clone()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();
        match self.create_policy {
            Create::OnlyIfNotExists => {
                if let Some(parent_folder) = self.pathbuf.clone().parent() {
                    if let Err(_) = create_dir_all(parent_folder) {
                        Err(DocumentError::CouldNotCreateParentFolder(
                            parent_folder
                                .to_path_buf()
                                .to_str()
                                .unwrap_or("")
                                .to_string(),
                        ))?
                    }
                }
                if !self.pathbuf.exists() {
                    OpenOptions::new()
                        .read(false)
                        .write(true)
                        .create_new(true)
                        .open(self.pathbuf.clone())?;
                }
            }
            Create::AutoRenameIfExists => {
                if let Some(parent_folder) = self.pathbuf.clone().parent() {
                    if let Err(_) = create_dir_all(parent_folder) {
                        Err(DocumentError::CouldNotCreateParentFolder(
                            parent_folder
                                .to_path_buf()
                                .to_str()
                                .unwrap_or("")
                                .to_string(),
                        ))?
                    }
                }
                let mut suffix: u32 = 0;
                while self.pathbuf.exists() {
                    suffix += 1;
                    let new_filename =
                        filename.clone() + suffix.to_string().as_str() + "." + extension.as_str();
                    self.pathbuf = self
                        .pathbuf
                        .clone()
                        .parent()
                        .unwrap_or(&Path::new(""))
                        .join(new_filename);
                }
                OpenOptions::new()
                    .read(false)
                    .write(true)
                    .create_new(true)
                    .open(self.pathbuf.clone())?;
            }
            _ => {}
        }
        if !self.pathbuf.exists() {
            Err(DocumentError::FileNotFound(self.path()))?
        }
        match OpenOptions::new()
            .read(permissions.readable())
            .write(permissions.writable())
            .append(permissions.appendable())
            .open(self.pathbuf.clone())
        {
            Ok(file) => Ok(file),
            Err(_) => Err(DocumentError::CouldNotOpenFile(self.path()))?,
        }
    }
    pub fn launch_with_default_app(&self) -> Result<(), Box<dyn Error>> {
        if let Err(_) = open::that_detached(self.path()) {
            Err(DocumentError::CouldNotLaunchFile(self.path()))?
        } else {
            Ok(())
        }
    }
    pub fn name(&self) -> String {
        self.pathbuf
            .clone()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or("")
            .to_string()
    }
    pub fn path(&self) -> String {
        self.pathbuf.as_os_str().to_str().unwrap_or("").to_string()
    }
    pub fn file(&mut self, permissions: Mode) -> Result<File, Box<dyn Error>> {
        self.open_file(permissions)
    }
    pub fn write(&mut self, content: &str) -> Result<&mut Self, Box<dyn Error>> {
        let mut file = self.open_file(Mode::Append)?;
        file.write_all(content.as_bytes())?;
        Ok(self)
    }
}

pub struct Map(HashMap<String, Document>);

impl Index<&str> for Map {
    type Output = Document;
    fn index(&self, index: &str) -> &Self::Output {
        &self.0[index]
    }
}

impl IndexMut<&str> for Map {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        self.0.get_mut(index).unwrap()
    }
}

pub fn with<Closure>(documents: &[Result<Document, Box<dyn Error>>], closure: Closure)
where
    Closure: FnOnce(Map) -> Result<(), Box<dyn Error>>,
{
    let mut document_map = HashMap::new();
    for document_result in documents.to_owned() {
        let document = match document_result {
            Ok(document) => document,
            Err(error) => {
                eprintln!("{}", error);
                return;
            }
        };
        document_map.insert(document.name(), (*document).clone());
    }
    match closure(Map(document_map)) {
        Ok(_) => {}
        Err(error) => eprintln!("{}", error),
    }
}
