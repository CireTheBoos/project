# Multiple files

There's multiple files because I'm defining some helper library crates. The main project is called "tetra".

# Launch

Need :
- rustc (rust compiler)
- cargo (rust package, called "crate", manager)
- vulkan drivers

Download folder :

```bash
git clone https://github.com/CireTheBoos/project
```

Open terminal in "tetra" and type :

```bash
cargo run -r
```

This will compile every dependencies and launch the executable.
(`-r` for "release mode" as opposed to "debug mode" that contains additional checks)
