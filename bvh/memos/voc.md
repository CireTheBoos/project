# Vocabulary

## BVH

Bounding Volume Hierarchy := Tree (=> hierarchy) of simple volumes (usually AABB) around targeted shapes.

Goal : Reduce spatial test using the granularity of the hierarchy to dismiss a lot of tests.

## TLAS & BLAS

- Top-Level Acceleration Structure (TLAS) := Dynamic acceleration structure for the entire scene that contains model as leaves.

- Bottom-Level Acceleration Structure (BLAS) := Usually fixed-size acceleration structure per model that contains triangles as leaves.
