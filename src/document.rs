#![allow(dead_code)]
use directories;
use open;
use std::borrow::Borrow;
use std::error::Error;
use std::fmt::Display;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub struct Document {
    pathbuf: PathBuf,
    file: Option<File>,
    permissions: Option<Mode>,
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
                            let mut pathbuf = join_all(path, subdirs.clone());
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
                            let mut pathbuf = join_all(path, subdirs.clone());
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
                            let mut pathbuf = join_all(path, subdirs.clone());
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
                            let mut pathbuf = join_all(path, subdirs.clone());
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
                        let mut pathbuf = join_all(path, subdirs.clone());
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
    pub fn new(location: Folder, filename: &str) -> Result<Self, Box<dyn Error>> {
        let pathbuf = location.into_pathbuf_result(filename)?;
        Ok(Self {
            pathbuf,
            file: None,
            permissions: None,
        })
    }
    pub fn open(&mut self, permissions: Mode, create: Create) -> Result<&mut Self, Box<dyn Error>> {
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
        match create {
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
        if let Ok(file) = OpenOptions::new()
            .read(permissions.readable())
            .write(permissions.writable())
            .append(permissions.appendable())
            .open(self.pathbuf.clone())
        {
            self.file = Some(file);
            self.permissions = Some(permissions);
            Ok(self)
        } else {
            Err(DocumentError::CouldNotOpenFile(self.path()))?
        }
    }
    pub fn launch_with_default_app(&mut self) -> Result<(), Box<dyn Error>> {
        if let Some(_) = self.file {
            self.file = None;
        }
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
    pub fn file(&mut self) -> Option<&mut File> {
        self.file.as_mut()
    }
    pub fn close(&mut self) -> &mut Self {
        self.file = None;
        self.permissions = None;
        self
    }
    pub fn write(&mut self, content: &str) -> Result<&mut Self, Box<dyn Error>> {
        if let Some(file) = self.file() {
            file.write_all(content.as_bytes())?;
        }
        Ok(self)
    }
}

pub fn with<Closure>(document: &mut Document, closure: Closure)
where
    Closure: FnOnce(&mut Document) -> Result<(), Box<dyn Error>>,
{
    match closure(document) {
        Ok(_) => {}
        Err(error) => eprintln!("{}", error),
    }
}
