use documents::prelude::*;
use main::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use std::{
    collections::{HashMap, HashSet},
    env,
    error::Error,
    fmt::Display,
    io::{Read, Write},
    net::{SocketAddr, TcpListener, TcpStream},
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
    thread,
    time::{Duration, Instant},
};

fn served(server_id: String, filename: impl Display) -> PathBuf {
    PathBuf::from(format!(
        "{}/served_files/{}",
        server_id,
        filename.to_string()
    ))
}

fn reintegrate(chunks: Vec<Option<String>>) -> String {
    chunks
        .into_iter()
        .filter_map(identity())
        .collect::<Vec<String>>()
        .join("")
}

fn read_settings() -> HashMap<String, (String, u16)> {
    let mut settings = HashMap::new();
    with(
        &[Document::at_path(
            PathBuf::from("peer_settings.txt").path(),
            "settings",
            Create::No,
        )],
        |d| {
            for line in d["settings"].lines()? {
                let line = line?;
                let split = line.split_whitespace().collect::<Vec<_>>();
                let id = split[0].to_string();
                let addr = split[1].to_string();
                let port = split[2].parse()?;
                settings.insert(id, (addr, port));
            }
            OK
        },
    );
    settings
}

struct PartialUpload {
    filename: String,
    size: usize,
    chunks: Vec<Option<String>>,
    server_id: String,
}

impl PartialUpload {
    fn new(filename: String, size: usize, server_id: String) -> Self {
        Self {
            filename,
            size,
            chunks: Vec::with_capacity(size / 100),
            server_id,
        }
    }
    fn no_of_chunks(&self) -> usize {
        (self.size as f64 / 100.0).ceil() as usize
    }
    fn is_complete(&self) -> bool {
        (0..(self.no_of_chunks()))
            .into_iter()
            .all(|i| self.chunks[i].is_some())
    }
    fn save(&mut self, chunk: usize, content: String) {
        self.chunks[chunk] = Some(content);
    }
    fn write_to_disk(self) -> Whoops {
        if !self.is_complete() {
            Err(format!("Upload for {} is not complete", self.filename))?
        }
        let f = served(self.server_id.clone(), self.filename.clone());
        if f.exists() {
            Err(format!("{} already exists", self.filename))?
        }
        with(
            &[Document::at_path(
                f.display(),
                "file",
                Create::AutoRenameIfExists,
            )],
            |mut d| {
                d["file"].replace_with(reintegrate(self.chunks).as_bytes())?;
                OK
            },
        );
        OK
    }
}

struct Server {
    id: String,
    addr: String,
    port: u16,
    uploads: Arc<Mutex<HashMap<String, PartialUpload>>>,
    debug: bool,
}

impl Server {
    fn new(id: String, addr: String, port: u16, debug: bool) -> Self {
        Self {
            id,
            addr,
            port,
            uploads: Arc::new(Mutex::new(map! {})),
            debug,
        }
    }
    fn run(self) -> Whoops {
        let server_socket = TcpListener::bind((&self.addr[..], self.port))?;
        loop {
            let client = server_socket.accept()?;
            let id = self.id.clone();
            let uploads = self.uploads.clone();
            thread::spawn(move || {
                ServerWorker::new(client, id, uploads)
                    .run()
                    .debug()
                    .discard()
            });
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Command {
    Filelist,
    UploadDeclare {
        filename: String,
        bytes: usize,
    },
    UploadContent {
        filename: String,
        chunk: usize,
        content: String,
    },
    DownloadDeclare {
        filename: String,
    },
    DownloadContent {
        filename: String,
        chunk: usize,
        content: String,
    },
    Delete {
        filename: String,
    },
}

impl Command {
    fn from_string(s: String) -> serde_json::Result<Self> {
        from_str(&s[..])
    }
}

enum UserCommand<'a> {
    Filelist { ids: &'a [String] },
    Upload { ids: &'a [String], filename: String },
    Download { ids: &'a [String], filename: String },
    Delete { ids: &'a [String], filename: String },
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum ResponseCode {
    K200(String),
    K250(String),
    K330(String),
}

impl ResponseCode {
    fn ok(&self) -> bool {
        match self {
            Self::K200(_) => true,
            _ => false,
        }
    }
    fn err(&self) -> bool {
        match self {
            Self::K250(_) => true,
            _ => false,
        }
    }
    fn ready(&self) -> bool {
        match self {
            Self::K330(_) => true,
            _ => false,
        }
    }
    fn message(&self) -> String {
        match self {
            ResponseCode::K200(m) => m.clone(),
            ResponseCode::K250(m) => m.clone(),
            ResponseCode::K330(m) => m.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
enum Response {
    Filelist {
        code: ResponseCode,
        filenames: Vec<String>,
    },
    Upload {
        code: ResponseCode,
    },
    DownloadDeclare {
        code: ResponseCode,
    },
    DownloadContent {
        code: ResponseCode,
        chunk: u64,
        content: String,
    },
    Delete {
        code: ResponseCode,
    },
}

impl Response {
    fn to_string(&self) -> serde_json::Result<String> {
        to_string(self)
    }
}

struct ServerWorker {
    addr: SocketAddr,
    worker_uploads: HashSet<String>,
    connection_socket: TcpStream,
    server_id: String,
    uploads: Arc<Mutex<HashMap<String, PartialUpload>>>,
}

impl ServerWorker {
    fn new(
        client: (TcpStream, SocketAddr),
        server_id: String,
        uploads: Arc<Mutex<HashMap<String, PartialUpload>>>,
    ) -> Self {
        let (connection_socket, addr) = client;
        Self {
            addr,
            uploads,
            worker_uploads: set![],
            connection_socket,
            server_id,
        }
    }
    fn receive(&mut self) -> Result<Command, Box<dyn Error>> {
        let mut s = "".to_string();
        let start = Instant::now();
        while !(s.ends_with("\n")) {
            if (Instant::now() - start).as_millis() <= 1000 {
                Err("Timeout while receiving")?;
            }
            self.connection_socket.read_to_string(&mut s)?;
            thread::sleep(Duration::from_millis(100));
        }
        Ok(Command::from_string(s)?)
    }
    fn send(&mut self, response: Response) -> Result<(), Box<dyn Error>> {
        self.connection_socket
            .write(response.to_string()?.as_bytes())?;
        OK
    }
    fn handle(&self, command: Command) -> Result<Response, Box<dyn Error>> {
        match command {
            Command::Filelist => {
                let filelist: Vec<String> = std::fs::read_dir(served(self.server_id.clone(), ""))?
                    .filter_map(|f| match f {
                        Ok(f) => {
                            if f.file_type().unwrap().is_dir() {
                                None
                            } else {
                                Some(f.file_name().to_str().unwrap().to_string())
                            }
                        }
                        Err(_) => None,
                    })
                    .collect();
                Ok(Response::Filelist {
                    code: ResponseCode::K200("".to_string()),
                    filenames: filelist,
                })
            }
            Command::UploadDeclare { filename, bytes } => {
                let mut uploads = self.uploads.lock().unwrap();
                if uploads.contains_key(&filename) {
                    return Ok(Response::Upload {
                        code: ResponseCode::K250(format!("Currently receiving file {}", filename)),
                    });
                }
                if served(self.server_id.clone(), filename.clone()).exists() {
                    return Ok(Response::Upload {
                        code: ResponseCode::K250(format!("Already serving file {}", filename)),
                    });
                }
                uploads.insert(
                    filename.clone(),
                    PartialUpload::new(filename.clone(), bytes, self.server_id.clone()),
                );
                Ok(Response::Upload {
                    code: ResponseCode::K200(format!("Ready to receive file {}", filename)),
                })
            }
            Command::UploadContent {
                filename,
                chunk,
                content,
            } => {
                let mut uploads = self.uploads.lock().unwrap();
                if !uploads.contains_key(&filename) {
                    return Ok(Response::Upload {
                        code: ResponseCode::K250(format!("Upload for {} not initiated", filename)),
                    });
                }
                uploads[&filename].save(chunk, content);
                Err("Todo")?
            }
            Command::DownloadDeclare { filename } => todo!(),
            Command::DownloadContent {
                filename,
                chunk,
                content,
            } => todo!(),
            Command::Delete { filename } => todo!(),
        }
    }
    fn run(&mut self) -> Result<(), &'static str> {
        Err("Not implemented")
    }
}

fn main() -> Whoops {
    let mut id = "".to_string();
    let mut debug = false;
    for (i, arg) in env::args().enumerate() {
        match i {
            1 => id = arg,
            2 => debug = arg == "debug",
            _ => {}
        }
    }
    let (addr, port) = read_settings()[&id].clone();
    Server::new(id, addr, port, debug).run()
}
