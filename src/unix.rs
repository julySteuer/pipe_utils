use std::{
    io::{self, PipeReader, PipeWriter},
    os::fd::{AsFd, OwnedFd},
};

fn dup_fd(as_fd: impl AsFd) -> io::Result<OwnedFd> {
    as_fd.as_fd().try_clone_to_owned()
}

pub fn dup_fd_to_pipe_reader(as_fd: impl AsFd) -> io::Result<PipeReader> {
    dup_fd(as_fd).map(|cloned_fd| PipeReader::from(cloned_fd))
}

pub fn dup_fd_to_pipe_writer(as_fd: impl AsFd) -> io::Result<PipeWriter> {
    dup_fd(as_fd).map(|cloned_fd| PipeWriter::from(cloned_fd))
}

pub(crate) mod imp {
    use super::{dup_fd, dup_fd_to_pipe_reader, dup_fd_to_pipe_writer};
    use core::convert::From;
    use std::{
        fs::File,
        io::{self, PipeReader, PipeWriter},
        process::ChildStdout,
    };

    pub fn file_to_pipe_writer(file: File) -> io::Result<PipeWriter> {
        dup_fd(file).map(|cloned_fd| PipeWriter::from(cloned_fd))
    }

    pub fn file_to_pipe_reader(file: File) -> io::Result<PipeReader> {
        dup_fd(file).map(|cloned_fd| PipeReader::from(cloned_fd))
    }

    pub fn dup_stdin_to_pipe_reader() -> io::Result<PipeReader> {
        dup_fd_to_pipe_reader(io::stdin())
    }

    pub fn dup_stdout_to_pipe_writer() -> io::Result<PipeWriter> {
        dup_fd_to_pipe_writer(io::stdout())
    }

    pub fn dup_stderr_to_pipe_reader() -> io::Result<PipeReader> {
        dup_fd_to_pipe_reader(io::stderr())
    }

    pub fn child_stdout_to_pipe_reader(child_stdout: ChildStdout) -> io::Result<PipeReader> {
        dup_fd_to_pipe_reader(child_stdout.as_fd())
    }
}
