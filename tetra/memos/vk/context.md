# Context

Vulkan need to be "configured" before being used. Configuration is applied in two phases :
- **Instance** creation.
- **Device** creation.

Vulkan functions are categorized based on the configuration they need before being called :
- *entry-level* : No configuration needed.
- *instance-level* : Need an instance.
- *device-level* : Need an instance and a device.

## `Instance`

When creating an instance, we configure :
- **Application info** (API version) := About the application.
- **Layers** := Code inserted before of after function calls (Ex: Validation layer insert some checks).
- **Instance extensions** := Additionnal instance-level functions (Ex: Appropriate display extensions are needed to create a surface).

### Application info

```rust
    //-----// get instance API version //-----//

    let instance_api_version: u32 = unsafe { entry.try_enumerate_instance_version()? }.unwrap_or(vk::API_VERSION_1_0),
```

### Layers

```rust
    //-----// get supported layers //-----//

    let layers: Vec<vk::LayerProperties> = unsafe { entry.enumerate_instance_layer_properties()? };
```

### Instance extensions

```rust
    //-----// get supported instance extensions //-----//

    let instance_extensions: Vec<vk::ExtensionProperties> = unsafe { entry.enumerate_instance_extension_properties(None)? };
```

## `Device`

When creating a device, we configure :
- **Device extensions** :=  Additionnal device-level functions (Ex: Swapchain functions to present images to the screen).
- **Features** := Allow the use of certain hardware features (Ex: Ray tracing).
- **Queue families** := Will provide queues to execute the work, impact synchronization.

We also *check support* for :
- Surface presentation.
- Formats.

Device configuration is more complex because :
- It has *arguments* (`vk::PhysicalDevice` and `vk::SurfaceKHR`):
    - Instance only check support for a configuration known upfront.
    - Device need to *complete* its configuration with data queried using the arguments.
- We may have several device configurations (because of different arguments).

### Device extensions

```rust
    //-----// get supported device extensions //-----//

    let device_extensions: Vec<vk::ExtensionProperties> = unsafe { instance.enumerate_device_extension_properties(physical_device)? };
```

### Features

```rust
    //-----// get supported features //-----//

    // structures
    let vulkan_10_features = vk::PhysicalDeviceFeatures::default();
    let mut vulkan_11_features = vk::PhysicalDeviceVulkan11Features::default();
    let mut vulkan_12_features = vk::PhysicalDeviceVulkan12Features::default();
    let mut vulkan_13_features = vk::PhysicalDeviceVulkan13Features::default();
    let mut features = vk::PhysicalDeviceFeatures2::default()
        .features(vulkan_10_features)
        .push_next(&mut vulkan_11_features)
        .push_next(&mut vulkan_12_features)
        .push_next(&mut vulkan_13_features);

    // set each field to `vk::TRUE` if supported (`vk::FALSE` if not)
    unsafe {
        instance.get_physical_device_features2(physical_device, &mut features)
    };
```
### Queue families

Queue flags : Some queue families are general (it's common to have a queue family with all flags) and some are more specialized (map to underlying specialized hardware, with less flags).

Queue families can differ on :
- Their flags.
- Their maximum number of queues.
- Their support of the surface.
- Etc.

More queues might impact parallelism and performance but it also impact synchronization need.

```rust
    //-----// query queue families data (flags, max queues, etc.) //-----//

    let queue_family_properties = unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

    //-----// check support for surface //-----//

    let surface_support = unsafe { surface_instance.get_physical_device_surface_support(physical_device, queue_family_index, surface)? };
```

### Formats

```rust
    // structures
    let mut format_properties = vk::FormatProperties3::default();
    let mut format_properties2 = vk::FormatProperties2::default().push_next(&mut format_properties);

    // set bit masks (per tiling : optimal or linear)
    unsafe {
        instance.get_physical_device_format_properties2(
            physical_device,
            format,
            &mut format_properties2,
        )
    };
```

## `Swapchain`

A swapchain is a collection of images used for presentation on screen using *multiple-buffering* : One image is presented whereas the others can be written to.

Changing the current image being presented is called **swapping**.

Because this is such a common pattern, Vulkan encapsulate it into a `vk::Swapchain` object.

### Creation

The vulkan swapchain is a structure with a big configuration (~15 arguments) :
- `min_image_count` : Minium number of images in the swapchain.
- `present_mode` : Swapping logic.
- `image_format`: Images format (how pixels are stored).
- `image_color_space`: Images color space (how pixels should be interpreted).
- `image_usage`: Images usage (how images can be used).
- `surface` : The surface to rendered to.
- `queue_family_indices` : Queue families allowed to access images.
- `image_sharing_mode` : sharing mode of the queue families.
- `old_swapchain` : For quicker swapchain recreation. 
- `flags` (usually: `empty`) : For stereo rendering or dynamic changes in format.
- `image_array_layers` (usually `1`): For stereo rendering.
- `image_extent` (usually `surface.capabilities.current_extent`): Extent of the images.
- `pre_transform` (usually `surface.capabilities.current_transform`): Transform to apply before presenting.
- `composite_alpha` (usually `OPAQUE`): Transparency with windows in the back, not that well supported.
- `clipped` (usually `true`) : Clip the image if it's partially offscreen.

### Arguments

#### `present_mode`

To understand present modes, we must first understand these :
- v-sync := Moment when the screen is not writing and we can safely swap presented image (usually 60).
- Tearing := When screen is half an image, half another.
- Input lag := Lag between action and reaction on screen.

The main present modes are :

- **IMMEDIATE** : Swap immediately without waiting v-sync.
    - Might provoke tearing.
    - Minimal input lag.
    - No limits on GPU load (multiple rendering per screen frame).

- **FIFO** : Queue images and wait v-sync to swap.
    - No tearing.
    - Maximum input lag (depends on numbers of images in the swapchain).
    - Minimal GPU load (one rendering per refresh).

- **MAILBOX** : Swap newest image during v-sync.
    - No tearing.
    - Low input lag (refresh rate at most).
    - No limits on GPU load (multiple rendering per screen frame)

"Multiple rendering per screen frame" is possible only if the GPU can process frames faster than the refresh rate of the screen.



