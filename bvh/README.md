- [BVH](#bvh)
   * [I. Example](#i-example)
   * [II. Building](#ii-building)
   * [III. Traversal](#iii-traversal)
      + [`Node` structure](#node-structure)
   * [V. Performance](#v-performance)
      + [Cache efficiency](#cache-efficiency)
      + [Instruction count](#instruction-count)

# BVH

Heavely inspired by blog series ["how to build a bvh"](https://jacco.ompf2.com/2022/04/13/how-to-build-a-bvh-part-1-basics/) by jbikker.

BVH :
- *Binary* : 0 or 2 children per node.
- *SAH* : Tries to minimize the Surface Area Heuristic (SAH) cost function.
- *Indirection* : Add an indirection step between leaves and primitives (leaves -> indirection -> primitives).
- No support for primitive indexing.

Crate :
- *No heap allocations* : All functions require their needed memory as arguments. It's up to caller to manage memory.
- *Trait arguments* : Use of trait argument (`impl Trait`) whenever possible to stay flexible (does not impact performance).
- *use `glam`'s `Vec3`* (crate) : Closer to GLSL, simpler, better graphics functions than `nalgebra`'s `Vector3<f32>`.

## I. Example

```rust
/////////////////////////////////////////////////////////////////////////////
// Primitive
/////////////////////////////////////////////////////////////////////////////

struct Triangle {
    a,b,c : Vec3
}

// Triangle's AABB = min and max values of its vertices
impl bvh::AsAabb for Triangle {
    fn aabb_min(&self) -> Vec3 {
        self.a.min(self.b.min(self.c))
    }

    fn aabb_max(&self) -> Vec3 {
        self.a.max(self.b.max(self.c))
    }
}

/////////////////////////////////////////////////////////////////////////////
// Example
/////////////////////////////////////////////////////////////////////////////

pub fn simple_test() {
    //---// data setup //---//

    // primitives = 3 simple triangles (top-right quadrant, snapped to grid)
    let primitives: Vec<Triangle> = vec![
        Triangle::new(
            Vec3::new(1., 1., 0.),
            Vec3::new(1., 4., 0.),
            Vec3::new(3., 1., 0.),
        ),
        Triangle::new(
            Vec3::new(5., 1., 0.),
            Vec3::new(7., 1., 0.),
            Vec3::new(7., 3., 0.),
        ),
        Triangle::new(
            Vec3::new(2., 3., 0.),
            Vec3::new(4., 5., 0.),
            Vec3::new(4., 3., 0.),
        ),
    ];

    // nodes (uninit)
    let mut nodes: Box<[MaybeUninit<bvh::Node>]> = Box::new_uninit_slice(primitives.len() * 2);

    // indirection (uninit)
    let mut indirection: Box<[MaybeUninit<u32>]> = Box::new_uninit_slice(primitives.len());

    //---// build //---//

    // nodes & indirection returned references are initialized !
    let (init_nodes: &[bvh::Node], init_indirection: &[u32]) = bvh::build(&mut nodes, &mut indirection, &primitives).unwrap();

    //---// print //---//

    bvh::print(init_nodes, init_indirection);
}
```

`bvh::print(..)` output :

```bash
node 0 : AABB = [1, 1, 0] to [7, 5, 0]
 | node 2 : AABB = [5, 1, 0] to [7, 3, 0] - primitives = [1]
 | node 3 : AABB = [1, 1, 0] to [4, 5, 0]
 |  | node 4 : AABB = [1, 1, 0] to [3, 4, 0] - primitives = [0]
 |  | node 5 : AABB = [2, 3, 0] to [4, 5, 0] - primitives = [2]
```

There's no "node 1" as it's used for alignment (see performance).

## II. Building

Top-down building using SAH cost function.

## III. Traversal

Depth-first stack traversal.

### `Node` structure

```rust
//------// pseudo-code //------//

/// 32 bits :
/// - aabb_min = 0..12 bits
/// - index = 12..16 bits
/// - aabb_max = 16..28 bits
/// - indirection_count = 28..32 bits
struct Node {
    aabb_min: Vec3,
    index: u32,
    aabb_max: Vec3,
    primitive_count: u32,
}
```

When node is internal :
- `primitive_count` = 0.
- `index` is the index of its left child (its right child is just after).
So `left_child = nodes[self.index]` and `right_child = nodes[self.index + 1]`.

When node is leaf :
- `primitive_count` > 0.
- `index` is the start of its indirection slice.
So `first_primitive = primitives[indirection[self.index]]`, `second_primitive = primitives[indirection[self.index + 1]]`, Etc.

## V. Performance

### Cache efficiency

1. Siblings

Definition : **Sibling nodes** = Nodes with the the same parent = left and right in a binary tree.

We want siblings to be on the same cacheline as *they're tested together during traversal* (spatial locality).

It is the case if all these conditions are met :
- (cacheline >= 64 bytes)
- `nodes` is 64-bytes aligned.
- Siblings are continuous in memory.
- An alignment node is pushed after root.

I won't prove it here as it is fairly simple to see yet heavy to prove rigorously.

You can see this is the case in the example (although we're not sure that `nodes` is 64-bytes aligned).

### Instruction count

1. Precompute AABB values when building (need space)

It might improve performance to precompute and store primitives AABB values.

In the example above, we give directly the primitives and aabb values (`aabb_min/max()`) are computed on the fly each time from the vertices.

`center()` may not benefit much from precomputing as it's the mean of `aabb_min()` and `aabb_max()` : Addition + multiplication (as it is a constant) + aabb values are most likely already cached by the moment center is needed.

But for example, such precomputation for spheres (where min and max are derived from origin + radius, so 16 bytes per sphere) performs worse (roughly 4.5 ms to 5.2 ms).

2. Use custom SIMD (not currently supported)
