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
    use std::io::{self, PipeReader, PipeWriter};

    pub fn dup_stdin_to_pipe_reader() -> io::Result<PipeReader> {
        super::dup_fd_to_pipe_reader(io::stdin())
    }

    pub fn dup_stdout_to_pipe_writer() -> io::Result<PipeWriter> {
        super::dup_fd_to_pipe_writer(io::stdout())
    }

    pub fn dup_stderr_to_pipe_reader() -> io::Result<PipeReader> {
        super::dup_fd_to_pipe_reader(io::stderr())
    }
}
