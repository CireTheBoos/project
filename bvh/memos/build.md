# Build

## Cost function

Surface Area Heuristic := `primitive_cost * primitive_count * aabb_surface_area + aabb_cost`

## Splits

```rust
for axis in axes { // X, Y, Z
    for plane in 1..n { // half, quarter, etc.
        // test differents splits that minimize the cost function
        let split_cost = split_cost(axis, plane);
        if split_cost <= best_cost {
            best_cost = split_cost;
        }
    }
}
```
