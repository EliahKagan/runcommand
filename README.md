# runcommand - Simple `Command::spawn` wrapper for manual testing

It can be useful to examine the behavior of `std::process::Command` in the Rust standard library, such as how it performs path search on some operating systems.

This utility uses `Command` to run the command specified in its arguments. This is as simple as possible, like [`useenv`](ihttps://github.com/EliahKagan/useenv) but without the environment modification and without recognizing any options of its own.

## Usage

```sh
runcommand COMMAND ARGS...
```

## License

[0BSD](LICENSE)
