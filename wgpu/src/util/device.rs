/// Describes a [Buffer](crate::Buffer) when allocating.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct BufferInitDescriptor<'a> {
    /// Debug label of a buffer. This will show up in graphics debuggers for easy identification.
    pub label: crate::Label<'a>,
    /// Contents of a buffer on creation.
    pub contents: &'a [u8],
    /// Usages of a buffer. If the buffer is used in any way that isn't specified here, the operation
    /// will panic.
    pub usage: crate::BufferUsages,
}

/// Utility methods not meant to be in the main API.
pub trait DeviceExt {
    /// Creates a [Buffer](crate::Buffer) with data to initialize it.
    fn create_buffer_init(&self, desc: &BufferInitDescriptor) -> crate::Buffer;

    /// Upload an entire texture and its mipmaps from a source buffer.
    ///
    /// Expects all mipmaps to be tightly packed in the data buffer.
    ///
    /// If the texture is a 2DArray texture, uploads each layer in order, expecting
    /// each layer and its mips to be tightly packed.
    ///
    /// Example:
    /// Layer0Mip0 Layer0Mip1 Layer0Mip2 ... Layer1Mip0 Layer1Mip1 Layer1Mip2 ...
    ///
    /// Implicitly adds the `COPY_DST` usage if it is not present in the descriptor,
    /// as it is required to be able to upload the data to the gpu.
    fn create_texture_with_data(
        &self,
        queue: &crate::Queue,
        desc: &crate::TextureDescriptor,
        data: &[u8],
    ) -> crate::Texture;
}

impl DeviceExt for crate::Device {
    fn create_buffer_init(&self, descriptor: &BufferInitDescriptor<'_>) -> crate::Buffer {
        // Skip mapping if the buffer is zero sized
        if descriptor.contents.is_empty() {
            let wgt_descriptor = crate::BufferDescriptor {
                label: descriptor.label,
                size: 0,
                usage: descriptor.usage,
                mapped_at_creation: false,
            };

            self.create_buffer(&wgt_descriptor)
        } else {
            let unpadded_size = descriptor.contents.len() as crate::BufferAddress;
            // Valid vulkan usage is
            // 1. buffer size must be a multiple of COPY_BUFFER_ALIGNMENT.
            // 2. buffer size must be greater than 0.
            // Therefore we round the value up to the nearest multiple, and ensure it's at least COPY_BUFFER_ALIGNMENT.
            let align_mask = crate::COPY_BUFFER_ALIGNMENT - 1;
            let padded_size =
                ((unpadded_size + align_mask) & !align_mask).max(crate::COPY_BUFFER_ALIGNMENT);

            let wgt_descriptor = crate::BufferDescriptor {
                label: descriptor.label,
                size: padded_size,
                usage: descriptor.usage,
                mapped_at_creation: true,
            };

            let buffer = self.create_buffer(&wgt_descriptor);

            buffer.slice(..).get_mapped_range_mut()[..unpadded_size as usize]
                .copy_from_slice(descriptor.contents);
            buffer.unmap();

            buffer
        }
    }

    fn create_texture_with_data(
        &self,
        queue: &crate::Queue,
        desc: &crate::TextureDescriptor,
        data: &[u8],
    ) -> crate::Texture {
        let _texture = self.create_texture(&desc);
        loop {}
   }
}
