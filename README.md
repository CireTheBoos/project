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

**Press ESCAPE to focus/unfocus once window appears.**

# Project

![tetra_screenshot](./tetra_screenshot.png)

Features :
- **Memos** : See inside "tetra/memos" for all my vulkan notes. I try to understand deeply every concepts.
- Basic per triangle rendering (currently using geometry shader but I will change that for duplicate vertices and `gl_VertexID / 3` trick).
- Controls camera + ZQSD + Shift + Space (camera lacks a bit of fluidity).
- Resizable window.
- Depth testing.
- Dynamic addition of meshes (cubes are added after renderer creation).
- Multi-Draw Indirect.
- Custom vulkan context (instance, device creation). I didn't know of Vulkan Bootstrap at the time.
- Memory management using VMA.
- dynamic rendering (I don't know well render passes).

Just a basic renderer for now, with premises of dynamic mesh edit.
