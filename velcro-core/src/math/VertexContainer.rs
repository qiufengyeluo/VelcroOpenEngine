#![warn(clippy::pedantic)]
#![allow(clippy::many_single_char_names)]

type IndexFunction  = |i32|;
pub struct VertexContainer{
     _vertices:Vec<Vertex>, ///< Vertices (positions).

     m_addCallback = nullptr; ///< Callback for when a vertex is added.
    IndexFunction m_removeCallback = nullptr; ///< Callback for when a vertex is removed.
    IndexFunction m_updateCallback = nullptr; ///< Callback for when a vertex is updated/modified.
    VoidFunction m_setCallback = nullptr; ///< Callback for when a all vertices are set.
    VoidFunction m_clearCallback = nullptr; ///< Callback for when a all vertices are cleared.
}