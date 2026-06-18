# Pipe Utils 
This Library is made to help write cross platform programms using the new `pipe()` call in rust. 
It helps with duplicating file descriptors to mainly aid in redirecting stdin, stdout and stderr to child processes or chaining subproccesses together. This library also aids in redirecting Pipe input into a file or accessing the content of a file via a pipes reading end 