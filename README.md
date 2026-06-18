# Pipe Utils 
This Library is made to help write cross platform programms using the new `pipe()` call in rust. 
It helps with duplicating file descriptors to mainly aid in redirecting stdin, stdout and stderr to child processes or chaining subproccesses together. This library also aids in redirecting Pipe input into a file or accessing the content of a file via a pipes reading end 

## Examples
```rust
let file = File::open("foo");
let file_reader = file_to_pipe_reader(file);

let command = Command::new("grep")
            ...
            .stdin(file_reader);    
```

The internal library can also be used to duplicate arbitrary Handles and File Descriptors.
```rust
let pipe_writer = pipe_utils::os::dup_handle_to_pipe_writer(handle);
```

If you need to use the running pocesses stdin and stdout as a writer/reader this is also possible.
This is usefull when dealing with recursive calls where every call of the recursion needs a PipeWriter or PipeReader

```rust
let (stdin, stdout, stderr) = get_process_stdio_as_pipe();

// ...
let command = Command::new("...")
    .stdin(stdin)
    .stdout(stdout)
    .stderr(stderr);
```