use super::VertexIndex;

/////////////////////////////////////////////////////////////////////////////

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Triangle {
    pub a: VertexIndex,
    pub b: VertexIndex,
    pub c: VertexIndex,
}
