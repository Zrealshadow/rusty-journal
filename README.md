# Rusty-Journal

A simple command-line to-do list program.

Final Project of Microsoft Rust Tutorial : [Take your first steps with Rust](https://docs.microsoft.com/en-us/learn/paths/rust-first-steps/)



**Usage**

Use Cargo to build 

```shell
cargo build
```



Use Cargo to run or  running executable program

```shell
cargo run
```

under the project directory, run executable program

```shell
./target/debug/rusty-journal
```



Help info will be printed in terminal

```
Rusty Journal 0.1.0
A command line to-do app written in Rust

USAGE:
    rusty-journal [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -j, --journal-file <journal-file>    

SUBCOMMANDS:
    add     Write tasks to the journal file
    done    Remove an entry from the journal file by position
    help    Prints this message or the help of the given subcommand(s)
    list    List all tasks in the journal file
```





**Note**

When use `cargo run`, please do not forget to add `--` to ensure that all the arguments passed after `--` will be sent to our program instead of `cargo` itself.

For example

```shell
cargo run -- -j test-journal.json add "Learning Mit 6.824"
```



