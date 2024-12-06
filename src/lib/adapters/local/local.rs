use tokio::process::{Command, Child, ChildStdin, ChildStdout};
use tokio::io::{AsyncBufReadExt, BufReader, Lines, BufWriter};
use std::process::Stdio;
use std::path::Path;
use thiserror::Error;

pub struct Local {
    _child: Child,
    pub reader: Lines<BufReader<ChildStdout>>,
    pub writer: BufWriter<ChildStdin>,
}

#[derive(Debug, Error)]
pub enum CreateLocalProgramError {
    #[error("create subprocess error: `{0}`")]
    CreateSubprocessError(#[from] tokio::io::Error),
}

impl Local {
    pub async fn new(binary: &Path) -> Result<Self, CreateLocalProgramError> {
        let mut child = Command::new(binary)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .kill_on_drop(true)
            .spawn()?;

        let stdout = child.stdout.take().expect("");
        let stdin = child.stdin.take().expect("");

        let reader = BufReader::new(stdout).lines();
        let writer = BufWriter::new(stdin);
        
        Ok(Self {
            _child: child,
            reader,
            writer,
        })
    }
}
