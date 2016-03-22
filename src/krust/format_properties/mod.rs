mod image_format_features;
pub use self::image_format_features::ImageFormatFeatures;

mod buffer_format_features;
pub use self::buffer_format_features::BufferFormatFeatures;

use vk::structs::*;

#[derive(Debug)]
pub struct FormatProperties {
	
	/// The linearTilingFeatures members of the ImageFormatFeatures structure
	/// describe what features are supported by VK_IMAGE_TILING_LINEAR images.
	linear_tiling_features: ImageFormatFeatures,
	
	/// The optimalTilingFeatures members of the ImageFormatFeatures structure
	/// describe what features are supported by VK_IMAGE_TILING_OPTIMAL images.
	optimal_tiling_features: ImageFormatFeatures,
	
	/// The bufferFeatures member of the VkFormatProperties structure describes
	/// what features are supported by buffers.
	buffer_features: BufferFormatFeatures,
}


impl<'a> From<&'a VkFormatProperties> for FormatProperties {
	
	fn from(format_properties: &VkFormatProperties) -> Self {
		FormatProperties {
			linear_tiling_features: ImageFormatFeatures::from(format_properties.linearTilingFeatures),
			optimal_tiling_features: ImageFormatFeatures::from(format_properties.optimalTilingFeatures),
			buffer_features: BufferFormatFeatures::from(format_properties.bufferFeatures),
		}
	}
	
}

impl FormatProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		
		let mut indent: String = String::from(prefix);
		indent.push('\t');
		
		println!("{}linear_tiling_features: {{", prefix);
		self.linear_tiling_features.dump(&indent);
		println!("{}}}", prefix);
		
		println!("{}optimal_tiling_features: {{", prefix);
		self.optimal_tiling_features.dump(&indent);
		println!("{}}}", prefix);
		
		println!("{}buffer_features: {{", prefix);
		self.buffer_features.dump(&indent);
		println!("{}}}", prefix);
		
	}
}