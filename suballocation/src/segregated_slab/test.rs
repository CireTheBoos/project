//! Run "cargo test -- --nocapture" to print memory.
//! It's hard to debug because stacks orders are hard to follow (which slab will be the next free slab for example).
//! So println and check manually.

// Import
use super::*;

// External
use mem_utils::RangeOf;
use std::mem::MaybeUninit;

/// Not related to state of the allocator
#[test]
fn allocation_absolute() {
    let allocator_configuration =
        SegregatedSlabConfiguration::pot(RangeOf::new(0, 16), 4, 2).unwrap();
    let mut allocator =
        SegregatedSlabSuballocator::<i32>::new_from_configuration(allocator_configuration).unwrap();

    let size_null = allocator.allocate(0);
    assert!(size_null.is_err());

    let size_too_big = allocator.allocate(5);
    assert!(size_too_big.is_err());
}

/// Not related to state of the allocator
#[test]
fn deallocation_absolute() {
    let allocator_configuration =
        SegregatedSlabConfiguration::pot(RangeOf::new(0, 16), 4, 2).unwrap();
    let mut allocator =
        SegregatedSlabSuballocator::<i32>::new_from_configuration(allocator_configuration).unwrap();

    let offset_out_of_bounds = allocator.deallocate(RangeOf::new(16, 3));
    assert!(offset_out_of_bounds.is_err());
}

/// Follow manually with a pen.
#[test]
fn evolve_as_expected() {
    // memory + allocator
    let mut memory: Box<[MaybeUninit<i32>]> = Box::new_uninit_slice(16);
    let allocator_configuration =
        SegregatedSlabConfiguration::pot(RangeOf::new(0, memory.len()), 4, 2).unwrap();
    let mut allocator =
        SegregatedSlabSuballocator::<i32>::new_from_configuration(allocator_configuration).unwrap();

    unsafe {
        memory
            .iter_mut()
            .for_each(|offset| *offset.assume_init_mut() = 0);
    }

    // PHASE 1 : Setup -> Only allocate
    let mut one = Item::new(0, 3, 1);
    one.range = allocator.allocate(one.range.size).unwrap();
    write_value(&mut memory, one);

    let mut two: Item = Item::new(0, 2, 2);
    two.range = allocator.allocate(two.range.size).unwrap();
    write_value(&mut memory, two);

    let mut three: Item = Item::new(0, 4, 3);
    three.range = allocator.allocate(three.range.size).unwrap();
    write_value(&mut memory, three);

    let mut four: Item = Item::new(0, 1, 4);
    four.range = allocator.allocate(four.range.size).unwrap();
    write_value(&mut memory, four);

    // false negative : might fail if free indices are created and used in a different order.
    assert_eq!(
        unsafe { memory.clone().assume_init() },
        [1, 1, 1, 0, 2, 2, 4, 0, 3, 3, 3, 3, 0, 0, 0, 0]
            .as_slice()
            .into()
    );

    // Test invalid offsets
    assert!(allocator.deallocate(RangeOf::new(10, 3)).is_err());
    assert!(allocator.deallocate(RangeOf::new(12, 3)).is_err());

    // PHASE 2 : Test Deallocation and reuse free slabs.
    let mut five: Item = Item::new(0, 1, 5);
    five.range = allocator.allocate(five.range.size).unwrap();
    write_value(&mut memory, five);

    allocator.deallocate(three.range).unwrap();

    let mut six: Item = Item::new(0, 2, 6);
    six.range = allocator.allocate(six.range.size).unwrap();
    write_value(&mut memory, six);

    let mut seven: Item = Item::new(0, 1, 7);
    seven.range = allocator.allocate(seven.range.size).unwrap();
    write_value(&mut memory, seven);

    // false negative : might fail if free indices are created and used in a different order.
    assert_eq!(
        unsafe { memory.clone().assume_init() },
        [1, 1, 1, 0, 2, 2, 4, 0, 7, 3, 3, 3, 5, 0, 6, 6]
            .as_slice()
            .into()
    );

    allocator.deallocate(two.range).unwrap();

    let mut eight: Item = Item::new(0, 2, 8);
    eight.range = allocator.allocate(eight.range.size).unwrap();
    write_value(&mut memory, eight);

    // false negative : might fail if free indices are created and used in a different order.
    assert_eq!(
        unsafe { memory.clone().assume_init() },
        [1, 1, 1, 0, 8, 8, 4, 0, 7, 3, 3, 3, 5, 0, 6, 6]
            .as_slice()
            .into()
    );

    // Test no more space
    assert!(allocator.allocate(4).is_err());

    // PHASE 3 : Reallocations
    // shrink
    one.range = allocator.reallocate(one.range, 2).unwrap();
    write_value(&mut memory, one);
    // same class
    four.range = allocator.reallocate(four.range, 2).unwrap();
    write_value(&mut memory, four);
    // grow
    eight.range = allocator.reallocate(eight.range, 4).unwrap();
    write_value(&mut memory, eight);

    // false negative : might fail if free indices are created and used in a different order.
    assert_eq!(
        unsafe { memory.clone().assume_init() },
        [8, 8, 8, 8, 8, 8, 4, 4, 7, 3, 1, 1, 5, 0, 6, 6]
            .as_slice()
            .into()
    );
}

#[derive(Debug, Clone, Copy)]
struct Item {
    range: RangeOf<i32>,
    value: i32,
}

impl Item {
    pub fn new(offset: usize, size: usize, value: i32) -> Item {
        Item {
            range: RangeOf::new(offset, size),
            value,
        }
    }
}

fn write_value(memory: &mut [MaybeUninit<i32>], item: Item) {
    for index in item.range.to_std_range() {
        unsafe { *memory[index].assume_init_mut() = item.value };
    }
}
