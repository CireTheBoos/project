# Allocate VS Suballocate

Allocate deals with system call to the OS allocator :
- Slow as the OS memory is shared between programs and OS must balance fragmentation and compaction.
- Non contiguous.

Suballocate si the process of allocating a memory region already owned by the program :
- Fast as memory is already owned.
- Contiguous.

They're not comparable but just slightly different problems.

# Algorithms

Allocation algorithms/strategies tries to balance between :
- Minimizing fragmentation (internal one => padding in allocations, external => gaps between allocations).
- Maximizing compaction or speed.

## Segregated slab

The principle is to allocate *slabs* of memory that contains only allocations of a defined size per slab (called the slab's *class*).
Hence the *segregation* : Allocations of different sizes are put into different slabs.

This process allows to never have external fragmentation, even when continuously allocating and deallocating, because each deallocation in a slab can be replaced by an allocation of same size.

The downside is that we're limited to allocating slab classes objects.

To have more flexibility we can pad the allocations and use more general classes, for examples powers of 2.
- This will increase internal fragmentation (because 13 bytes will be padded to 16 bytes allocations).
- If objects are highly dynamics and uniformly distributed in sizes, it will cost an average of 25% internal fragmentation.
- But it allows in-place resizing (going for 13 to 15 won't reallocate since we allocate 16), so less copying around and thus better speed.

## Table / Simple mono-size allocations

You just maintains a table of the free and allocated slots :
- Maximum speed, compaction.
- Minimum fragmentation.
- But only for one allocation size.

## Others

Buddy : You split and merge memory in halves. 8-8-16 can become 4-4-8-16 (split) or 16-16 (merge). I haven't well understood this one other than that to be honest.



