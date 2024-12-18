use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, BufWriter};
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

/// An implementation of a player interface for a local binary launch by the
/// gomokurs manager.
pub struct Local {
    /// The AI local process handled by the player interface.
    _child: Child,
    /// The buffer reader of the AI local process.
    pub reader: Arc<Mutex<Lines<BufReader<ChildStdout>>>>,
    /// The buffer writer of the AI local process.
    pub writer: Arc<Mutex<BufWriter<ChildStdin>>>,
}

/// An error returned by the player interface when it failed to instantiate
/// itself.
#[derive(Debug, Error)]
pub enum CreateLocalProgramError {
    /// An error returned when the player interface failed to execute the AI
    /// binary as a child process.
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl Local {
    /// Create a new local player interface from a AI binary path.
    /// 
    /// # Arguments
    /// 
    /// * `binary` - The path to the local AI binary.
    pub async fn new(binary: &Path) -> Result<Self, CreateLocalProgramError> {
        let mut child = Command::new(binary)
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