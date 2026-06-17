use std::{
    io::{self, PipeReader, PipeWriter},
    os::windows::io::{AsHandle, OwnedHandle},
};

pub fn dup_handle(as_handle: impl AsHandle) -> io::Result<OwnedHandle> {
    as_handle.as_handle().try_clone_to_owned()
}

pub fn dup_handle_to_pipe_reader(as_handle: impl AsHandle) -> io::Result<PipeReader> {
    dup_handle(as_handle).map(|cloned_fd| PipeReader::from(cloned_fd))
}

pub fn dup_handle_to_pipe_writer(as_handle: impl AsHandle) -> io::Result<PipeWriter> {
    dup_handle(as_handle).map(|cloned_fd| PipeWriter::from(cloned_fd))
}

pub(crate) mod imp {
    use super::{dup_handle, dup_handle_to_pipe_reader, dup_handle_to_pipe_writer};
    use std::io::{self, PipeReader, PipeWriter};

    pub fn file_to_pipe_writer(file: File) -> io::Result<PipeWriter> {
        dup_handle(file).map(|cloned_fd| PipeWriter::from(cloned_fd))
    }

    pub fn file_to_pipe_reader(file: File) -> io::Result<PipeReader> {
        dup_handle(file).map(|cloned_fd| PipeReader::from(cloned_fd))
    }

    pub fn dup_stdin_to_pipe_reader() -> io::Result<PipeReader> {
        dup_handle_to_pipe_reader(io::stdin())
    }

    pub fn dup_stdout_to_pipe_writer() -> io::Result<PipeWriter> {
        dup_handle_to_pipe_writer(io::stdout())
    }

    pub fn dup_stderr_to_pipe_reader() -> io::Result<PipeReader> {
        dup_handle_to_pipe_reader(io::stderr())
    }
}
