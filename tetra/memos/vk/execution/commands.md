# Commands

## `vk::CommandBuffer`

Chain of commands.

Recording order matters : For sync and state management.

### State

**State** := All *indirect* parameters.

```rust
// state is *empty*
device.begin_command_buffer(..);

// change state
device.cmd_bind_pipeline(..);
device.cmd_set_viewport(..);
device.cmd_push_constants(..);

// use state
device.cmd_draw(..);
```

### Level

`vk::CommandBufferLevel` := 
- `PRIMARY` : Submittable.
- `SECONDARY` : Callable by cmdbufs (primary or secondary). Inherit state.

Goal : *Avoid repetition*. Exactly like using a function instead of rewriting the same code multiple times.

## `vk::CommandPool`

Commands memory allocator.

*Single-thread use* (recording is a usage). To get parallel command recording, use different command pools.

### Creation

```rust
let create_info = vk::CommandPoolCreateInfo::default()
    .queue_family_index(queue_family_index)
    .flags(flags);
```

- `queue_family_index` := Intended queue family for submission.
- `flags` :
    - `RESET_COMMAND_BUFFER` : Command buffers can be individually reset (more overhead than full reset).
    - `TRANSIENT` : Commands buffers will be short-lived.
    - `PROTECTED` : ? Related to ownership and resources ?
