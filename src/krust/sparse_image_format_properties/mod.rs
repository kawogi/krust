
use vk::structs::*;
use vk::flags::*;

#[derive(Debug)]
pub struct SparseImageFormatProperties {

	/// aspectMask is a VkImageAspectFlags specifying which aspects of the image the properties apply to.
	aspect_mask: VkImageAspectFlags,
	
	/// imageGranularity is the width, height, and depth of the block in pixels / compressed blocks.
	image_granularity: VkExtent3D,
	
	/// If VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT is set, the image uses a single mip tail region for all array layers.
	is_single_miptail: bool,
	
	/// If VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT is set, the first mip level that is not an exact multiple of the sparse image block size begins the mip tail region.
	is_aligned_mip_size: bool,
	
	/// If VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT is set, the image uses a non-standard sparse block size, and the imageGranularity values do not match the standard block size for the given pixel format.
	is_nonstandard_block_size: bool,
}

impl<'a> From<&'a VkSparseImageFormatProperties> for SparseImageFormatProperties {
	
	fn from(sparse_image_format_properties: &VkSparseImageFormatProperties) -> Self {
		SparseImageFormatProperties {
			aspect_mask: sparse_image_format_properties.aspectMask,
			image_granularity: sparse_image_format_properties.imageGranularity,
			is_single_miptail: sparse_image_format_properties.flags.contains(VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT),
			is_aligned_mip_size: sparse_image_format_properties.flags.contains(VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT),
			is_nonstandard_block_size: sparse_image_format_properties.flags.contains(VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT),
		}
	}
	
}

impl SparseImageFormatProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	#[allow(dead_code)]
	pub fn dump(&self, prefix: &str) {
		println!("{}aspect_mask: {:?}", prefix, self.aspect_mask);
		println!("{}image_granularity: {:?}", prefix, self.image_granularity);
		println!("{}is_single_miptail: {:?}", prefix, self.is_single_miptail);
		println!("{}is_aligned_mip_size: {:?}", prefix, self.is_aligned_mip_size);
		println!("{}is_nonstandard_block_size: {:?}", prefix, self.is_nonstandard_block_size);
	}
}
