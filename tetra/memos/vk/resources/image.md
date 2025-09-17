# Images

## `vk::Image`

**Layers** : Images can be put together as an array (might be useful for cube maps or animation).

**Mipmap** : Images of smaller resolution for use in far away distances.

```rust
    let create_info = vk::ImageCreateInfo::default()
        // layout
        .tiling(tiling)
        .initial_layout(initial_layout)
        .usage(usage)
        // data
        .image_type(image_type)
        .extent(extent)
        .format(format)
        .array_layers(array_layers)
        .mip_levels(mip_levels)
        .samples(samples)
        // sharing
        .queue_family_indices(queue_family_indices)
        .sharing_mode(sharing_mode)
        // optional
        .flags(flags)
        .push_next(next);
```

Layout impacting :
- `tiling` := 
    - `LINEAR` (stored row after row, best for sequential access like copy with CPU).
    - `OPTIMAL` (stored by 2x2 groups *for ex*, best for rendering accesses).
- `initial_layout` := Hint for optimization.
- `usage` := Hint for optimization.

Data :
- `image_type` := 1,2, or 3D.
- `array_layers`

## `vk::ImageView`

```rust
    let create_info = vk::ImageViewCreateInfo::default()
        .image(image)
        .components(components)
        .format(format)
        .subresource_range(subresource_range)
        .view_type(view_type)
        // optional
        .flags(flags)
        .push_next(next);
```

