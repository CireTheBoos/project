# Folders

There's multiple folders because the main project is splitted into a binary crate and multiple custom library crates.

You can explore the ones you want to know more about (they all have READMEs) :
- "tetra" (main project) : Graphics renderer.
- "suballocation" : Allocate/Reallocate/Deallocate regions of a given memory buffer, for objects that changes in size at runtime.
- "bvh" : Bounding Volume Hierachy, an acceleration struture used in ray tracing.
- ".._utils" : Just some utilities, not that interesting.

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
