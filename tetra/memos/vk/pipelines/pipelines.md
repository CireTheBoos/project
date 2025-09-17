# Executing code on the GPU

There are 2 types of pipelines :
- **Graphics pipeline** with :
    - 2-4 stages : vertex, fragment, geometry, tesselation, etc.
    - 6-8 states : color blending, viewport, etc.
- **Compute pipeline** with a single stage.

**Stage** := Shader, programmable part of a pipeline.

**State** := Fixed function, code already included and hardware accelerated.

## Pipeline creation

**Derivation** := We can use an existing pipeline to create a new one.

**Caching** := Storing compiled pipeline binaries to recreate faster.

`vk::GraphicsPipelineCreateInfo` :
- TODO

