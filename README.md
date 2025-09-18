# Folders

There's multiple folders because the main project use some custom library crates :
- Main project is called "tetra".
- Others are library crates (mostly independent).
- "bvh" is not yet used by "tetra" but working.

They all have a README if you're interested in one of them.

# Launch

Need :
- rustc (rust compiler)
- cargo (rust package, called "crate", manager)
- vulkan drivers

Download whole project :

```bash
git clone https://github.com/CireTheBoos/project
```

Open terminal in "project/tetra" and type :

```bash
cargo run -r
```

This will compile every dependencies and launch the executable in release mode.

**Press ESCAPE to focus/unfocus once window appears.**
