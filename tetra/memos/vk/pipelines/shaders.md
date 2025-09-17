# Executing code on the GPU

## Shaders

A shader is a chunk of code for GPU (source or compiled code can be designated as shaders).

Coding a shader in Vulkan follow these steps :
1. Write a shader in `GLSL` or `HLSL` (high-level shader languages).
2. Compile it to `SPIR-V` binary (harware-agnostic hal-compiled language).
3. Create `vk::ShaderModule` from it.

Then shader modules can be used by other vulkan functions.

## `vk::ShaderModule`

`vk::ShaderModule` := shader code + one or more entry points.

An entry point is a function to start from when executing code (usually called `main`).