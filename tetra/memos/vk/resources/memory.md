# Memory

Basic vocabulary :
- **Cache** := Copy from memory to cache (for reads).
- **Flush** := Copy from cache to memory (for writes).
- **Invalidate** := Mark as invalid (Need to cache again if reading).

GPU vocabulary :
- **Coalesce** := Group memory requests from contiguous threads (sharing same cache) into a single one.

Memory is viewed through :
- `vk::MemoryHeap`. Describes actual physical memory locations :
    - Size (in bytes).
    - `vk::MemoryHeapFlags` (unused in most cases).
- `vk::MemoryType`. More practical, describes memory behaviours :
    - Heap (=> size).
    - `vk::MemoryPropertyFlags`.

## Allocating memory

Then memory of a given type is allocated if compatible with the resource using `vk::MemoryRequirements`.

## `vk::MemoryPropertyFlags`

### `DEVICE_LOCAL`

Located on the most efficient storage for GPU access.

### `HOST_VISIBLE`

Can be mapped (:= accessed by CPU).

Usually not DEVICE_LOCAL, if it is then it's small.

### `HOST_CACHED` (=> HOST_VISIBLE)

Cached by CPU.

### `HOST_COHERENT` (=> HOST_VISIBLE)

Do not need manual CPU cache management.

```rust
    //---// Manual CPU cache management //---//

    // flush CPU writes
    device.flush_mapped_memory_ranges(..);

    // invalidate for future CPU reads
    device.invalidate_mapped_memory_ranges(..);
```

## `vk::MemoryRequirements`

Size

Alignement

Compatible memory types (depends on resource).

