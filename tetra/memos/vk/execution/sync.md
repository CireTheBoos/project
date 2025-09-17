# Sync

## Scopes

They are 2 level of dependencies in vulkan :
1. Execution dependencies := A operations *happen-before* B operations.
2. Memory dependencies (stronger execution dependencies that make caches coherent) :=
    - A operations
    - *happen-before* data X-accessed by A is made available (flushed from A caches)
    - *happen-before* data that will be Y-accessed by B is made visible (invalidated from B caches)
    - *happen-before* B operations (will cache updated data)

These dependencies are constructed using scopes :
- **First synchronization scope** := Stages to select A prior in submission order.
- **Second synchronization scope** := Stages to select B next in submission order.
- **First access scope** := Accesses by A that will be made available after A.
- **Second access scope** := Accesses by B that need to be visible before B.

## Pipeline barriers

Execution and memory barriers. Use sync2.

## Swapchain

For swapchain-related sync functions, *timeline semaphores are not supported*.

### `acquire_next_image(..)`

Here I use **available** when an image can be rendered onto and **acquirable** when it can be acquired but might not be available for render yet.
Might differ from online definitions (available is used for both kinda).

```rust
let (swapchain_image_index, is_swapchain_suboptimal) = unsafe {
    device.swapchain_device.acquire_next_image(
        swapchain,
        timeout,
        image_ready_binary_semaphore,
        image_ready_fence,
    )
}?;
```

Block execution if no images can be acquired, 3 strategies for this :
- **Wait** (`timeout = u64::MAX`) : Block indefinitely until an image is *acquirable*. Might softlock the application.
- **Try once** (`timeout = 0`) : Try once and handle the result.
- **Try until** (`timeout = SOME_TIME_NANOSECONDS`) : Try until some time has passed. Not common.

Provided image might not be *available* (ex: it can be currently on screen) :
- `image_ready_binary_semaphore` and/or `image_ready_fence` to signal availability.
- We *must* use one of them (can't set them both to `null`).

No need for `acquire_next_image2()`. It only adds a device mask for device groups (= multiple physical devices under one device).

### `queue_present(..)`

```rust
// present info
let swapchains = [swapchain];
let image_indices = [image_index];
let wait_binary_semaphores = [..];
let present_info = vk::PresentInfoKHR::default()
    .swapchains(&swapchains)
    .image_indices(&image_indices)
    .wait_semaphores(&wait_binary_semaphores);

// present
let is_swapchain_suboptimal = unsafe {
    device.swapchain_device.queue_present(
        present_queue,
        &present_info,
    )
}?;
```

It can present *multiple swapchains*. If not the case (most of the time) :
- Use the return value, not the `results` field of `present_info` (store potentially multiple swapchain results). 
- Use singleton arrays for `swapchains` and `image_indices`.

The queue must be from a queue family that supports the surface.

### "present semaphores can't be recycled" problem

This is not its official name.

It happens when we use a per-frame semaphore to know when an image can be presentable.

We queue the following sync constraints on the GPU :
1. signal availability (of the swapchain image) (when calling `acquire_next_image(..)`)
2. wait availability
3. render
4. signal presentability & fence end of rendering (last 3 when calling `submit(rendering_work)`)
5. wait presentability
6. present (when calling `queue_present(..)`)

BUT the frame might loop after 4. and redo the first 4 steps BEFORE 5.
=> We tell the queue to signal presentability but it has not yet waited on it.

So this is UB.

Solutions :
- Have presentable binary semaphores per-image (and not per-frame).
    - Good and the most common solution, not that clean nonetheless.
- Timeline semaphore (as we can queue `signal n+1` without having passed `wait n`).
    - But not supported for swapchains.
- Use a fence on `queue_present(..)` and not `submit(rendering_work)`.
    - Clean and maybe just as performant.
    - But only possible with an extension `VK_EXT_SWAPCHAIN_MAINTENANCE1`.
