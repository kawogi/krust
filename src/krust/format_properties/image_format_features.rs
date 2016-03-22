use vk::flags::*;

#[derive(Debug)]
pub struct ImageFormatFeatures {
	
	/// VkImageView can be sampled from. See sampled images section.
	is_sampled_image: bool,
	
	/// VkImageView can be used as storage image. See storage images section.
	is_storage_image: bool,
	
	/// VkImageView can be used as storage image that supports atomic operations.
	is_storage_image_atomic: bool,
	
	/// VkImageView can be used as a framebuffer color attachment and as an input attachment.
	is_color_attachment: bool,
	
	/// VkImageView can be used as a framebuffer color attachment that supports blending and as an input attachment.
	is_color_attachment_blend: bool,
	
	/// VkImageView can be used as a framebuffer depth/stencil attachment and as an input attachment.
	is_depth_stencil_attachment: bool,
	
	/// VkImage can be used as srcImage for the vkCmdBlitImage command.
	is_blit_src: bool,
	
	/// VkImage can be used as dstImage for the vkCmdBlitImage command.
	is_blit_dst: bool,
	
	/// VkImage can be used with a sampler that has either of magFilter or minFilter set to VK_FILTER_LINEAR,
	/// or mipmapMode set to VK_SAMPLER_MIPMAP_MODE_LINEAR. This bit must only be exposed for formats that
	/// also support the VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT.
	/// If the format being queried is a depth/stencil format, this bit only indicates that the depth aspect (not the stencil
	/// aspect) supports linear filtering, and that linear filtering of the depth aspect is supported whether depth compare is
	/// enabled in the sampler or not. If this bit is not present, linear filtering with depth compare disabled is unsupported
	/// and linear filtering with depth compare enabled is supported, but may compute the filtered value in an
	/// implementation-dependent manner which differs from the normal rules of linear filtering. The resulting value must
	/// be in the range [0,1] and should be proportional to, or a weighted average of, the number of comparison passes or
	/// failures.
	is_sampled_image_filter_linear: bool,
}

impl From<VkFormatFeatureFlags> for ImageFormatFeatures {

	fn from(format_feature_flags: VkFormatFeatureFlags) -> Self {
		ImageFormatFeatures {
			is_sampled_image: format_feature_flags.contains(VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT),
			is_storage_image: format_feature_flags.contains(VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT),
			is_storage_image_atomic: format_feature_flags.contains(VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT),
			is_color_attachment: format_feature_flags.contains(VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT),
			is_color_attachment_blend: format_feature_flags.contains(VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT),
			is_depth_stencil_attachment: format_feature_flags.contains(VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT),
			is_blit_src: format_feature_flags.contains(VK_FORMAT_FEATURE_BLIT_SRC_BIT),
			is_blit_dst: format_feature_flags.contains(VK_FORMAT_FEATURE_BLIT_DST_BIT),
			is_sampled_image_filter_linear: format_feature_flags.contains(VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT),
		}
	}
}

impl ImageFormatFeatures {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}is_sampled_image: {:?}", prefix, self.is_sampled_image);
		println!("{}is_storage_image: {:?}", prefix, self.is_storage_image);
		println!("{}is_storage_image_atomic: {:?}", prefix, self.is_storage_image_atomic);
		println!("{}is_color_attachment: {:?}", prefix, self.is_color_attachment);
		println!("{}is_color_attachment_blend: {:?}", prefix, self.is_color_attachment_blend);
		println!("{}is_depth_stencil_attachment: {:?}", prefix, self.is_depth_stencil_attachment);
		println!("{}is_blit_src: {:?}", prefix, self.is_blit_src);
		println!("{}is_blit_dst: {:?}", prefix, self.is_blit_dst);
		println!("{}is_sampled_image_filter_linear: {:?}", prefix, self.is_sampled_image_filter_linear);
	}
}
