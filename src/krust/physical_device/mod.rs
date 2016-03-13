extern crate libc;

mod features;
mod queue_family_properties;

use self::features::PhysicalDeviceFeatures;
use self::queue_family_properties::QueueFamilyProperties;

use std::ptr;
use std::vec::Vec;

use vk::consts::*;
use vk::enums::*;
use vk::flags::*;
use vk::fns::*;
use vk::structs::*;
use vk::types::*;

use krust::ApiVersion;

#[derive(Debug)]
pub struct PhysicalDevice {
	handle: VkPhysicalDevice,
	//properties: VkPhysicalDeviceProperties,
	
	///  the version of Vulkan supported by the device, encoded as described in the API Version Numbers and Semantics section.
	pub api_version: ApiVersion, // apiVersion: 0u32,
	
	/// the vendor-specified version of the driver.
	pub driver_version: u32, // driverVersion: 0u32,
	
	/// a unique identifier for the vendor (see below) of the physical device.
	pub vendor_id: u32, // vendorID: 0u32,
	
	/// a unique identifier for the physical device among devices available from the vendor.
	pub device_id: u32, // deviceID: 0u32,
	
	/// a VkPhysicalDeviceType specifying the type of device.
	pub device_type: VkPhysicalDeviceType, // deviceType: VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_OTHER,
	
	/// a string containing the name of the device.
	pub device_name: String, // deviceName: [b'\0' as libc::c_uchar; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	
	/// an array of size VK_UUID_SIZE, containing 8-bit values that represent a universally unique identifier for the device.
	pub pipeline_cache_uuid: [u8; VK_UUID_SIZE], // pipelineCacheUUID: [0u8; VK_UUID_SIZE],
	
//limits: VkPhysicalDeviceLimits{
	/// the maximum dimension ( width ) of an image created with an imageType of VK_IMAGE_TYPE_1D.
	pub max_image_dimension_1d: u32, // maxImageDimension1D: 0u32,
	
	///the maximum dimension ( width or height ) of an image created with an imageType of VK_IMAGE_TYPE_2D and without VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT set in flags .
	pub max_image_dimension_2d: u32, // maxImageDimension2D: 0u32,
	
	/// the maximum dimension ( width , height , or depth ) of an image created with an imageType of VK_IMAGE_TYPE_3D.
	pub max_image_dimension_3d: u32, // maxImageDimension3D: 0u32,
	
	/// the maximum dimension ( width or height ) of an image created with an imageType of VK_IMAGE_TYPE_2D and with VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT set in flags 
	pub max_image_dimension_cube: u32, // maxImageDimensionCube: 0u32,
	
	/// the maximum number of layers ( arrayLayers ) for an image.
	pub max_image_array_layers: u32, // maxImageArrayLayers: 0u32,
	
	/// the maximum number of addressable texels for a buffer view created on a buffer which was created with the VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT or VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT set in the usage member of the VkBufferCreateInfo structure.
	pub max_texel_buffer_elements: u32, // maxTexelBufferElements: 0u32,
	
	/// the maximum value that can be specified in the range member of any VkDescriptorBufferInfo structures passed to a call to vkUpdateDescriptorSets for descriptors of type VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER or VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC.
	pub max_uniform_buffer_range: u32, // maxUniformBufferRange: 0u32,
	
	/// the maximum value that can be specified in the range member of any VkDescriptorBufferInfo structures passed to a call to vkUpdateDescriptorSets for descriptors of type VK_DESCRIPTOR_TYPE_STORAGE_BUFFER or VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC.
	pub max_storage_buffer_range: u32, // maxStorageBufferRange: 0u32,
	
	/// the maximum size, in bytes, of the pool of push constant memory. For each of the push constant ranges indicated by the pPushConstantRanges member of the VkPipelineLayoutCreateInfo structure, offset + size must be less than or equal to this limit.
	pub max_push_constants_size: u32, // maxPushConstantsSize: 0u32,
	
	/// the maximum number of device memory allocations, as created by vkAllocateMemory, which can simultaneously exist.
	pub max_memory_allocation_count: u32, // maxMemoryAllocationCount: 0u32,
	
	/// the maximum number of sampler objects, as created by vkCreateSampler, which can simultaneously exist on a device.
	pub max_sample_allocation_count: u32, // maxSamplerAllocationCount: 0u32,
	
	/// the granularity, in bytes, at which buffer or linear image resources, and optimal image resources can be bound to adjacent memory for simultaneous usage. See Buffer-Image Granularity for more details.
	pub buffer_image_granularity: VkDeviceSize, // bufferImageGranularity: 0 as VkDeviceSize,
	
	/// the total amount of address space available, in bytes, for sparse memory resources. This is an upper bound on the sum of the size of all sparse resources, regardless of whether any memory is bound to them.
	pub sparse_address_space_size: VkDeviceSize, // sparseAddressSpaceSize: 0 as VkDeviceSize,
	
	/// the maximum number of descriptor sets that can be simultaneously used by a pipeline. All DescriptorSet decorations in shader modules must have a value less than maxBoundDescriptorSets . See Section 13.2.
	pub max_bound_descriptor_sets: u32, // maxBoundDescriptorSets: 0u32,
	
	/// the maximum number of samplers that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_SAMPLER or VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER count against this limit. A descriptor is accessible to a shader stage when the stageFlags member of the VkDescriptorSetLayoutBinding structure has the bit for that shader stage set. See Section 13.1.2 and Section 13.1.4.
	pub max_per_stage_descriptor_samplers: u32, // maxPerStageDescriptorSamplers: 0u32,
	
	/// the maximum number of uniform buffers that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER or VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC count against this limit. A descriptor is accessible to a shader stage when the stageFlags member of the VkDescriptorSetLayoutBinding structure has the bit for that shader stage set. See Section 13.1.7 and Section 13.1.9.
	pub max_per_stage_descriptor_uniform_buffers: u32, // maxPerStageDescriptorUniformBuffers: 0u32,
	
	/// is the maximum number of storage buffers that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_STORAGE_BUFFER or VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC count against this limit. A descriptor is accessible to a pipeline shader stage when the stageFlags member of the VkDescriptorSetLayoutBinding structure has the bit for that shader stage set. See Section 13.1.8 and Section 13.1.10.
	pub max_per_stage_descriptor_storage_buffers: u32, // maxPerStageDescriptorStorageBuffers: 0u32,
	
	/// the maximum number of sampled images that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, or VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER count against this limit. A descriptor is accessible to a pipeline shader stage when the stageFlags member of the VkDescriptorSetLayoutBinding structure has the bit for that shader stage set. See Section 13.1.4, Section 13.1.3, and Section 13.1.5.
	pub max_per_stage_descriptor_sampled_images: u32, // maxPerStageDescriptorSampledImages: 0u32,
	
	/// the maximum number of storage images that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, or VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER count against this limit. A descriptor is accessible to a pipeline shader stage when the stageFlags member of the VkDescriptorSetLayoutBinding structure has the bit for that shader stage set. See Section 13.1.1, and Section 13.1.6.
	pub max_per_stage_descriptor_storage_images: u32, // maxPerStageDescriptorStorageImages: 0u32,
	
	/// the maximum number of input attachments that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT count against this limit. A descriptor is accessible to a pipeline shader stage when the stageFlags member of the VkDescriptorSetLayoutBinding structure has the bit for that shader stage set. These are only supported for the fragment stage. See Section 13.1.11.
	pub max_per_stage_descriptor_input_attachments: u32, // maxPerStageDescriptorInputAttachments: 0u32,
	
	/// the maximum number of resources that can be accessible to a single shader stage in a pipeline layout. Descriptors with a type of VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER, VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER, VK_DESCRIPTOR_TYPE_STORAGE_BUFFER, VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC, VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC, or VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT count against this limit. For the fragment shader stage the framebuffer color attachments also count against this limit.
	pub max_per_stage_resources: u32, // maxPerStageResources: 0u32,
	
	/// the maximum number of samplers that can be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_SAMPLER or VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER count against this limit. See Section 13.1.2 and Section 13.1.4.
	pub max_descriptor_set_samplers: u32, // maxDescriptorSetSamplers: 0u32,
	
	/// the maximum number of uniform buffers that can be be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER or VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC count against this limit. See Section 13.1.7 and Section 13.1.9.
	pub max_descriptor_set_uniform_buffers: u32, // maxDescriptorSetUniformBuffers: 0u32,
	
	/// the maximum number of dynamic uniform buffers that can be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC count against this limit. See Section 13.1.9.
	pub max_descriptor_set_uniform_buffers_dynamic: u32, // maxDescriptorSetUniformBuffersDynamic: 0u32,
	
	/// the maximum number of storage buffers that can be be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_STORAGE_BUFFER or VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC count against this limit. See Section 13.1.8 and Section 13.1.10.
	pub max_descriptor_set_storage_buffers: u32, // maxDescriptorSetStorageBuffers: 0u32,
	
	/// the maximum number of dynamic storage buffers that can be be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC count against this limit. See Section 13.1.10.
	pub max_descriptor_set_storage_buffers_dynamic: u32, // maxDescriptorSetStorageBuffersDynamic: 0u32,
	
	///the maximum number of sampled images that can be be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER, VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE, or VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER count against this limit. See Section 13.1.4, Section 13.1.3, and Section 13.1.5.
	pub max_descriptor_set_samples_images: u32, // maxDescriptorSetSampledImages: 0u32,
	
	/// the maximum number of storage images that can be be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_STORAGE_IMAGE, or VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER count against this limit. See Section 13.1.1, and Section 13.1.6.
	pub max_descriptor_set_storage_images: u32, // maxDescriptorSetStorageImages: 0u32,
	
	/// the maximum number of input attachments that can be be included in descriptor bindings in a pipeline layout across all pipeline shader stages and descriptor set numbers. Descriptors with a type of VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT count against this limit. See Section 13.1.11.
	pub max_descriptor_set_input_attachments: u32, // maxDescriptorSetInputAttachments: 0u32,
	
	/// the maximum number of vertex input attributes that can be specified for a graphics pipeline. These are described in the array of VkVertexInputAttributeDescription structures that are provided at graphics pipeline creation time via the pVertexAttributeDescriptions member of the VkPipelineVertexInputStateCreateInfo structure. See Section 20.1 and Section 20.2.
	pub max_vertex_input_attributes: u32, // maxVertexInputAttributes: 0u32,
	
	/// the maximum number of vertex buffers that can be specified for providing vertex attributes to a graphics pipeline. These are described in the array of VkVertexInputBindingDescription structures that are provided at graphics pipeline creation time via the pVertexBindingDescriptions member of the VkPipelineVertexInputStateCreateInfo structure. The binding member of VkVertexInputBindingDescription must be less than this limit. See Section 20.2.
	pub max_vertex_input_bindings: u32, // maxVertexInputBindings: 0u32,
	
	/// the maximum vertex input attribute offset that can be added to the vertex input binding stride. The offset member of the VkVertexInputAttributeDescription structure must be less than or equal to this limit. See Section 20.2.
	pub max_vertex_input_attribute_offset: u32, // maxVertexInputAttributeOffset: 0u32,
	
	/// the maximum vertex input binding stride that can be specified in a vertex input binding. The stride member of the VkVertexInputBindingDescription structure must be less than or equal to this limit. See Section 20.2.
	pub max_vertex_input_binding_stride: u32, // maxVertexInputBindingStride: 0u32,
	
	/// the maximum number of components of output variables which can be output by a vertex shader. See Section 8.5
	pub max_vertex_output_components: u32, // maxVertexOutputComponents: 0u32,
	
	/// the maximum tessellation generation level supported by the fixed-function tessellation primitive generator. See Chapter 21.
	pub max_tesselation_generation_level: u32, // maxTessellationGenerationLevel: 0u32,
	
	///the maximum patch size, in vertices, of patches that can be processed by the tessellation control shader and tessellation primitive generator. The value of the patchControlPoints member of the VkPipelineTessellationStateCreateInfo structure specified at pipeline creation time and the value provided in the OutputVertices execution mode of shader modules must be less than or equal to this limit. See Chapter 21.
	pub max_tesselation_patch_size: u32, // maxTessellationPatchSize: 0u32,
	
	/// the maximum number of components of input variables which can be provided as per-vertex inputs to the tessellation control shader stage.
	pub max_tesselation_control_per_vertex_input_components: u32, // maxTessellationControlPerVertexInputComponents: 0u32,
	
	/// the maximum number of components of per-vertex output variables which can be output from the tessellation control shader stage.
	pub max_tesselation_control_per_vertex_output_components: u32, // maxTessellationControlPerVertexOutputComponents: 0u32,
	
	/// the maximum number of components of per-patch output variables which can be output from the tessellation control shader stage.
	pub max_tesselation_control_per_patch_output_components: u32, // maxTessellationControlPerPatchOutputComponents: 0u32,
	
	/// the maximum total number of components of per-vertex and per-patch output variables which can be output from the tessellation control shader stage.
	pub max_tesselation_control_total_output_components: u32, // maxTessellationControlTotalOutputComponents: 0u32,
	
	/// the maximum number of components of input variables which can be provided as per-vertex inputs to the tessellation evaluation shader stage.
	pub max_tesselation_evaluation_input_components: u32, // maxTessellationEvaluationInputComponents: 0u32,
	
	/// the maximum number of components of per-vertex output variables which can be output from the tessellation evaluation shader stage.
	pub max_tesselation_evaluation_output_components: u32, // maxTessellationEvaluationOutputComponents: 0u32,
	
	///the maximum invocation count supported for instanced geometry shaders. The value provided in the Invocations execution mode of shader modules must be less than or equal to this limit. See Chapter 22.
	pub max_geometry_shader_invocations: u32, // maxGeometryShaderInvocations: 0u32,
	
	///the maximum number of components of input variables which can be provided as inputs to the geometry shader stage.
	pub max_geometry_input_components: u32, // maxGeometryInputComponents: 0u32,
	
	/// the maximum number of components of output variables which can be output from the geometry shader stage.
	pub max_geometry_output_components: u32, // maxGeometryOutputComponents: 0u32,
	
	/// the maximum number of vertices which can be emitted by any geometry shader.
	pub max_geometry_output_vertices: u32, // maxGeometryOutputVertices: 0u32,
	
	///the maximum total number of components of output, across all emitted vertices, which can be output from the geometry shader stage.
	pub max_geometry_total_output_components: u32, // maxGeometryTotalOutputComponents: 0u32,
	
	///the maximum number of components of input variables which can be provided as inputs to the fragment shader stage.
	pub max_fragment_input_components: u32, // maxFragmentInputComponents: 0u32,
	
	/// the maximum number of output attachments which can be written to by the fragment shader stage.
	pub max_fragment_output_attachments: u32, // maxFragmentOutputAttachments: 0u32,
	
	/// the maximum number of output attachments which can be written to by the fragment shader stage when blending is enabled and one of the dual source blend modes is in use. See Section 26.1.2 and dualSrcBlend.
	pub max_fragment_dual_src_attachments: u32, // maxFragmentDualSrcAttachments: 0u32,
	
	/// the total number of storage buffers, storage images, and output buffers which can be used in the fragment shader stage.
	pub max_fragment_combined_output_resources: u32, // maxFragmentCombinedOutputResources: 0u32,
	
	///the maximum total storage size, in bytes, of all variables declared with the WorkgroupLocal storage class in shader modules (or with the shared storage qualifier in GLSL) in the compute shader stage.
	pub max_compute_shared_memory_size: u32, // maxComputeSharedMemorySize: 0u32,
	
	/// the maximum number of work groups that can be dispatched by a single dispatch command. These three values represent the maximum number of work groups for the X, Y, and Z dimensions, respectively. The x , y , and z parameters to the vkCmdDispatch command, or members of the VkDispatchIndirectCommand structure must be less than or equal to the corresponding limit. See Chapter 27.
	pub max_compute_work_group_count: [u32; 3], // maxComputeWorkGroupCount: [0u32; 3],
	
	/// the maximum total number of compute shader invocations in a single local work group. The product of the X, Y, and Z sizes as specified by the LocalSize execution mode in shader modules must be less than or equal to this limit.
	pub max_compute_work_group_invocations: u32, // maxComputeWorkGroupInvocations: 0u32,
	
	/// the maximum size of a local compute work group, per dimension. These three values represent the maximum local work group size in the X, Y, and Z dimensions, respectively. The x , y , and z sizes specified by the LocalSize execution mode in shader modules must be less than or equal to the corresponding limit.
	pub max_compute_work_group_size: [u32; 3], // maxComputeWorkGroupSize: [0u32; 3],
	
	/// the number of bits of subpixel precision in framebuffer coordinates x f and y f . See Chapter 24
	pub sub_pixel_precision_bits: u32, // subPixelPrecisionBits: 0u32,
	
	/// the number of bits of precision in the division along an axis of an image used for minification and magnification filters. 2 subTexelPrecisionBits is the actual number of divisions along each axis of the image represented. The filtering hardware will snap to these locations when computing the filtered results.
	pub sub_texel_precision_bits: u32, // subTexelPrecisionBits: 0u32,
	
	/// the number of bits of division that the LOD calculation for mipmap fetching get snapped to when determining the contribution from each miplevel to the mip filtered results. 2 mipmapPrecisionBits is the actual number of divisions.
	///
	/// Note: For example, if this value is 2 bits then when linearly filtering between two levels, each level could: contribute: 0%, 33%, 66%, or 100% (this is just an example and the amount of contribution should be covered by different equations in the spec).
	pub mipmap_precision_bits: u32, // mipmapPrecisionBits: 0u32,
	
	/// the maximum index value that can be used for indexed draw calls when using 32-bit indices. This excludes the primitive restart index value of 0xFFFFFFFF. See fullDrawIndexUint32.
	pub max_draw_indexed_index_value: u32, // maxDrawIndexedIndexValue: 0u32,
	
	///the maximum draw count that is supported for indirect draw calls. See multiDrawIndirect.
	pub max_draw_indirect_count: u32, // maxDrawIndirectCount: 0u32,
	
	/// the maximum absolute sampler level of detail bias. The sum of the mipLodBias member of the VkSamplerCreateInfo structure and the Bias operand of image sampling operations in shader modules (or 0 if no Bias operand is provided to an image sampling operation) are clamped to the range [−maxSamplerLodBias,+maxSamplerLodBias]. See [samplers-mipLodBias].
	pub max_sampler_lod_bias: f32, // maxSamplerLodBias: 0.0f32,
	
	/// the maximum degree of sampler anisotropy. The maximum degree of anisotropic filtering used for an image sampling operation is the minimum of the maxAnisotropy member of the VkSamplerCreateInfo structure and this limit. See [samplers-maxAnisotropy].
	pub max_sampler_anisotropy: f32, // maxSamplerAnisotropy: 0.0f32,
	
	///the maximum number of active viewports. The viewportCount member of the VkPipelineViewportStateCreateInfo structure that is provided at pipeline creation must be less than or equal to this limit.
	pub max_viewports: u32, // maxViewports: 0u32,
	
	/// the maximum viewport dimensions in the X (width) and Y (height) dimensions, respectively. The maximum viewport dimensions must be greater than or equal to the largest image which can be created and used as a framebuffer attachment. See Controlling the Viewport.
	pub max_viewport_dimensions: [u32; 2], // maxViewportDimensions: [0u32; 2],
	
	/// the viewport bounds range [minimum,maximum]. See Controlling the Viewport.
	pub viewport_bounds_range: [f32; 2], // viewportBoundsRange: [0.0f32; 2],
	
	/// the number of bits of subpixel precision for viewport bounds. The subpixel precision that floating-point viewport bounds are interpreted at is given by this limit.
	pub viewport_sub_pixel_bits: u32, // viewportSubPixelBits: 0u32,
	
	/// the minimum required alignment, in bytes, of host visible memory allocations within the host address space. When mapping a memory allocation with vkMapMemory, subtracting offset bytes from the returned pointer will always produce an integer multiple of this limit. See Section 10.2.1.
	pub min_memory_map_alignment: usize, // minMemoryMapAlignment: 0 as usize,
	
	/// the minimum required alignment, in bytes, for the offset member of the VkBufferViewCreateInfo structure for texel buffers. When a buffer view is created for a buffer which was created with VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT or VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT set in the usage member of the VkBufferCreateInfo structure, the offset must be an integer multiple of this limit. This limit is a maximum, not a minimum.
	pub min_texel_buffer_offset_alignment: VkDeviceSize, // minTexelBufferOffsetAlignment: 0 as VkDeviceSize,
	
	///the minimum required alignment, in bytes, for the offset member of the VkDescriptorBufferInfo structure for uniform buffers. When a descriptor of type VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER or VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC is updated, the offset must be an integer multiple of this limit. Similarly, dynamic offsets for uniform buffers must be multiples of this limit. This limit is a maximum, not a minimum.
	pub min_uniform_buffer_offset_alignment: VkDeviceSize, // minUniformBufferOffsetAlignment: 0 as VkDeviceSize,
	
	/// the minimum required alignment, in bytes, for the offset member of the VkDescriptorBufferInfo structure for storage buffers. When a descriptor of type VK_DESCRIPTOR_TYPE_STORAGE_BUFFER or VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC is updated, the offset must be an integer multiple of this limit. Similarly, dynamic offsets for storage buffers must be multiples of this limit. This limit is a maximum, not a minimum.
	pub min_storage_buffer_offset_alignment: VkDeviceSize, // minStorageBufferOffsetAlignment: 0 as VkDeviceSize,
	
	/// the minimum offset value for the ConstOffset image operand of any of the OpImageSample* or OpImageFetch* image instructions.
	pub min_texel_offset: i32, // minTexelOffset: 0i32,
	
	/// the maximum offset value for the ConstOffset image operand of any of the OpImageSample* or OpImageFetch* image instructions.
	pub max_texel_offset: u32, // maxTexelOffset: 0u32,
	
	/// the minimum offset value for the Offset or ConstOffsets image operands of any of the OpImage*Gather image instructions.
	pub min_texel_gather_offset: i32, // minTexelGatherOffset: 0i32,
	
	/// the maximum offset value for the Offset or ConstOffsets image operands of any of the OpImage*Gather image instructions.
	pub max_texel_gather_offset: u32, // maxTexelGatherOffset: 0u32,
	
	/// the minimum negative offset value for the offset operand of the InterpolateAtOffset extended instruction.
	pub min_interpolation_offset: f32, // minInterpolationOffset: 0.0f32,
	
	/// the maximum positive offset value for the offset operand of the InterpolateAtOffset extended instruction.
	pub max_interpolation_offset: f32, // maxInterpolationOffset: 0.0f32,
	
	/// the number of subpixel fractional bits that the x and y offsets to the InterpolateAtOffset extended instruction may be rounded to as fixed-point values.
	pub sub_pixel_interpolation_offset_bits: u32, // subPixelInterpolationOffsetBits: 0u32,
	
	/// the maximum width for a framebuffer. The width member of the VkFramebufferCreateInfo structure must be less than or equal to this limit.
	pub max_framebuffer_width: u32, // maxFramebufferWidth: 0u32,
	
	/// the maximum height for a framebuffer. The height member of the VkFramebufferCreateInfo structure must be less than or equal to this limit.
	pub max_framebuffer_height: u32, // maxFramebufferHeight: 0u32,
	
	/// the maximum layer count for a layered framebuffer. The layers member of the VkFramebufferCreateInfo structure must be less than or equal to this limit.
	pub max_framebuffer_layers: u32, // maxFramebufferLayers: 0u32,
	
	/// a bitmask of VkSampleCountFlagBits bits indicating the supported color sample counts for a framebuffer color attachment.
	pub framebuffer_color_sample_counts: VkSampleCountFlags, // framebufferColorSampleCounts: 0 as VkSampleCountFlags,
	
	/// a bitmask of VkSampleCountFlagBits bits indicating the supported depth sample counts for a framebuffer depth/stencil attachment, when the format includes a depth component.
	pub framebuffer_depth_sample_counts: VkSampleCountFlags, // framebufferDepthSampleCounts: 0 as VkSampleCountFlags,
	
	/// a bitmask of VkSampleCountFlagBits bits indicating the supported stencil sample counts for a framebuffer depth/stencil attachment, when the format includes a stencil component.
	pub framebuffer_stencil_sample_counts: VkSampleCountFlags, // framebufferStencilSampleCounts: 0 as VkSampleCountFlags,
	
	/// a bitmask of VkSampleCountFlagBits bits indicating the supported sample counts for a framebuffer with no attachments.
	pub framebuffer_no_attachments_sample_counts: VkSampleCountFlags, // framebufferNoAttachmentsSampleCounts: 0 as VkSampleCountFlags,
	
	///the maximum number of color attachments that can be used by a subpass in a render pass. The colorAttachmentCount member of the VkSubpassDescription structure must be less than or equal to this limit.
	pub max_color_attachments: u32, // maxColorAttachments: 0u32,
	
	///a bitmask of VkSampleCountFlagBits bits indicating the sample counts supported for all images with a non-integer color format.
	pub sampled_image_color_sample_counts: VkSampleCountFlags, // sampledImageColorSampleCounts: 0 as VkSampleCountFlags,
	
	/// a bitmask of VkSampleCountFlagBits bits indicating the sample counts supported for all images with a integer color format.
	pub sampled_image_integer_sample_counts: VkSampleCountFlags, // sampledImageIntegerSampleCounts: 0 as VkSampleCountFlags,
	
	///a bitmask of VkSampleCountFlagBits bits indicating the sample counts supported for all images with a depth format.
	pub sampled_image_depth_sample_counts: VkSampleCountFlags, // sampledImageDepthSampleCounts: 0 as VkSampleCountFlags,
	
	///a bitmask of VkSampleCountFlagBits bits indicating the sample supported for all images with a stencil format.
	pub sampled_image_stencil_sample_counts: VkSampleCountFlags, // sampledImageStencilSampleCounts: 0 as VkSampleCountFlags,
	
	/// a bitmask of VkSampleCountFlagBits bits indicating the sample counts supported for all images used for storage operations.
	pub storage_image_sample_counts: VkSampleCountFlags, // storageImageSampleCounts: 0 as VkSampleCountFlags,
	
	/// the maximum number of array elements of a variable decorated with the SampleMask built-in decoration.
	pub max_sample_mask_words: u32, // maxSampleMaskWords: 0u32,
	
	/// indicates support for timestamps on all graphics and compute queues. If this limit is set to VK_TRUE, all queues that advertise the VK_QUEUE_GRAPHICS_BIT or VK_QUEUE_COMPUTE_BIT in the VkQueueFamilyProperties:: queueFlags support VkQueueFamilyProperties:: timestampValidBits of at least 36. See Timestamp Queries.
	pub timestamp_compute_and_graphics: bool, // timestampComputeAndGraphics: false as VkBool32,
	
	/// the number of nanoseconds required for a timestamp query to be incremented by 1. See Timestamp Queries.
	pub timestamp_period: f32, // timestampPeriod: 0.0f32,
	
	///the maximum number of clip distances that can be used in a single shader stage. The size of any array declared with the ClipDistance built-in decoration in a shader module must be less than or equal to this limit.
	pub max_clip_distances: u32, // maxClipDistances: 0u32,
	
	///the maximum number of cull distances that can be used in a single shader stage. The size of any array declared with the CullDistance built-in decoration in a shader module must be less than or equal to this limit.
	pub max_cull_distances: u32, // maxCullDistances: 0u32,
	
	/// the maximum combined number of clip and cull distances that can be used in a single shader stage. The sum of the sizes of any pair of arrays declared with the ClipDistance and CullDistance built-in decoration used by a single shader stage in a shader module must be less than or equal to this limit.
	pub max_combined_clip_and_cull_distances: u32, // maxCombinedClipAndCullDistances: 0u32,
	
	/// the number of discrete priorities that can be assigned to a queue based on the value of each member of VkDeviceQueueCreateInfo:: pQueuePriorities . This must be at least 2, and levels must be spread evenly over the range, with at least one level at 1.0, and another at 0.0. See Section 4.3.4.
	pub discrete_queue_priorities: u32, // discreteQueuePriorities: 0u32,
	
	///the range [minimum,maximum] of supported sizes for points. Values written to variables decorated with the PointSize built-in decoration are clamped to this range.
	pub point_size_range: [f32; 2], // pointSizeRange: [0.0f32; 2],
	
	/// the range [minimum,maximum] of supported widths for lines. Values specified by the lineWidth member of the VkPipelineRasterizationStateCreateInfo or the lineWidth parameter to vkCmdSetLineWidth are clamped to this range.
	pub line_width_range: [f32; 2], // lineWidthRange: [0.0f32; 2],
	
	/// the granularity of supported point sizes. Not all point sizes in the range defined by pointSizeRange are supported. This limit specifies the granularity (or increment) between successive supported point sizes.
	pub point_size_granularity: f32, // pointSizeGranularity: 0.0f32,
	
	/// the granularity of supported line widths. Not all line widths in the range defined by lineWidthRange are supported. This limit specifies the granularity (or increment) between successive supported line widths.
	pub line_width_granularity: f32, // lineWidthGranularity: 0.0f32,
	
	///indicates whether lines are rasterized according to the preferred method of rasterization. If set to VK_FALSE, lines may be rasterized under a relaxed set of rules. If set to VK_TRUE, lines are rasterized as per the strict definition. See Basic Line Segment Rasterization.
	pub strict_lines: bool, // strictLines: false as VkBool32,
	
	/// indicates whether rasterization uses the standard sample locations as documented in Multisampling. If set to VK_TRUE, the implementation uses the documented sample locations. If set to VK_FALSE, the implementation may use different sample locations.
	pub standard_sample_locations: bool, // standardSampleLocations: false as VkBool32,
	
	/// the optimal buffer offset alignment in bytes for vkCmdCopyBufferToImage and vkCmdCopyImageToBuffer. The per texel alignment requirements are still enforced, this is just an additional alignment recommendation for optimal performance and power.
	pub optimal_buffer_copy_offset_alignment: VkDeviceSize, // optimalBufferCopyOffsetAlignment: 0 as VkDeviceSize,
	
	/// the optimal buffer row pitch alignment in bytes for vkCmdCopyBufferToImage and vkCmdCopyImageToBuffer. Row pitch is the number of bytes between texels with the same X coordinate in adjacent rows (Y coordinates differ by one). The per texel alignment requirements are still enforced, this is just an additional alignment recommendation for optimal performance and power.
	pub optimal_buffer_copy_row_pitch_alignment: VkDeviceSize, // optimalBufferCopyRowPitchAlignment: 0 as VkDeviceSize,
	
	/// is the size and alignment in bytes that bounds concurrent access to host-mapped device memory
	pub non_coherent_atom_size: VkDeviceSize, // nonCoherentAtomSize: 0 as VkDeviceSize,
	
//	},
//sparseProperties: VkPhysicalDeviceSparseProperties{
	
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
//},	

	pub queue_family_properties: Vec<QueueFamilyProperties>,

	pub features: PhysicalDeviceFeatures,
}


impl PhysicalDevice {
	
	pub fn create(handle: VkPhysicalDevice) -> Self {
		
		let mut properties = VkPhysicalDeviceProperties::default();
		unsafe { vkGetPhysicalDeviceProperties(handle, &mut properties) };

		// TODO there has to be a std-solution somewhere
		let device_name = {
			let &DeviceNameBuffer(device_name_bytes) = &properties.deviceName;
			let device_name_len = device_name_bytes.iter().position(|&c| c == 0).unwrap_or(device_name_bytes.len());
			String::from_utf8_lossy(&device_name_bytes[0..device_name_len]).into_owned()
		};
		
		let mut features = VkPhysicalDeviceFeatures::default(); 
		unsafe { vkGetPhysicalDeviceFeatures(handle, &mut features) };
		
		PhysicalDevice {
			handle: handle,
			//properties: properties,

			api_version: ApiVersion::from_raw(properties.apiVersion),
			driver_version: properties.driverVersion,
			vendor_id: properties.vendorID,
			device_id: properties.deviceID,
			device_type: properties.deviceType,
			device_name: device_name,
			pipeline_cache_uuid: properties.pipelineCacheUUID,

	//limits: VkPhysicalDeviceLimits{
			max_image_dimension_1d: properties.limits.maxImageDimension1D,
			max_image_dimension_2d: properties.limits.maxImageDimension2D,
			max_image_dimension_3d: properties.limits.maxImageDimension3D,
			max_image_dimension_cube: properties.limits.maxImageDimensionCube,
			max_image_array_layers: properties.limits.maxImageArrayLayers,
			max_texel_buffer_elements: properties.limits.maxTexelBufferElements,
			max_uniform_buffer_range: properties.limits.maxUniformBufferRange,
			max_storage_buffer_range: properties.limits.maxStorageBufferRange,
			max_push_constants_size: properties.limits.maxPushConstantsSize,
			max_memory_allocation_count: properties.limits.maxMemoryAllocationCount,
			max_sample_allocation_count: properties.limits.maxSamplerAllocationCount,
			buffer_image_granularity: properties.limits.bufferImageGranularity,
			sparse_address_space_size: properties.limits.sparseAddressSpaceSize,
			max_bound_descriptor_sets: properties.limits.maxBoundDescriptorSets,
			max_per_stage_descriptor_samplers: properties.limits.maxPerStageDescriptorSamplers,
			max_per_stage_descriptor_uniform_buffers: properties.limits.maxPerStageDescriptorUniformBuffers,
			max_per_stage_descriptor_storage_buffers: properties.limits.maxPerStageDescriptorStorageBuffers,
			max_per_stage_descriptor_sampled_images: properties.limits.maxPerStageDescriptorSampledImages,
			max_per_stage_descriptor_storage_images: properties.limits.maxPerStageDescriptorStorageImages,
			max_per_stage_descriptor_input_attachments: properties.limits.maxPerStageDescriptorInputAttachments,
			max_per_stage_resources: properties.limits.maxPerStageResources,
			max_descriptor_set_samplers: properties.limits.maxDescriptorSetSamplers,
			max_descriptor_set_uniform_buffers: properties.limits.maxDescriptorSetUniformBuffers,
			max_descriptor_set_uniform_buffers_dynamic: properties.limits.maxDescriptorSetUniformBuffersDynamic,
			max_descriptor_set_storage_buffers: properties.limits.maxDescriptorSetStorageBuffers,
			max_descriptor_set_storage_buffers_dynamic: properties.limits.maxDescriptorSetStorageBuffersDynamic,
			max_descriptor_set_samples_images: properties.limits.maxDescriptorSetSampledImages,
			max_descriptor_set_storage_images: properties.limits.maxDescriptorSetStorageImages,
			max_descriptor_set_input_attachments: properties.limits.maxDescriptorSetInputAttachments,
			max_vertex_input_attributes: properties.limits.maxVertexInputAttributes,
			max_vertex_input_bindings: properties.limits.maxVertexInputBindings,
			max_vertex_input_attribute_offset: properties.limits.maxVertexInputAttributeOffset,
			max_vertex_input_binding_stride: properties.limits.maxVertexInputBindingStride,
			max_vertex_output_components: properties.limits.maxVertexOutputComponents,
			max_tesselation_generation_level: properties.limits.maxTessellationGenerationLevel,
			max_tesselation_patch_size: properties.limits.maxTessellationPatchSize,
			max_tesselation_control_per_vertex_input_components: properties.limits.maxTessellationControlPerVertexInputComponents,
			max_tesselation_control_per_vertex_output_components: properties.limits.maxTessellationControlPerVertexOutputComponents,
			max_tesselation_control_per_patch_output_components: properties.limits.maxTessellationControlPerPatchOutputComponents,
			max_tesselation_control_total_output_components: properties.limits.maxTessellationControlTotalOutputComponents,
			max_tesselation_evaluation_input_components: properties.limits.maxTessellationEvaluationInputComponents,
			max_tesselation_evaluation_output_components: properties.limits.maxTessellationEvaluationOutputComponents,
			max_geometry_shader_invocations: properties.limits.maxGeometryShaderInvocations,
			max_geometry_input_components: properties.limits.maxGeometryInputComponents,
			max_geometry_output_components: properties.limits.maxGeometryOutputComponents,
			max_geometry_output_vertices: properties.limits.maxGeometryOutputVertices,
			max_geometry_total_output_components: properties.limits.maxGeometryTotalOutputComponents,
			max_fragment_input_components: properties.limits.maxFragmentInputComponents,
			max_fragment_output_attachments: properties.limits.maxFragmentOutputAttachments,
			max_fragment_dual_src_attachments: properties.limits.maxFragmentDualSrcAttachments,
			max_fragment_combined_output_resources: properties.limits.maxFragmentCombinedOutputResources,
			max_compute_shared_memory_size: properties.limits.maxComputeSharedMemorySize,
			max_compute_work_group_count: properties.limits.maxComputeWorkGroupCount,
			max_compute_work_group_invocations: properties.limits.maxComputeWorkGroupInvocations,
			max_compute_work_group_size: properties.limits.maxComputeWorkGroupSize,
			sub_pixel_precision_bits: properties.limits.subPixelPrecisionBits,
			sub_texel_precision_bits: properties.limits.subTexelPrecisionBits,
			mipmap_precision_bits: properties.limits.mipmapPrecisionBits,
			max_draw_indexed_index_value: properties.limits.maxDrawIndexedIndexValue,
			max_draw_indirect_count: properties.limits.maxDrawIndirectCount,
			max_sampler_lod_bias: properties.limits.maxSamplerLodBias,
			max_sampler_anisotropy: properties.limits.maxSamplerAnisotropy,
			max_viewports: properties.limits.maxViewports,
			max_viewport_dimensions: properties.limits.maxViewportDimensions,
			viewport_bounds_range: properties.limits.viewportBoundsRange,
			viewport_sub_pixel_bits: properties.limits.viewportSubPixelBits,
			min_memory_map_alignment: properties.limits.minMemoryMapAlignment,
			min_texel_buffer_offset_alignment: properties.limits.minTexelBufferOffsetAlignment,
			min_uniform_buffer_offset_alignment: properties.limits.minUniformBufferOffsetAlignment,
			min_storage_buffer_offset_alignment: properties.limits.minStorageBufferOffsetAlignment,
			min_texel_offset: properties.limits.minTexelOffset,
			max_texel_offset: properties.limits.maxTexelOffset,
			min_texel_gather_offset: properties.limits.minTexelGatherOffset,
			max_texel_gather_offset: properties.limits.maxTexelGatherOffset,
			min_interpolation_offset: properties.limits.minInterpolationOffset,
			max_interpolation_offset: properties.limits.maxInterpolationOffset,
			sub_pixel_interpolation_offset_bits: properties.limits.subPixelInterpolationOffsetBits,
			max_framebuffer_width: properties.limits.maxFramebufferWidth,
			max_framebuffer_height: properties.limits.maxFramebufferHeight,
			max_framebuffer_layers: properties.limits.maxFramebufferLayers,
			framebuffer_color_sample_counts: properties.limits.framebufferColorSampleCounts,
			framebuffer_depth_sample_counts: properties.limits.framebufferDepthSampleCounts,
			framebuffer_stencil_sample_counts: properties.limits.framebufferStencilSampleCounts,
			framebuffer_no_attachments_sample_counts: properties.limits.framebufferNoAttachmentsSampleCounts,
			max_color_attachments: properties.limits.maxColorAttachments,
			sampled_image_color_sample_counts: properties.limits.sampledImageColorSampleCounts,
			sampled_image_integer_sample_counts: properties.limits.sampledImageIntegerSampleCounts,
			sampled_image_depth_sample_counts: properties.limits.sampledImageDepthSampleCounts,
			sampled_image_stencil_sample_counts: properties.limits.sampledImageStencilSampleCounts,
			storage_image_sample_counts: properties.limits.storageImageSampleCounts,
			max_sample_mask_words: properties.limits.maxSampleMaskWords,
			timestamp_compute_and_graphics: properties.limits.timestampComputeAndGraphics != VK_FALSE,
			timestamp_period: properties.limits.timestampPeriod,
			max_clip_distances: properties.limits.maxClipDistances,
			max_cull_distances: properties.limits.maxCullDistances,
			max_combined_clip_and_cull_distances: properties.limits.maxCombinedClipAndCullDistances,
			discrete_queue_priorities: properties.limits.discreteQueuePriorities,
			point_size_range: properties.limits.pointSizeRange,
			line_width_range: properties.limits.lineWidthRange,
			point_size_granularity: properties.limits.pointSizeGranularity,
			line_width_granularity: properties.limits.lineWidthGranularity,
			strict_lines: properties.limits.strictLines != VK_FALSE,
			standard_sample_locations: properties.limits.standardSampleLocations != VK_FALSE,
			optimal_buffer_copy_offset_alignment: properties.limits.optimalBufferCopyOffsetAlignment,
			optimal_buffer_copy_row_pitch_alignment: properties.limits.optimalBufferCopyRowPitchAlignment,
			non_coherent_atom_size: properties.limits.nonCoherentAtomSize,

	//sparseProperties: VkPhysicalDeviceSparseProperties{
			residency_standard_2d_block_shape: properties.sparseProperties.residencyStandard2DBlockShape != VK_FALSE,
			residency_standard_2d_multisample_block_shape: properties.sparseProperties.residencyStandard2DMultisampleBlockShape != VK_FALSE,
			residency_standard_3d_block_shape: properties.sparseProperties.residencyStandard3DBlockShape != VK_FALSE,
			residency_aligned_mip_size: properties.sparseProperties.residencyAlignedMipSize != VK_FALSE,
			residency_non_resident_strict: properties.sparseProperties.residencyNonResidentStrict != VK_FALSE,
			
			queue_family_properties: Self::get_queue_family_properties(handle),

			features: PhysicalDeviceFeatures::from(features),
		}
	}


    fn get_queue_family_properties(handle: VkPhysicalDevice) -> Vec<QueueFamilyProperties> {
    	
   		let mut queue_family_property_count: u32 = 0;
    	
    	// query number of properties available to this device
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(handle, &mut queue_family_property_count, ptr::null_mut()) };
		
		// create result buffer
		let mut queue_family_properties = Vec::with_capacity(queue_family_property_count as usize);
		queue_family_properties.resize(queue_family_property_count as usize, VkQueueFamilyProperties::default());
		
		// fill the result buffer
		unsafe { vkGetPhysicalDeviceQueueFamilyProperties(handle, &mut queue_family_property_count, queue_family_properties.as_mut_ptr()) };
	    
		queue_family_properties.truncate(queue_family_property_count as usize);
		
		//let mapper = |queue_family_properties| QueueFamilyProperties::from(queue_family_properties);
		queue_family_properties.iter().map(|item| QueueFamilyProperties::from(item)).collect::<Vec<QueueFamilyProperties>>()
    }

	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		
		let mut indent: String = String::from(prefix);
		indent.push('\t');
		
		println!("{}handle: {:?}", prefix, self.handle);

		println!("{}api_version: {:?}", prefix, self.api_version);
		println!("{}driver_version: {:?}", prefix, self.driver_version);
		println!("{}vendor_id: {:?}", prefix, self.vendor_id);
		println!("{}device_id: {:?}", prefix, self.device_id);
		println!("{}device_type: {:?}", prefix, self.device_type);
		println!("{}device_name: {:?}", prefix, self.device_name);
		println!("{}pipeline_cache_uuid: {:?}", prefix, self.pipeline_cache_uuid);

		println!("{}max_image_dimension_1d: {:?}", prefix, self.max_image_dimension_1d);
		println!("{}max_image_dimension_2d: {:?}", prefix, self.max_image_dimension_2d);
		println!("{}max_image_dimension_3d: {:?}", prefix, self.max_image_dimension_3d);
		println!("{}max_image_dimension_cube: {:?}", prefix, self.max_image_dimension_cube);
		println!("{}max_image_array_layers: {:?}", prefix, self.max_image_array_layers);
		println!("{}max_texel_buffer_elements: {:?}", prefix, self.max_texel_buffer_elements);
		println!("{}max_uniform_buffer_range: {:?}", prefix, self.max_uniform_buffer_range);
		println!("{}max_storage_buffer_range: {:?}", prefix, self.max_storage_buffer_range);
		println!("{}max_push_constants_size: {:?}", prefix, self.max_push_constants_size);
		println!("{}max_memory_allocation_count: {:?}", prefix, self.max_memory_allocation_count);
		println!("{}max_sample_allocation_count: {:?}", prefix, self.max_sample_allocation_count);
		println!("{}buffer_image_granularity: {:?}", prefix, self.buffer_image_granularity);
		println!("{}sparse_address_space_size: {:?}", prefix, self.sparse_address_space_size);
		println!("{}max_bound_descriptor_sets: {:?}", prefix, self.max_bound_descriptor_sets);
		println!("{}max_per_stage_descriptor_samplers: {:?}", prefix, self.max_per_stage_descriptor_samplers);
		println!("{}max_per_stage_descriptor_uniform_buffers: {:?}", prefix, self.max_per_stage_descriptor_uniform_buffers);
		println!("{}max_per_stage_descriptor_storage_buffers: {:?}", prefix, self.max_per_stage_descriptor_storage_buffers);
		println!("{}max_per_stage_descriptor_sampled_images: {:?}", prefix, self.max_per_stage_descriptor_sampled_images);
		println!("{}max_per_stage_descriptor_storage_images: {:?}", prefix, self.max_per_stage_descriptor_storage_images);
		println!("{}max_per_stage_descriptor_input_attachments: {:?}", prefix, self.max_per_stage_descriptor_input_attachments);
		println!("{}max_per_stage_resources: {:?}", prefix, self.max_per_stage_resources);
		println!("{}max_descriptor_set_samplers: {:?}", prefix, self.max_descriptor_set_samplers);
		println!("{}max_descriptor_set_uniform_buffers: {:?}", prefix, self.max_descriptor_set_uniform_buffers);
		println!("{}max_descriptor_set_uniform_buffers_dynamic: {:?}", prefix, self.max_descriptor_set_uniform_buffers_dynamic);
		println!("{}max_descriptor_set_storage_buffers: {:?}", prefix, self.max_descriptor_set_storage_buffers);
		println!("{}max_descriptor_set_storage_buffers_dynamic: {:?}", prefix, self.max_descriptor_set_storage_buffers_dynamic);
		println!("{}max_descriptor_set_samples_images: {:?}", prefix, self.max_descriptor_set_samples_images);
		println!("{}max_descriptor_set_storage_images: {:?}", prefix, self.max_descriptor_set_storage_images);
		println!("{}max_descriptor_set_input_attachments: {:?}", prefix, self.max_descriptor_set_input_attachments);
		println!("{}max_vertex_input_attributes: {:?}", prefix, self.max_vertex_input_attributes);
		println!("{}max_vertex_input_bindings: {:?}", prefix, self.max_vertex_input_bindings);
		println!("{}max_vertex_input_attribute_offset: {:?}", prefix, self.max_vertex_input_attribute_offset);
		println!("{}max_vertex_input_binding_stride: {:?}", prefix, self.max_vertex_input_binding_stride);
		println!("{}max_vertex_output_components: {:?}", prefix, self.max_vertex_output_components);
		println!("{}max_tesselation_generation_level: {:?}", prefix, self.max_tesselation_generation_level);
		println!("{}max_tesselation_patch_size: {:?}", prefix, self.max_tesselation_patch_size);
		println!("{}max_tesselation_control_per_vertex_input_components: {:?}", prefix, self.max_tesselation_control_per_vertex_input_components);
		println!("{}max_tesselation_control_per_vertex_output_components: {:?}", prefix, self.max_tesselation_control_per_vertex_output_components);
		println!("{}max_tesselation_control_per_patch_output_components: {:?}", prefix, self.max_tesselation_control_per_patch_output_components);
		println!("{}max_tesselation_control_total_output_components: {:?}", prefix, self.max_tesselation_control_total_output_components);
		println!("{}max_tesselation_evaluation_input_components: {:?}", prefix, self.max_tesselation_evaluation_input_components);
		println!("{}max_tesselation_evaluation_output_components: {:?}", prefix, self.max_tesselation_evaluation_output_components);
		println!("{}max_geometry_shader_invocations: {:?}", prefix, self.max_geometry_shader_invocations);
		println!("{}max_geometry_input_components: {:?}", prefix, self.max_geometry_input_components);
		println!("{}max_geometry_output_components: {:?}", prefix, self.max_geometry_output_components);
		println!("{}max_geometry_output_vertices: {:?}", prefix, self.max_geometry_output_vertices);
		println!("{}max_geometry_total_output_components: {:?}", prefix, self.max_geometry_total_output_components);
		println!("{}max_fragment_input_components: {:?}", prefix, self.max_fragment_input_components);
		println!("{}max_fragment_output_attachments: {:?}", prefix, self.max_fragment_output_attachments);
		println!("{}max_fragment_dual_src_attachments: {:?}", prefix, self.max_fragment_dual_src_attachments);
		println!("{}max_fragment_combined_output_resources: {:?}", prefix, self.max_fragment_combined_output_resources);
		println!("{}max_compute_shared_memory_size: {:?}", prefix, self.max_compute_shared_memory_size);
		println!("{}max_compute_work_group_count: {:?}", prefix, self.max_compute_work_group_count);
		println!("{}max_compute_work_group_invocations: {:?}", prefix, self.max_compute_work_group_invocations);
		println!("{}max_compute_work_group_size: {:?}", prefix, self.max_compute_work_group_size);
		println!("{}sub_pixel_precision_bits: {:?}", prefix, self.sub_pixel_precision_bits);
		println!("{}sub_texel_precision_bits: {:?}", prefix, self.sub_texel_precision_bits);
		println!("{}mipmap_precision_bits: {:?}", prefix, self.mipmap_precision_bits);
		println!("{}max_draw_indexed_index_value: {:?}", prefix, self.max_draw_indexed_index_value);
		println!("{}max_draw_indirect_count: {:?}", prefix, self.max_draw_indirect_count);
		println!("{}max_sampler_lod_bias: {:?}", prefix, self.max_sampler_lod_bias);
		println!("{}max_sampler_anisotropy: {:?}", prefix, self.max_sampler_anisotropy);
		println!("{}max_viewports: {:?}", prefix, self.max_viewports);
		println!("{}max_viewport_dimensions: {:?}", prefix, self.max_viewport_dimensions);
		println!("{}viewport_bounds_range: {:?}", prefix, self.viewport_bounds_range);
		println!("{}viewport_sub_pixel_bits: {:?}", prefix, self.viewport_sub_pixel_bits);
		println!("{}min_memory_map_alignment: {:?}", prefix, self.min_memory_map_alignment);
		println!("{}min_texel_buffer_offset_alignment: {:?}", prefix, self.min_texel_buffer_offset_alignment);
		println!("{}min_uniform_buffer_offset_alignment: {:?}", prefix, self.min_uniform_buffer_offset_alignment);
		println!("{}min_storage_buffer_offset_alignment: {:?}", prefix, self.min_storage_buffer_offset_alignment);
		println!("{}min_texel_offset: {:?}", prefix, self.min_texel_offset);
		println!("{}max_texel_offset: {:?}", prefix, self.max_texel_offset);
		println!("{}min_texel_gather_offset: {:?}", prefix, self.min_texel_gather_offset);
		println!("{}max_texel_gather_offset: {:?}", prefix, self.max_texel_gather_offset);
		println!("{}min_interpolation_offset: {:?}", prefix, self.min_interpolation_offset);
		println!("{}max_interpolation_offset: {:?}", prefix, self.max_interpolation_offset);
		println!("{}sub_pixel_interpolation_offset_bits: {:?}", prefix, self.sub_pixel_interpolation_offset_bits);
		println!("{}max_framebuffer_width: {:?}", prefix, self.max_framebuffer_width);
		println!("{}max_framebuffer_height: {:?}", prefix, self.max_framebuffer_height);
		println!("{}max_framebuffer_layers: {:?}", prefix, self.max_framebuffer_layers);
		println!("{}framebuffer_color_sample_counts: {:?}", prefix, self.framebuffer_color_sample_counts);
		println!("{}framebuffer_depth_sample_counts: {:?}", prefix, self.framebuffer_depth_sample_counts);
		println!("{}framebuffer_stencil_sample_counts: {:?}", prefix, self.framebuffer_stencil_sample_counts);
		println!("{}framebuffer_no_attachments_sample_counts: {:?}", prefix, self.framebuffer_no_attachments_sample_counts);
		println!("{}max_color_attachments: {:?}", prefix, self.max_color_attachments);
		println!("{}sampled_image_color_sample_counts: {:?}", prefix, self.sampled_image_color_sample_counts);
		println!("{}sampled_image_integer_sample_counts: {:?}", prefix, self.sampled_image_integer_sample_counts);
		println!("{}sampled_image_depth_sample_counts: {:?}", prefix, self.sampled_image_depth_sample_counts);
		println!("{}sampled_image_stencil_sample_counts: {:?}", prefix, self.sampled_image_stencil_sample_counts);
		println!("{}storage_image_sample_counts: {:?}", prefix, self.storage_image_sample_counts);
		println!("{}max_sample_mask_words: {:?}", prefix, self.max_sample_mask_words);
		println!("{}timestamp_compute_and_graphics: {:?}", prefix, self.timestamp_compute_and_graphics);
		println!("{}timestamp_period: {:?}", prefix, self.timestamp_period);
		println!("{}max_clip_distances: {:?}", prefix, self.max_clip_distances);
		println!("{}max_cull_distances: {:?}", prefix, self.max_cull_distances);
		println!("{}max_combined_clip_and_cull_distances: {:?}", prefix, self.max_combined_clip_and_cull_distances);
		println!("{}discrete_queue_priorities: {:?}", prefix, self.discrete_queue_priorities);
		println!("{}point_size_range: {:?}", prefix, self.point_size_range);
		println!("{}line_width_range: {:?}", prefix, self.line_width_range);
		println!("{}point_size_granularity: {:?}", prefix, self.point_size_granularity);
		println!("{}line_width_granularity: {:?}", prefix, self.line_width_granularity);
		println!("{}strict_lines: {:?}", prefix, self.strict_lines);
		println!("{}standard_sample_locations: {:?}", prefix, self.standard_sample_locations);
		println!("{}optimal_buffer_copy_offset_alignment: {:?}", prefix, self.optimal_buffer_copy_offset_alignment);
		println!("{}optimal_buffer_copy_row_pitch_alignment: {:?}", prefix, self.optimal_buffer_copy_row_pitch_alignment);
		println!("{}non_coherent_atom_size: {:?}", prefix, self.non_coherent_atom_size);

		println!("{}residency_standard_2d_block_shape: {:?}", prefix, self.residency_standard_2d_block_shape);
		println!("{}residency_standard_2d_multisample_block_shape: {:?}", prefix, self.residency_standard_2d_multisample_block_shape);
		println!("{}residency_standard_3d_block_shape: {:?}", prefix, self.residency_standard_3d_block_shape);
		println!("{}residency_aligned_mip_size: {:?}", prefix, self.residency_aligned_mip_size);
		println!("{}residency_non_resident_strict: {:?}", prefix, self.residency_non_resident_strict);
		
		for properties in &self.queue_family_properties {
			println!("{}queue_family_properties[]: {{", prefix);
			properties.dump(&indent);
			println!("{}}}", prefix);
		}

		println!("{}features: {{", prefix);
		self.features.dump(&indent);
		println!("{}}}", prefix);
	}
}
