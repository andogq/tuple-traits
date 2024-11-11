use std::marker::PhantomData;

use tuple_traits::{Append, Contains};

/// Buffer flags, represented in the type system as zero-sized types (no runtime overhead).
mod flags {
    pub struct Vertex;
    pub struct MapRead;
}

/// Sample buffer object, which is generic over some flags.
struct Buffer<Flags> {
    /// Flags are only referenced as a generic, so can be stored as [`PhantomData`].
    _flags: PhantomData<Flags>,

    /// Sample field.
    data: [u8; 512],
}

impl Buffer<()> {
    /// Create a new instance of a buffer with no flags.
    pub fn new() -> Self {
        Self {
            _flags: PhantomData,
            data: [0; 512],
        }
    }
}

impl<Flags> Buffer<Flags>
where
    // By constraining `Flags` to the `Append` trait, the list of flags can be modified in the
    // return signature.
    Flags: Append,
{
    /// Add the [`flags::Vertex`] flag to the buffer.
    pub fn vertex_flag(self) -> Buffer<Flags::Append<flags::Vertex>> {
        Buffer {
            _flags: PhantomData,
            data: self.data,
        }
    }

    /// Add the [`flags::MapRead`] flag to the buffer.
    pub fn map_read_flag(self) -> Buffer<Flags::Append<flags::MapRead>> {
        Buffer {
            _flags: PhantomData,
            data: self.data,
        }
    }
}

/// A function that requires the buffer has the `Vertex` flag.
fn process_vertex_buffer<Flags, Index>(_buffer: &Buffer<Flags>)
where
    // Constrain the `Flags` type to ensure it contains the `Vertex` flag.
    Flags: Contains<flags::Vertex, Index>,
{
}

/// A function that requires the buffer has both the `Vertex` and `MapRead` flags.
fn read_vertex_buffer<Flags, VertexIndex, MapReadIndex>(_buffer: &Buffer<Flags>)
where
    // Constrain the `Flags` type to ensure it contains the `Vertex` and `MapRead` flags. Note that
    // two separate indexes are required, one for each target.
    Flags: Contains<flags::Vertex, VertexIndex> + Contains<flags::MapRead, MapReadIndex>,
{
}

fn main() {
    // Create a new buffer with no flags.
    let buffer: Buffer<()> = Buffer::new();

    // Add the `Vertex` flag to the buffer.
    let buffer: Buffer<(flags::Vertex,)> = buffer.vertex_flag();

    // Add the `MapRead` flag to the buffer, which will append it to the end of the flag list.
    let buffer: Buffer<(flags::Vertex, flags::MapRead)> = buffer.map_read_flag();

    // Call some methods that require a specific flag configuration.
    process_vertex_buffer(&buffer);
    read_vertex_buffer(&buffer);

    // If uncommented, the following error would be produced:
    //   error[E0277]: the trait bound `(flags::Vertex,): tuple_traits::Contains<flags::MapRead, _>` is not satisfied
    // read_vertex_buffer(&Buffer::new().vertex_flag());
}
