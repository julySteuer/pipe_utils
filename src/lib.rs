use std::{
    fs::File,
    io::{self, PipeReader, PipeWriter, pipe},
    process::Command,
};

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
pub mod os;

#[cfg(not(target_os = "windows"))]
#[path = "unix.rs"]
pub mod os;

/*
 * Pipes the stdout of frst to the stdin of scnd
 * the stdin of frst and stdout of scnd are not set so they have to be set before / after this function
 */
pub fn pipe_commands(frst: &mut Command, scnd: &mut Command) -> io::Result<()> {
    let (pipe_reader, pipe_writer) = pipe()?;
    frst.stdout(pipe_writer);
    scnd.stdin(pipe_reader);
    Ok(())
}

/*
 * Helpful when you have to write input from a pipe to a file stdout -> File
 */
pub fn file_to_pipe_writer(file: File) -> io::Result<PipeWriter> {
    os::imp::file_to_pipe_writer(file)
}

/*
 * Helpful when you want to read from a file in a pipe File -> stdin 
 */
pub fn file_to_pipe_reader(file: File) -> io::Result<PipeReader> {
    os::imp::file_to_pipe_reader(file)
}

/*
 * Duplicates the proceeses stdin and wraps it into a PipeReader
 */
pub fn dup_stdin_to_pipe_reader() -> io::Result<PipeReader> {
    os::imp::dup_stdin_to_pipe_reader()
}

/*
 * Duplicates the proceeses stdout and wraps it into a PipeWriter
 */
pub fn dup_stdout_to_pipe_writer() -> io::Result<PipeWriter> {
    os::imp::dup_stdout_to_pipe_writer()
}

/*
 * Duplicates the proceeses stderr and wraps it into a PipeReader
 */
pub fn dup_stderr_to_pipe_reader() -> io::Result<PipeReader> {
    os::imp::dup_stderr_to_pipe_reader()
}

/*
 * Duplicates and returns stdin, stdout and stderr as PipeReader, PipeWriter and PipeReader.
 * This can be used to directly use them in a command
 */
pub fn get_process_stdio_as_pipe() -> io::Result<(PipeReader, PipeWriter, PipeReader)> {
    let stdin = dup_stdin_to_pipe_reader()?;
    let stdout = dup_stdout_to_pipe_writer()?;
    let stderr = dup_stderr_to_pipe_reader()?;
    Ok((stdin, stdout, stderr))
}
