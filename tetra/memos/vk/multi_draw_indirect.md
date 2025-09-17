# Multi draw indirect (MDI)

Since Vulkan 1.2, draw parameters can be put in a buffer and issued from a single multi-draw call.

It reduces CPU overhead for large number of draw calls and allows GPU culling through compute shaders.

## `device.cmd_draw_[indexed]_indirect_[count](..)`

Draw indirect command has 4 versions because 2 parameters can be set on/off :
- **indexed** := Use an index buffer or not. Same arguments but different draw indirect command structure (indexed or not).
- **count** := Fetch the draw count from a buffer. `draw_count` become `count_buffer` + `count_buffer_offset` + `max_draw_count`.

```rust
// without GPU count (same arguments for indexed)
unsafe { device.cmd_draw_indirect(
    command_buffer,
    buffer,
    offset,
    draw_count,
    stride
) };

// with GPU count (same arguments for indexed)
unsafe { device.cmd_draw_indirect_count(
    command_buffer,
    buffer,
    offset,
    count_buffer,
    count_buffer_offset,
    max_draw_count,
    stride,
) };
```

Note the `stride` parameters, it allows us to put the `vk::Draw[Indexed]IndirectCommand` structure inside a bigger structure if we want.

### `vk::Draw[Indexed]IndirectCommand`

Draw indirect command *structure* has 2 versions because 1 paremeters can be set on/off :
- **indexed** : Use an index buffer or not.

```rust
// non-indexed
let draw_indirect_command = vk::DrawIndirectCommand::default()
    .first_vertex(first_vertex)
    .vertex_count(vertex_count)
    .first_instance(first_instance)
    .instance_count(instance_count);

// indexed
let draw_indexed_indirect_command = vk::DrawIndexedIndirectCommand::default()
    .first_index(first_index)
    .index_count(index_count)
    .vertex_offset(vertex_offset)
    .first_instance(first_instance)
    .instance_count(instance_count);
```

By setting `instance_count` to 0, we can skip rendering. Useful for GPU culling.