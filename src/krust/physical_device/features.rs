
use vk::consts::*;
use vk::structs::*;

#[derive(Debug)]
pub struct PhysicalDeviceFeatures {

	/// robustBufferAccess indicates that out of bounds accesses to buffers via shader operations are well-defined.
	///
	/// – When enabled, out-of-bounds buffer reads will return any of the following values:
	///		- Values from anywhere within the buffer object.
	///		- Zero values, or (0,0,0,x) vectors for vector reads where x is a valid value represented in the type of the vector components and may be any of:
	///			- 0, 1, or the maximum representable positive integer value, for signed or unsigned integer components
	///			- 0.0 or 1.0, for floating-point components
	///	- When enabled, out-of-bounds writes may modify values within the buffer object or be ignored.
	///	– If not enabled, out of bounds accesses may cause undefined behaviour up-to and including process termination.
	///
	pub robust_buffer_access: bool, // robustBufferAccess: VkBool32,
	
	/// fullDrawIndexUint32 indicates the full 32-bit range of indices is supported for indexed draw calls when using a VkIndexType of VK_INDEX_TYPE_UINT32. maxDrawIndexedIndexValue is the maximum index value that may be used (aside from the primitive restart index, which is always 2 32 -1 when the VkIndexType is VK_INDEX_TYPE_UINT32). If this feature is supported, maxDrawIndexedIndexValue must be 2 32 -1; otherwise it must be no smaller than 2 24 -1. See maxDrawIndexedIndexValue.
	pub full_draw_index_uint32: bool, // fullDrawIndexUint32: VkBool32,
	
	/// imageCubeArray indicates whether image views with a VkImageViewType of VK_IMAGE_VIEW_TYPE_CUBE_ARRAY can be created, and that the corresponding SampledCubeArray and ImageCubeArray SPIR-V capabilities can be used in shader code.
	pub image_cube_array: bool, // imageCubeArray: VkBool32,
	
	/// independentBlend indicates whether the VkPipelineColorBlendAttachmentState settings are controlled independently per-attachment. If this feature is not enabled, the VkPipelineColorBlendAttachmentState settings for all color attachments must be identical. Otherwise, a different VkPipelineColorBlendAttachmentState can be provided for each bound color attachment.
	pub independent_blend: bool, // independentBlend: VkBool32,
	
	/// geometryShader indicates whether geometry shaders are supported. If this feature is not enabled, the VK_SHADER_STAGE_GEOMETRY_BIT and VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT enum values must not be used. This also indicates whether shader modules can declare the Geometry capability.
	pub geometry_shader: bool, // geometryShader: VkBool32,
	
	/// tessellationShader indicates whether tessellation control and evaluation shaders are supported. If this feature is not enabled, the VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT, VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT, VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT, VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT, and VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO enum values must not be used. This also indicates whether shader modules can declare the Tessellation capability.
	pub tesselation_shader: bool, // tessellationShader: VkBool32,
	
	/// sampleRateShading indicates whether per-sample shading and multisample interpolation are supported. If this feature is not enabled, the sampleShadingEnable member of the VkPipelineMultisampleStateCreateInfo structure must be set to VK_FALSE and the minSampleShading member is ignored. This also indicates whether shader modules can declare the SampleRateShading capability.
	pub sample_rate_shading: bool, // sampleRateShading: VkBool32,
	
	/// dualSrcBlend indicates whether blend operations which take two sources are supported. If this feature is not enabled, the VK_BLEND_FACTOR_SRC1_COLOR, VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR, VK_BLEND_FACTOR_SRC1_ALPHA, and VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA enum values must not be used as source or destination blending factors. See Section 26.1.2.
	pub dual_src_blend: bool, // dualSrcBlend: VkBool32,
	
	/// logicOp indicates whether logic operations are supported. If this feature is not enabled, the logicOpEnable member of the VkPipelineColorBlendStateCreateInfo structure must be set to VK_FALSE, and the logicOp member is ignored.
	pub logic_op: bool, // logicOp: VkBool32,
	
	/// multiDrawIndirect indicates whether multi-draw indirect is supported. If this feature is not enabled, the drawCount parameter to the vkCmdDrawIndirect and vkCmdDrawIndexedIndirect commands must be 1. The maxDrawIndirectCount member of the VkPhysicalDeviceLimits structure must also be 1 if this feature is not supported. See maxDrawIndirectCount.
	pub multi_draw_indirect: bool, // multiDrawIndirect: VkBool32,
	
	/// drawIndirectFirstInstance indicates whether indirect draw calls support the firstInstance parameter. If this feature is not enabled, the firstInstance member of all VkDrawIndirectCommand and VkDrawIndexedIndirectCommand structures that are provided to the vkCmdDrawIndirect and vkCmdDrawIndexedIndirect commands must be 0.
	pub draw_indirect_first_instance: bool, // drawIndirectFirstInstance: VkBool32,
	
	/// depthClamp indicates whether depth clamping is supported. If this feature is not enabled, the depthClampEnable member of the VkPipelineRasterizationStateCreateInfo structure must be set to VK_FALSE. Otherwise, setting depthClampEnable to VK_TRUE will enable depth clamping.
	pub depth_clamp: bool, // depthClamp: VkBool32,
	
	/// depthBiasClamp indicates whether depth bias clamping is supported. If this feature is not enabled, the depthBiasClamp member of the VkPipelineRasterizationStateCreateInfo structure must be set to 0.0.
	pub depth_bias_clamp: bool, // depthBiasClamp: VkBool32,
	
	/// fillModeNonSolid indicates whether point and wireframe fill modes are supported. If this feature is not enabled, the VK_POLYGON_MODE_POINT and VK_POLYGON_MODE_LINE enum values must not be used.
	pub fill_mode_non_solid: bool, // fillModeNonSolid: VkBool32,
	
	/// depthBounds indicates whether depth bounds tests are supported. If this feature is not enabled, the depthBoundsTestEnable member of the VkPipelineDepthStencilStateCreateInfo structure must be set to VK_FALSE. When depthBoundsTestEnable is set to VK_FALSE, the values of the minDepthBounds and maxDepthBounds members of the VkPipelineDepthStencilStateCreateInfo structure are ignored.
	pub depth_bounds: bool, // depthBounds: VkBool32,
	
	/// wideLines indicates whether lines with width other than 1.0 are supported. If this feature is not enabled, the lineWidth member of the VkPipelineRasterizationStateCreateInfo structure must be set to 1.0. When this feature is supported, the range and granularity of supported line widths are indicated by the lineWidthRange and lineWidthGranularity members of the VkPhysicalDeviceLimits structure, respectively.
	pub wide_lines: bool, // wideLines: VkBool32,
	
	/// largePoints indicates whether points with size greater than 1.0 are supported. If this feature is not enabled, only a point size of 1.0 written by a shader is supported. The range and granularity of supported point sizes are indicated by the pointSizeRange and pointSizeGranularity members of the VkPhysicalDeviceLimits structure, respectively.
	pub large_points: bool, // largePoints: VkBool32,
	
	/// alphaToOne indicates whether the implementation is able to replace the alpha value of the color fragment output from the fragment shader with the maximum representable alpha value for fixed-point colors or 1.0 for floating-point colors. If this feature is not enabled, then the alphaToOneEnable member of the VkPipelineMultisampleStateCreateInfo structure must be set to VK_FALSE. Otherwise setting alphaToOneEnable to VK_TRUE will enable alpha-to-one behaviour.
	pub alpha_to_one: bool, // alphaToOne: VkBool32,
	
	/// multiViewport indicates whether more than one viewport is supported. If this feature is not enabled, the viewportCount and scissorCount members of the VkPipelineViewportStateCreateInfo structure must be set to 1. Similarly, the viewportCount parameter to the vkCmdSetViewport command and the scissorCount parameter to the vkCmdSetScissor command must be 1, and the firstViewport parameter to the vkCmdSetViewport command and the firstScissor parameter to the vkCmdSetScissor command must be 0.
	pub multi_viewport: bool, // multiViewport: VkBool32,
	
	/// samplerAnisotropy indicates whether anisotropic filtering is supported. If this feature is not enabled, the maxAnisotropy member of the VkSamplerCreateInfo structure must be 1.0.
	pub sampler_anisotropy: bool, // samplerAnisotropy: VkBool32,
	
	/// textureCompressionETC2 indicates whether the ETC2 and EAC compressed texture formats are supported. If this feature is not enabled, the following formats must not be used to create images:
	///
	/// – VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK
	/// – VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK
	/// – VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK
	/// – VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK
	/// – VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK
	/// – VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK
	/// – VK_FORMAT_EAC_R11_UNORM_BLOCK
	/// – VK_FORMAT_EAC_R11_SNORM_BLOCK
	/// – VK_FORMAT_EAC_R11G11_UNORM_BLOCK
	/// – VK_FORMAT_EAC_R11G11_SNORM_BLOCK
	/// 
	/// vkGetPhysicalDeviceFormatProperties is used to check for the supported properties of individual formats.
	pub texture_compression_etc2: bool, // textureCompressionETC2: VkBool32,
	
	/// textureCompressionASTC_LDR indicates whether the ASTC LDR compressed texture formats are supported. If this feature is not enabled, the following formats must not be used to create images:
	///
	/// – VK_FORMAT_ASTC_4x4_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_4x4_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_5x4_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_5x4_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_5x5_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_5x5_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_6x5_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_6x5_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_6x6_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_6x6_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_8x5_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_8x5_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_8x6_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_8x6_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_8x8_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_8x8_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_10x5_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_10x5_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_10x6_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_10x6_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_10x8_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_10x8_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_10x10_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_10x10_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_12x10_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_12x10_SRGB_BLOCK
	/// – VK_FORMAT_ASTC_12x12_UNORM_BLOCK
	/// – VK_FORMAT_ASTC_12x12_SRGB_BLOCK
	///
	/// vkGetPhysicalDeviceFormatProperties is used to check for the supported properties of individual formats.
	pub texture_compression_astc_ldr: bool, // textureCompressionASTC_LDR: VkBool32,
	
	/// textureCompressionBC indicates whether the BC compressed texture formats are supported. If this feature is not enabled, the following formats must not be used to create images:
	///
	/// – VK_FORMAT_BC1_RGB_UNORM_BLOCK
	/// – VK_FORMAT_BC1_RGB_SRGB_BLOCK
	/// – VK_FORMAT_BC1_RGBA_UNORM_BLOCK
	/// – VK_FORMAT_BC1_RGBA_SRGB_BLOCK
	/// – VK_FORMAT_BC2_UNORM_BLOCK
	/// – VK_FORMAT_BC2_SRGB_BLOCK
	/// – VK_FORMAT_BC3_UNORM_BLOCK
	/// – VK_FORMAT_BC3_SRGB_BLOCK
	/// – VK_FORMAT_BC4_UNORM_BLOCK
	/// – VK_FORMAT_BC4_SNORM_BLOCK
	/// – VK_FORMAT_BC5_UNORM_BLOCK
	/// – VK_FORMAT_BC5_SNORM_BLOCK
	/// – VK_FORMAT_BC6H_UFLOAT_BLOCK
	/// – VK_FORMAT_BC6H_SFLOAT_BLOCK
	/// – VK_FORMAT_BC7_UNORM_BLOCK
	/// – VK_FORMAT_BC7_SRGB_BLOCK
	///
	/// vkGetPhysicalDeviceFormatProperties is used to check for the supported properties of individual formats.
	pub texture_compression_bc: bool, // textureCompressionBC: VkBool32,
	
	/// occlusionQueryPrecise indicates whether occlusion queries returning actual sample counts are supported. Occlusion queries are created in a VkQueryPool by specifying the queryType of VK_QUERY_TYPE_OCCLUSION in the VkQueryPoolCreateInfo structure which is passed to vkCreateQueryPool. If this feature is enabled, queries of this type can enable VK_QUERY_CONTROL_PRECISE_BIT in the flags parameter to vkCmdBeginQuery. If this feature is not supported, the implementation supports only boolean occlusion queries. When any samples are passed, boolean queries will return a non-zero result value, otherwise a result value of zero is returned. When this feature is enabled and VK_QUERY_CONTROL_PRECISE_BIT is set, occlusion queries will report the actual number of samples passed.
	pub occlusion_query_precise: bool, // occlusionQueryPrecise: VkBool32,
	
	/// pipelineStatisticsQuery indicates whether the pipeline statistics queries are supported. If this feature is not enabled, queries of type VK_QUERY_TYPE_PIPELINE_STATISTICS cannot be created, and none of the VkQueryPipelineStatisticFlagBits bits can be set in the pipelineStatistics member of the VkQueryPoolCreateInfo structure.
	pub pipeline_statistics_query: bool, // pipelineStatisticsQuery: VkBool32,
	
	/// vertexPipelineStoresAndAtomics indicates whether storage buffers and images support stores and atomic operations in the vertex, tessellation, and geometry shader stages. If this feature is not enabled, all storage image, storage texel buffers, and storage buffer variables used by these stages in shader modules must be decorated with the NonWriteable decoration (or the readonly memory qualifier in GLSL).
	pub vertex_pipeline_stores_and_atomics: bool, // vertexPipelineStoresAndAtomics: VkBool32,
	
	/// fragmentStoresAndAtomics indicates whether storage buffers and images support stores and atomic operations in the fragment shader stage. If this feature is not enabled, all storage image, storage texel buffers, and storage buffer variables used by the fragment stage in shader modules must be decorated with the NonWriteable decoration (or the readonly memory qualifier in GLSL).
	pub fragment_stores_and_atomics: bool, // fragmentStoresAndAtomics: VkBool32,
	
	/// shaderTessellationAndGeometryPointSize indicates whether the PointSize built-in decoration is available in the tessellation control, tessellation evaluation, and geometry shader stages. If this feature is not enabled, the PointSize built-in decoration is not available in these shader stages and all points written from a tessellation or geometry shader will have a size of 1.0. This also indicates whether shader modules can declare the TessellationPointSize capability for tessellation control and evaluation shaders, or if the shader modules can declare the GeometryPointSize capability for geometry shaders. An implementation supporting this feature must also support one or both of the tessellationShader or geometryShader features.
	pub shader_tesselation_and_geometry_point_size: bool, // shaderTessellationAndGeometryPointSize: VkBool32,
	
	/// shaderImageGatherExtended indicates whether the extended set of image gather instructions are available in shader code. If this feature is not enabled, the OpImage*Gather instructions do not support the Offset and ConstOffsets operands. This also indicates whether shader modules can declare the ImageGatherExtended capability.
	pub shader_image_gather_extended: bool, // shaderImageGatherExtended: VkBool32,
	
	/// shaderStorageImageExtendedFormats indicates whether the extended storage image formats are available in shader code. If this feature is not enabled, the formats requiring the StorageImageExtendedFormats capability are not supported for storage images. This also indicates whether shader modules can declare the StorageImageExtendedFormats capability.
	pub shader_storage_image_extended_formats: bool, // shaderStorageImageExtendedFormats: VkBool32,
	
	/// shaderStorageImageMultisample indicates whether multisampled storage images are supported. If this feature is not enabled, images that are created with a usage that includes VK_IMAGE_USAGE_STORAGE_BIT must be created with samples equal to VK_SAMPLE_COUNT_1_BIT. This also indicates whether shader modules can declare the StorageImageMultisample capability.
	pub shader_storage_image_multisample: bool, // shaderStorageImageMultisample: VkBool32,
	
	/// shaderStorageImageReadWithoutFormat indicates whether storage images require a format qualifier to be specified when reading from storage images. If this feature is not enabled, the OpImageRead instruction must not have an OpTypeImage of Unknown. This also indicates whether shader modules can declare the StorageImageReadWithoutFormat capability.
	pub shader_storage_image_read_without_format: bool, // shaderStorageImageReadWithoutFormat: VkBool32,
	
	/// shaderStorageImageWriteWithoutFormat indicates whether storage images require a format qualifier to be specified when writing to storage images. If this feature is not enabled, the OpImageWrite instruction must not have an OpTypeImage of Unknown. This also indicates whether shader modules can declare the StorageImageWriteWithoutFormat capability.
	pub shader_storage_image_write_without_format: bool, // shaderStorageImageWriteWithoutFormat: VkBool32,
	
	/// shaderUniformBufferArrayDynamicIndexing indicates whether arrays of uniform buffers can be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER or VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC must be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules can declare the UniformBufferArrayDynamicIndexing capability.
	pub shader_uniform_buffer_array_dynamic_indexing: bool, // shaderUniformBufferArrayDynamicIndexing: VkBool32,
	
	/// shaderSampledImageArrayDynamicIndexing indicates whether arrays of samplers or sampled images can be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of VK_DESCRIPTOR_TYPE_SAMPLER, VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, or VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE must be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules can declare the SampledImageArrayDynamicIndexing capability.
	pub shader_sampled_image_array_dynamic_indexing: bool, // shaderSampledImageArrayDynamicIndexing: VkBool32,
	
	/// shaderStorageBufferArrayDynamicIndexing indicates whether arrays of storage buffers can be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of VK_DESCRIPTOR_TYPE_STORAGE_BUFFER or VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC must be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules can declare the StorageBufferArrayDynamicIndexing capability.
	pub shader_storage_buffer_array_dynamic_indexing: bool, // shaderStorageBufferArrayDynamicIndexing: VkBool32,
	
	/// shaderStorageImageArrayDynamicIndexing indicates whether arrays of storage images can be indexed by dynamically uniform integer expressions in shader code. If this feature is not enabled, resources with a descriptor type of VK_DESCRIPTOR_TYPE_STORAGE_IMAGE must be indexed only by constant integral expressions when aggregated into arrays in shader code. This also indicates whether shader modules can declare the StorageImageArrayDynamicIndexing capability.
	pub shader_storage_image_array_dynamic_indexing: bool, // shaderStorageImageArrayDynamicIndexing: VkBool32,
	
	/// shaderClipDistance indicates whether clip distances are supported in shader code. If this feature is not enabled, the ClipDistance built-in decoration must not be used in shader modules. This also indicates whether shader modules can declare the ClipDistance capability.
	pub shader_clip_distance: bool, // shaderClipDistance: VkBool32,
	
	/// shaderCullDistance indicates whether cull distances are supported in shader code. If this feature is not enabled, the CullDistance built-in decoration must not be used in shader modules. This also indicates whether shader modules can declare the CullDistance capability.
	pub shader_cull_distance: bool, // shaderCullDistance: VkBool32,
	
	/// shaderFloat64 indicates whether 64-bit floats (doubles) are supported in shader code. If this feature is not enabled, 64-bit floating-point types must not be used in shader code. This also indicates whether shader modules can declare the Float64 capability.
	pub shader_float64: bool, // shaderFloat64: VkBool32,
	
	/// shaderInt64 indicates whether 64-bit integers (signed and unsigned) are supported in shader code. If this feature is not enabled, 64-bit integer types must not be used in shader code. This also indicates whether shader modules can declare the Int64 capability.
	pub shader_int64: bool, // shaderInt64: VkBool32,
	
	/// shaderInt16 indicates whether 16-bit integers (signed and unsigned) are supported in shader code. If this feature is not enabled, 16-bit integer types must not be used in shader code. This also indicates whether shader modules can declare the Int16 capability.
	pub shader_int16: bool, // shaderInt16: VkBool32,
	
	/// shaderResourceResidency indicates whether image operations that return resource residency information are supported in shader code. If this feature is not enabled, the OpImageSparse* instructions must not be used in shader code. This also indicates whether shader modules can declare the SparseResidency capability. The feature requires at least one of the sparseResidency * features to be supported.
	pub shader_resource_residency: bool, // shaderResourceResidency: VkBool32,
	
	/// shaderResourceMinLod indicates whether image operations that specify the minimum resource level-of-detail (LOD) are supported in shader code. If this feature is not enabled, the MinLod image operand must not be used in shader code. This also indicates whether shader modules can declare the MinLod capability.
	pub shader_resource_min_lod: bool, // shaderResourceMinLod: VkBool32,
	
	/// sparseBinding indicates whether resource memory can be managed at opaque block level instead of at the object level. If this feature is not enabled, resource memory must be bound only on a per-object basis using the vkBindBufferMemory and vkBindImageMemory commands. In this case, buffers and images must not be created with VK_BUFFER_CREATE_SPARSE_BINDING_BIT and VK_IMAGE_CREATE_SPARSE_BINDING_BIT set in the flags member of the VkBufferCreateInfo and VkImageCreateInfo structures, respectively. Otherwise resource memory can be managed as described in Sparse Resource Features.
	pub sparse_binding: bool, // sparseBinding: VkBool32,
	
	/// sparseResidencyBuffer indicates whether the device can access partially resident buffers. If this feature is not enabled, buffers must not be created with VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkBufferCreateInfo structure.
	pub sparse_residency_buffer: bool, // sparseResidencyBuffer: VkBool32,
	
	pub sparse_residency_image_2d: bool, // sparseResidencyImage2D: VkBool32,
	
	/// sparseResidencyImage3D indicates whether the device can access partially resident 3D images. If this feature is not enabled, images with an imageType of VK_IMAGE_TYPE_3D must not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkImageCreateInfo structure.
	pub sparse_residency_image_3d: bool, // sparseResidencyImage3D: VkBool32,
	
	/// sparseResidency2Samples indicates whether the physical device can access partially resident 2D images with 2 samples per pixel. If this feature is not enabled, images with an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_2_BIT must not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkImageCreateInfo structure.
	pub sparse_residency_2_samples: bool, // sparseResidency2Samples: VkBool32,
	
	/// sparseResidency4Samples indicates whether the physical device can access partially resident 2D images with 4 samples per pixel. If this feature is not enabled, images with an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_4_BIT must not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkImageCreateInfo structure.
	pub sparse_residency_4_samples: bool, // sparseResidency4Samples: VkBool32,
	
	/// sparseResidency8Samples indicates whether the physical device can access partially resident 2D images with 8 samples per pixel. If this feature is not enabled, images with an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_8_BIT must not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkImageCreateInfo structure.
	pub sparse_residency_8_samples: bool, // sparseResidency8Samples: VkBool32,
	
	/// sparseResidency16Samples indicates whether the physical device can access partially resident 2D images with 16 samples per pixel. If this feature is not enabled, images with an imageType of VK_IMAGE_TYPE_2D and samples set to VK_SAMPLE_COUNT_16_BIT must not be created with VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT set in the flags member of the VkImageCreateInfo structure.
	pub sparse_residency_16_samples: bool, // sparseResidency16Samples: VkBool32,
	
	/// sparseResidencyAliased indicates whether the physical device can correctly access data aliased into multiple locations. If this feature is not enabled, the VK_BUFFER_CREATE_SPARSE_ALIASED_BIT and VK_IMAGE_CREATE_SPARSE_ALIASED_BIT enum values must not be used in flags members of the VkBufferCreateInfo and VkImageCreateInfo structures, respectively.
	pub sparse_residency_aliased: bool, // sparseResidencyAliased: VkBool32,
	
	/// variableMultisampleRate indicates whether all pipelines that will be bound to a command buffer during a subpass with no attachments must have the same value for VkPipelineMultisampleStateCreateInfo:: rasterizationSamples . If set to VK_TRUE, the implementation supports variable multisample rates in a subpass with no attachments. If set to VK_FALSE, then all pipelines bound in such a subpass must have the same multisample rate. This has no effect in situations where a subpass uses any attachments.
	pub variable_multisampled_rate: bool, // variableMultisampleRate: VkBool32,
	
	/// inheritedQueries indicates whether a secondary command buffer may be executed while a query is active.
	pub inherited_queries: bool, // inheritedQueries: VkBool32,
	
}

impl From<VkPhysicalDeviceFeatures> for PhysicalDeviceFeatures {
	
	fn from(features: VkPhysicalDeviceFeatures) -> Self {
		PhysicalDeviceFeatures {
			robust_buffer_access: features.robustBufferAccess != VK_FALSE,
			full_draw_index_uint32: features.fullDrawIndexUint32 != VK_FALSE,
			image_cube_array: features.imageCubeArray != VK_FALSE,
			independent_blend: features.independentBlend != VK_FALSE,
			geometry_shader: features.geometryShader != VK_FALSE,
			tesselation_shader: features.tessellationShader != VK_FALSE,
			sample_rate_shading: features.sampleRateShading != VK_FALSE,
			dual_src_blend: features.dualSrcBlend != VK_FALSE,
			logic_op: features.logicOp != VK_FALSE,
			multi_draw_indirect: features.multiDrawIndirect != VK_FALSE,
			draw_indirect_first_instance: features.drawIndirectFirstInstance != VK_FALSE,
			depth_clamp: features.depthClamp != VK_FALSE,
			depth_bias_clamp: features.depthBiasClamp != VK_FALSE,
			fill_mode_non_solid: features.fillModeNonSolid != VK_FALSE,
			depth_bounds: features.depthBounds != VK_FALSE,
			wide_lines: features.wideLines != VK_FALSE,
			large_points: features.largePoints != VK_FALSE,
			alpha_to_one: features.alphaToOne != VK_FALSE,
			multi_viewport: features.multiViewport != VK_FALSE,
			sampler_anisotropy: features.samplerAnisotropy != VK_FALSE,
			texture_compression_etc2: features.textureCompressionETC2 != VK_FALSE,
			texture_compression_astc_ldr: features.textureCompressionASTC_LDR != VK_FALSE,
			texture_compression_bc: features.textureCompressionBC != VK_FALSE,
			occlusion_query_precise: features.occlusionQueryPrecise != VK_FALSE,
			pipeline_statistics_query: features.pipelineStatisticsQuery != VK_FALSE,
			vertex_pipeline_stores_and_atomics: features.vertexPipelineStoresAndAtomics != VK_FALSE,
			fragment_stores_and_atomics: features.fragmentStoresAndAtomics != VK_FALSE,
			shader_tesselation_and_geometry_point_size: features.shaderTessellationAndGeometryPointSize != VK_FALSE,
			shader_image_gather_extended: features.shaderImageGatherExtended != VK_FALSE,
			shader_storage_image_extended_formats: features.shaderStorageImageExtendedFormats != VK_FALSE,
			shader_storage_image_multisample: features.shaderStorageImageMultisample != VK_FALSE,
			shader_storage_image_read_without_format: features.shaderStorageImageReadWithoutFormat != VK_FALSE,
			shader_storage_image_write_without_format: features.shaderStorageImageWriteWithoutFormat != VK_FALSE,
			shader_uniform_buffer_array_dynamic_indexing: features.shaderUniformBufferArrayDynamicIndexing != VK_FALSE,
			shader_sampled_image_array_dynamic_indexing: features.shaderSampledImageArrayDynamicIndexing != VK_FALSE,
			shader_storage_buffer_array_dynamic_indexing: features.shaderStorageBufferArrayDynamicIndexing != VK_FALSE,
			shader_storage_image_array_dynamic_indexing: features.shaderStorageImageArrayDynamicIndexing != VK_FALSE,
			shader_clip_distance: features.shaderClipDistance != VK_FALSE,
			shader_cull_distance: features.shaderCullDistance != VK_FALSE,
			shader_float64: features.shaderFloat64 != VK_FALSE,
			shader_int64: features.shaderInt64 != VK_FALSE,
			shader_int16: features.shaderInt16 != VK_FALSE,
			shader_resource_residency: features.shaderResourceResidency != VK_FALSE,
			shader_resource_min_lod: features.shaderResourceMinLod != VK_FALSE,
			sparse_binding: features.sparseBinding != VK_FALSE,
			sparse_residency_buffer: features.sparseResidencyBuffer != VK_FALSE,
			sparse_residency_image_2d: features.sparseResidencyImage2D != VK_FALSE,
			sparse_residency_image_3d: features.sparseResidencyImage3D != VK_FALSE,
			sparse_residency_2_samples: features.sparseResidency2Samples != VK_FALSE,
			sparse_residency_4_samples: features.sparseResidency4Samples != VK_FALSE,
			sparse_residency_8_samples: features.sparseResidency8Samples != VK_FALSE,
			sparse_residency_16_samples: features.sparseResidency16Samples != VK_FALSE,
			sparse_residency_aliased: features.sparseResidencyAliased != VK_FALSE,
			variable_multisampled_rate: features.variableMultisampleRate != VK_FALSE,
			inherited_queries: features.inheritedQueries != VK_FALSE,
		}
	}
	
}


impl PhysicalDeviceFeatures {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}robust_buffer_access: {:?}", prefix, self.robust_buffer_access);
		println!("{}full_draw_index_uint32: {:?}", prefix, self.full_draw_index_uint32);
		println!("{}image_cube_array: {:?}", prefix, self.image_cube_array);
		println!("{}independent_blend: {:?}", prefix, self.independent_blend);
		println!("{}geometry_shader: {:?}", prefix, self.geometry_shader);
		println!("{}tesselation_shader: {:?}", prefix, self.tesselation_shader);
		println!("{}sample_rate_shading: {:?}", prefix, self.sample_rate_shading);
		println!("{}dual_src_blend: {:?}", prefix, self.dual_src_blend);
		println!("{}logic_op: {:?}", prefix, self.logic_op);
		println!("{}multi_draw_indirect: {:?}", prefix, self.multi_draw_indirect);
		println!("{}draw_indirect_first_instance: {:?}", prefix, self.draw_indirect_first_instance);
		println!("{}depth_clamp: {:?}", prefix, self.depth_clamp);
		println!("{}depth_bias_clamp: {:?}", prefix, self.depth_bias_clamp);
		println!("{}fill_mode_non_solid: {:?}", prefix, self.fill_mode_non_solid);
		println!("{}depth_bounds: {:?}", prefix, self.depth_bounds);
		println!("{}wide_lines: {:?}", prefix, self.wide_lines);
		println!("{}large_points: {:?}", prefix, self.large_points);
		println!("{}alpha_to_one: {:?}", prefix, self.alpha_to_one);
		println!("{}multi_viewport: {:?}", prefix, self.multi_viewport);
		println!("{}sampler_anisotropy: {:?}", prefix, self.sampler_anisotropy);
		println!("{}texture_compression_etc2: {:?}", prefix, self.texture_compression_etc2);
		println!("{}texture_compression_astc_ldr: {:?}", prefix, self.texture_compression_astc_ldr);
		println!("{}texture_compression_bc: {:?}", prefix, self.texture_compression_bc);
		println!("{}occlusion_query_precise: {:?}", prefix, self.occlusion_query_precise);
		println!("{}pipeline_statistics_query: {:?}", prefix, self.pipeline_statistics_query);
		println!("{}vertex_pipeline_stores_and_atomics: {:?}", prefix, self.vertex_pipeline_stores_and_atomics);
		println!("{}fragment_stores_and_atomics: {:?}", prefix, self.fragment_stores_and_atomics);
		println!("{}shader_tesselation_and_geometry_point_size: {:?}", prefix, self.shader_tesselation_and_geometry_point_size);
		println!("{}shader_image_gather_extended: {:?}", prefix, self.shader_image_gather_extended);
		println!("{}shader_storage_image_extended_formats: {:?}", prefix, self.shader_storage_image_extended_formats);
		println!("{}shader_storage_image_multisample: {:?}", prefix, self.shader_storage_image_multisample);
		println!("{}shader_storage_image_read_without_format: {:?}", prefix, self.shader_storage_image_read_without_format);
		println!("{}shader_storage_image_write_without_format: {:?}", prefix, self.shader_storage_image_write_without_format);
		println!("{}shader_uniform_buffer_array_dynamic_indexing: {:?}", prefix, self.shader_uniform_buffer_array_dynamic_indexing);
		println!("{}shader_sampled_image_array_dynamic_indexing: {:?}", prefix, self.shader_sampled_image_array_dynamic_indexing);
		println!("{}shader_storage_buffer_array_dynamic_indexing: {:?}", prefix, self.shader_storage_buffer_array_dynamic_indexing);
		println!("{}shader_storage_image_array_dynamic_indexing: {:?}", prefix, self.shader_storage_image_array_dynamic_indexing);
		println!("{}shader_clip_distance: {:?}", prefix, self.shader_clip_distance);
		println!("{}shader_cull_distance: {:?}", prefix, self.shader_cull_distance);
		println!("{}shader_float64: {:?}", prefix, self.shader_float64);
		println!("{}shader_int64: {:?}", prefix, self.shader_int64);
		println!("{}shader_int16: {:?}", prefix, self.shader_int16);
		println!("{}shader_resource_residency: {:?}", prefix, self.shader_resource_residency);
		println!("{}shader_resource_min_lod: {:?}", prefix, self.shader_resource_min_lod);
		println!("{}sparse_binding: {:?}", prefix, self.sparse_binding);
		println!("{}sparse_residency_buffer: {:?}", prefix, self.sparse_residency_buffer);
		println!("{}sparse_residency_image_2d: {:?}", prefix, self.sparse_residency_image_2d);
		println!("{}sparse_residency_image_3d: {:?}", prefix, self.sparse_residency_image_3d);
		println!("{}sparse_residency_2_samples: {:?}", prefix, self.sparse_residency_2_samples);
		println!("{}sparse_residency_4_samples: {:?}", prefix, self.sparse_residency_4_samples);
		println!("{}sparse_residency_8_samples: {:?}", prefix, self.sparse_residency_8_samples);
		println!("{}sparse_residency_16_samples: {:?}", prefix, self.sparse_residency_16_samples);
		println!("{}sparse_residency_aliased: {:?}", prefix, self.sparse_residency_aliased);
		println!("{}variable_multisampled_rate: {:?}", prefix, self.variable_multisampled_rate);
		println!("{}inherited_queries: {:?}", prefix, self.inherited_queries);
	}
}
