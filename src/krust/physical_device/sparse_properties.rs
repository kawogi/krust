
use vk::structs::*;
use vk::consts::*;

#[derive(Debug)]
pub struct PhysicalDeviceSparseProperties {
	/// VK_TRUE if the physical device will access all single-sample 2D sparse resources using the standard block shapes (based on image format), as described in the Standard Sparse Image Block Shapes (Single Sample) table. If this property is not supported the value returned in the imageGranularity member of the VkSparseImageFormatProperties structure for single-sample 2D images is not required to match the standard image block sizes listed in the table.
	pub residency_standard_2d_block_shape: bool, // residencyStandard2DBlockShape: false as VkBool32,
	
	///VK_TRUE if the physical device will access all multisample 2D sparse resources using the standard block shapes (based on image format), as described in the Standard Sparse Image Block Shapes (MSAA) table. If this property is not supported, the value returned in the imageGranularity member of the VkSparseImageFormatProperties structure for multisample 2D images is not required to match the standard image block sizes listed in the table.
	pub residency_standard_2d_multisample_block_shape: bool, // residencyStandard2DMultisampleBlockShape: false as VkBool32,
	
	///VK_TRUE if the physical device will access all 3D sparse resources using the standard block shapes (based on image format), as described in the Standard Sparse Image Block Shapes (Single Sample) table. If this property is not supported, the value returned in the imageGranularity member of the VkSparseImageFormatProperties structure for 3D images is not required to match the standard image block sizes listed in the table.
	pub residency_standard_3d_block_shape: bool, // residencyStandard3DBlockShape: false as VkBool32,
	
	/// VK_TRUE if images with mip level dimensions that are not a multiple of a block size may be placed in the mip tail. If this property is not reported, only mip levels with dimensions smaller than the value of the imageGranularity member of the VkSparseImageFormatProperties structure will be placed in the mip tail. If this property is reported the implementation is allowed to return VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT in the flags member of VkSparseImageFormatProperties, indicating that mip level dimensions that are not a multiple of a block size will be placed in the mip tail.
	pub residency_aligned_mip_size: bool, // residencyAlignedMipSize: false as VkBool32,
	
	/// specifies whether the physical device can consistently access non-resident regions of a resource. If this property is VK_TRUE, access to non-resident regions of resources will be guaranteed to return values as if the resource were populated with 0; writes to non-resident regions will be discarded.
	pub residency_non_resident_strict: bool, // residencyNonResidentStrict: false as VkBool32,
}

impl From<VkPhysicalDeviceSparseProperties> for PhysicalDeviceSparseProperties {
	
	fn from(sparse_properties: VkPhysicalDeviceSparseProperties) -> Self {

		PhysicalDeviceSparseProperties {
			residency_standard_2d_block_shape: sparse_properties.residencyStandard2DBlockShape != VK_FALSE,
			residency_standard_2d_multisample_block_shape: sparse_properties.residencyStandard2DMultisampleBlockShape != VK_FALSE,
			residency_standard_3d_block_shape: sparse_properties.residencyStandard3DBlockShape != VK_FALSE,
			residency_aligned_mip_size: sparse_properties.residencyAlignedMipSize != VK_FALSE,
			residency_non_resident_strict: sparse_properties.residencyNonResidentStrict != VK_FALSE,
		}
	}
	
}

impl PhysicalDeviceSparseProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}residency_standard_2d_block_shape: {:?}", prefix, self.residency_standard_2d_block_shape);
		println!("{}residency_standard_2d_multisample_block_shape: {:?}", prefix, self.residency_standard_2d_multisample_block_shape);
		println!("{}residency_standard_3d_block_shape: {:?}", prefix, self.residency_standard_3d_block_shape);
		println!("{}residency_aligned_mip_size: {:?}", prefix, self.residency_aligned_mip_size);
		println!("{}residency_non_resident_strict: {:?}", prefix, self.residency_non_resident_strict);
	}
}
