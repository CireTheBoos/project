# Dynamic rendering

## Graphics pipeline creation

```rust
let mut rendering_info = vk::PipelineRenderingCreateInfo::default()
    .color_attachment_formats(color_attachment_formats)
    .depth_attachment_format(depth_attachment_format)
    .stencil_attachment_format(stencil_attachment_format)
    .view_mask(view_mask);
```

Set format to `vk::Format::UNDEFINED` if it's not going to be used.
- `view_mask` : Used in multi-view scenario.