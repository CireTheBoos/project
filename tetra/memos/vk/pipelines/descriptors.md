# Descriptors

Descriptors are used for *resource binding*.

**Descriptor** := General term in CS meaning a structure that describe how to access a resource.
Used in context when we need more information than a pointer or index (Ex: File location, memory layout, etc.).

In vulkan, we use descriptors for :
- Flexibility : Adds a level of indirection to memory locations (pointing to a descriptor that points to memory).
- Performance : Contains resource layout.
- Optimization : Can be grouped and managed more intelligently.

They are 3 main structures to manage descriptors in vulkan :
- `vk::DescriptorPool` : Stores descriptors.
- `vk::DescriptorSetLayout` : Describes the layout of a set of descriptors.
- `vk::DescriptorSet` : A set of descriptors.

These structures need descriptor-related data but *there's no descriptor structure in vulkan*.
It's up to the programmer to organize descriptor data.

Vulkan allow **array descriptor** := One descriptor for an array of resources.

## `vk::DescriptorPool`

Manages a chunk of memory where descriptors can be stored.

Creation arguments :
- List of `vk::PoolSize`.
    - `vk::PoolSize` := `vk::DescriptorType` + count.
    - If multiple pool sizes have the same descriptor type, their count will be summed.
    - One array descriptor = Multiple descriptors.
- Some flags eventually.

```rust
/// PSEUDO CODE ///
// `#` = 1 byte.
// descriptor types (size) : A (3 bytes), B (1 byte).

pool_sizes = [{type: A, count: 2},{B, 5},{A, 1}];
descriptor_pool = create_descriptor_pool(pool_sizes);
// This pool has :
// - 14 bytes of memory total.
// - Space for 3 descriptors of type A and 5 of type B. 

descriptor_pool.memory -> ##############
```

## `vk::DescriptorSetLayout`

Represents the layout of a set of descriptors.

The fact that different descriptor sets can share the same layout is used when we want to optimize resources binding.
They act as function signature for the pipelines, where descriptor sets are arguments.

They are provided to :
- Descriptor pool, when allocating a descriptor set.
- Pipeline layout.

Creation arguments :
- List of `vk::DescriptorSetLayoutBinding`.
    - `vk::DescriptorSetLayoutBinding` := descriptor layout + descriptor shader access.
        - layout := `vk::DescriptorType` + array count (1 if not an array).
        - access := `vk::ShaderStageFlags` + binding.
- Some flags eventually.

```rust
/// PSEUDO CODE ///
// `#` = 1 byte.
// descriptor types (size) : A (3 bytes), B (1 byte).

bindings = [{type: A, count: 1, stage: VERTEX, binding: 0},{B, 2, VERTEX, 1}]; 
descriptor_set_layout = create_descriptor_set_layout(bindings);
// This layout means that :
// - In VERTEX stage, there will be a descriptor of type A at binding 0.
// - In VERTEX stage still, there will an array descriptor of type B and length 2 at binding 1.
```

## `vk::DescriptorSet`

A set of descriptors, points into a descriptor pool.

Allocation arguments (allocated by groups from a single pool) :
- `vk::DescriptorPool`.
- List of `vk::DescriptorSetLayout`, one per set to allocate.

```rust
/// PSEUDO CODE ///
// `#` = 1 byte.
// descriptor types (size) : A (3 bytes), B (1 byte).

// pool
pool_sizes = [{type: A, count: 2},{B, 5},{A, 1}];
descriptor_pool = create_descriptor_pool(pool_sizes);

descriptor_pool.memory -> ##############

// layout
bindings = [{type: A, count: 1, stage: VERTEX, binding: 0},{B, 2, VERTEX, 1}]; 
descriptor_set_layout = create_descriptor_set_layout(bindings);

// allocation
[set_x, set_y] = allocate_descriptor_sets(descriptor_pool, [descriptor_set_layout, descriptor_set_layout]);
// This will allocate 2 sets with the same layout.

descriptor_pool.memory -> (xxxxx)(yyyyy)####
```

Write arguments :
- layout : `vk::DescriptorType` + array count of element to write to (1 if not an array).
- descriptor location : set + binding + first array index of element to write to (0 if not an array).
- data : `&[vk::DescriptorBufferInfo]` | `&[vk::DescriptorImageInfo]` | `&[vk::BufferView]`.

Copy arguments :
- array count of element to write to (1 if not an array).
- *src* descriptor location : set + binding + first array index of element to copy from (0 if not an array).
- *dst* descriptor location : set + binding + first array index of element to copy to (0 if not an array).

## Descriptor

There's no `Descriptor` structure, but if there was, it could be designed :

```rust
struct Descriptor {
    layout: MemoryLayout,
    access: ShaderAccess,
    data: Data,
}

struct MemoryLayout {
    ty: vk::DescriptorType,
    len: u32, // descriptors are potentially arrays
}

struct ShaderAccess {
    stage: vk::ShaderStageFlags,
    binding: u32,
}

enum Data {
    BufferInfos(Vec<vk::DescriptorBufferInfo>), // `Vec` because descriptors are potentially arrays
    ImageInfos(Vec<vk::DescriptorImageInfo>),
}
```

