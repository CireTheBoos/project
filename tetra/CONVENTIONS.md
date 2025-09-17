# Project Conventions

## Top

```rust
// modules
mod test;
pub mod test_2;

// std imports
use std::collections::HashMap;

// external imports
use winit::event_loop::EventLoop<T>;

// internal imports (might be spaced : crate, super, children)
use crate::context::device::Device;
use super::Entry;
pub use test::Thing;
use test::Test;

// type aliases
type Error = Box<dyn std::error::Error>;

// constants
const LAYER: &CStr = c"my_layer";

/////////////////////////////////////////////////////////////////////////////

                                [BODY]
```

- `super::super` is forbidden in imports (a bit verbose but makes dependencies clearer).

## Headers

```rust
/////////////////////////////////////////////////////////////////////////////
// Global title
/////////////////////////////////////////////////////////////////////////////

pub MyStruct { .. }

/// Global block
impl MyStruct {
    fn test() {
        //----------// local title //----------//

        // local block
        [code block]

        // local block
        [code block]

        //----------// local title //----------//

        [code]
    }
}
```

## Naming

Create & Destroy structures :
- `new(..)` for automatic destruction (only `Drop`).
- `create(..)` + `destroy(..)` for manual destruction.
- Configuration pattern : `MyStruct::create_from_configuration(..)` + `my_struct::configuration::MyStructConfiguration`.

Modules :
- `[function_name]` if contains mainly a function + its sub-functions.
- `[structure_name]` if contains mainly a structure + its implementations + its sub-structures.

## Organization

- Structure implementations are in the same file as the structure they implement.

