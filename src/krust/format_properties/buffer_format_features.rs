use vk::flags::*;

#[derive(Debug)]
pub struct BufferFormatFeatures {
	
	/// Format can be used to create a VkBufferView that can be bound to a VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER descriptor.
	is_uniform_texel_buffer: bool,
	
	/// Format can be used to create a VkBufferView that can be bound to a VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER descriptor.
	is_storage_texel_buffer: bool,
	
	/// Atomic operations are supported on VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER with this format.
	is_storage_texel_buffer_atomic: bool,
	
	/// Format can be used as a vertex attribute format (VkVertexInputAttributeDescription.format).
	is_vertex_buffer: bool,
}


impl From<VkFormatFeatureFlags> for BufferFormatFeatures {
	
	fn from(format_feature_flags: VkFormatFeatureFlags) -> Self {
		BufferFormatFeatures {
			is_uniform_texel_buffer: format_feature_flags.contains(VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT),
			is_storage_texel_buffer: format_feature_flags.contains(VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT),
			is_storage_texel_buffer_atomic: format_feature_flags.contains(VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT),
			is_vertex_buffer: format_feature_flags.contains(VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT),
		}
	}
}


impl BufferFormatFeatures {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}is_uniform_texel_buffer: {:?}", prefix, self.is_uniform_texel_buffer);
		println!("{}is_storage_texel_buffer: {:?}", prefix, self.is_storage_texel_buffer);
		println!("{}is_storage_texel_buffer_atomic: {:?}", prefix, self.is_storage_texel_buffer_atomic);
		println!("{}is_vertex_buffer: {:?}", prefix, self.is_vertex_buffer);
	}
}
