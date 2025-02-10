use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, BufWriter};
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Represents a local player interface for running a Gomoku AI binary as a
/// subprocess managed by the Gomoku manager.
pub struct Local {
    /// The child process representing the AI binary.
    _child: Child,
    /// The buffered reader for the AI process's standard output.
    pub reader: Arc<Mutex<Lines<BufReader<ChildStdout>>>>,
    /// The buffered writer for the AI process's standard input.
    pub writer: Arc<Mutex<BufWriter<ChildStdin>>>,
}

/// An error type for issues encountered when creating the local player
/// interface.
#[derive(Debug, Error)]
pub enum CreateLocalProgramError {
    /// An error occurred when spawning the AI binary as a subprocess.
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl Local {
    /// Creates a new instance of the local player interface by launching the AI
    /// binary.
    /// 
    /// # Arguments
    ///
    /// * `binary` - The path to the AI binary to be executed as a subprocess.
    pub async fn new(binary: &Path, args: Vec<String>) -> Result<Self, CreateLocalProgramError> {
        let mut child = Command::new(binary)
            .args(&args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        // TODOs: handle stdout and stdin errors
        let stdout = child.stdout.take().expect("");
        let stdin = child.stdin.take().expect("");
        
        Ok(Self {
            _child: child,
            reader: Arc::new(Mutex::new(BufReader::new(stdout).lines())),
            writer: Arc::new(Mutex::new(BufWriter::new(stdin))),
        })
    }
}