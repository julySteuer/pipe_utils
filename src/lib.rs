use std::io::{self, PipeReader, PipeWriter};

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod os;

#[cfg(not(target_os = "windows"))]
#[path = "unix.rs"]
mod os;

pub fn dup_stdin_to_pipe_reader() -> io::Result<PipeReader> {
    os::dup_stdin_to_pipe_reader()
}

pub fn dup_stdout_to_pipe_writer() -> io::Result<PipeWriter> {
    os::dup_stdout_to_pipe_writer()
}

pub fn dup_stderr_to_pipe_reader() -> io::Result<PipeReader> {
    os::dup_stderr_to_pipe_reader()
}

pub fn get_process_stdio_as_pipe() -> io::Result<(PipeReader, PipeWriter, PipeReader)> {
    let stdin = dup_stdin_to_pipe_reader()?;
    let stdout = dup_stdout_to_pipe_writer()?;
    let stderr = dup_stderr_to_pipe_reader()?;
    Ok((stdin, stdout, stderr))
}
