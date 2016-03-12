
pub type VkFlags = u32;

pub type VkInstanceCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkFormatFeatureFlagBits {
	VK_FORMAT_FEATURE_SAMPLED_IMAGE_BIT = 0x00000001,
	VK_FORMAT_FEATURE_STORAGE_IMAGE_BIT = 0x00000002,
	VK_FORMAT_FEATURE_STORAGE_IMAGE_ATOMIC_BIT = 0x00000004,
	VK_FORMAT_FEATURE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000008,
	VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_BIT = 0x00000010,
	VK_FORMAT_FEATURE_STORAGE_TEXEL_BUFFER_ATOMIC_BIT = 0x00000020,
	VK_FORMAT_FEATURE_VERTEX_BUFFER_BIT = 0x00000040,
	VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BIT = 0x00000080,
	VK_FORMAT_FEATURE_COLOR_ATTACHMENT_BLEND_BIT = 0x00000100,
	VK_FORMAT_FEATURE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000200,
	VK_FORMAT_FEATURE_BLIT_SRC_BIT = 0x00000400,
	VK_FORMAT_FEATURE_BLIT_DST_BIT = 0x00000800,
	VK_FORMAT_FEATURE_SAMPLED_IMAGE_FILTER_LINEAR_BIT = 0x00001000,
}
pub type VkFormatFeatureFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageUsageFlagBits {
	VK_IMAGE_USAGE_TRANSFER_SRC_BIT = 0x00000001,
	VK_IMAGE_USAGE_TRANSFER_DST_BIT = 0x00000002,
	VK_IMAGE_USAGE_SAMPLED_BIT = 0x00000004,
	VK_IMAGE_USAGE_STORAGE_BIT = 0x00000008,
	VK_IMAGE_USAGE_COLOR_ATTACHMENT_BIT = 0x00000010,
	VK_IMAGE_USAGE_DEPTH_STENCIL_ATTACHMENT_BIT = 0x00000020,
	VK_IMAGE_USAGE_TRANSIENT_ATTACHMENT_BIT = 0x00000040,
	VK_IMAGE_USAGE_INPUT_ATTACHMENT_BIT = 0x00000080,
}
pub type VkImageUsageFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageCreateFlagBits {
	VK_IMAGE_CREATE_SPARSE_BINDING_BIT = 0x00000001,
	VK_IMAGE_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
	VK_IMAGE_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
	VK_IMAGE_CREATE_MUTABLE_FORMAT_BIT = 0x00000008,
	VK_IMAGE_CREATE_CUBE_COMPATIBLE_BIT = 0x00000010,
}
pub type VkImageCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSampleCountFlagBits {
	VK_SAMPLE_COUNT_1_BIT = 0x00000001,
	VK_SAMPLE_COUNT_2_BIT = 0x00000002,
	VK_SAMPLE_COUNT_4_BIT = 0x00000004,
	VK_SAMPLE_COUNT_8_BIT = 0x00000008,
	VK_SAMPLE_COUNT_16_BIT = 0x00000010,
	VK_SAMPLE_COUNT_32_BIT = 0x00000020,
	VK_SAMPLE_COUNT_64_BIT = 0x00000040,
}
pub type VkSampleCountFlags = VkFlags;

bitflags! {
	#[repr(C)]
	#[derive(Default)]
	pub flags VkQueueFlags: u32 { // VkFlags
		const VK_QUEUE_GRAPHICS_BIT = 0x00000001,
		const VK_QUEUE_COMPUTE_BIT = 0x00000002,
		const VK_QUEUE_TRANSFER_BIT = 0x00000004,
		const VK_QUEUE_SPARSE_BINDING_BIT = 0x00000008,
	}
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkMemoryPropertyFlagBits {
	VK_MEMORY_PROPERTY_DEVICE_LOCAL_BIT = 0x00000001,
	VK_MEMORY_PROPERTY_HOST_VISIBLE_BIT = 0x00000002,
	VK_MEMORY_PROPERTY_HOST_COHERENT_BIT = 0x00000004,
	VK_MEMORY_PROPERTY_HOST_CACHED_BIT = 0x00000008,
	VK_MEMORY_PROPERTY_LAZILY_ALLOCATED_BIT = 0x00000010,
}
pub type VkMemoryPropertyFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkMemoryHeapFlagBits {
	DUMMY = 0,
	VK_MEMORY_HEAP_DEVICE_LOCAL_BIT = 0x00000001,
}
pub type VkMemoryHeapFlags = VkFlags;
pub type VkDeviceCreateFlags = VkFlags;
pub type VkDeviceQueueCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPipelineStageFlagBits {
	VK_PIPELINE_STAGE_TOP_OF_PIPE_BIT = 0x00000001,
	VK_PIPELINE_STAGE_DRAW_INDIRECT_BIT = 0x00000002,
	VK_PIPELINE_STAGE_VERTEX_INPUT_BIT = 0x00000004,
	VK_PIPELINE_STAGE_VERTEX_SHADER_BIT = 0x00000008,
	VK_PIPELINE_STAGE_TESSELLATION_CONTROL_SHADER_BIT = 0x00000010,
	VK_PIPELINE_STAGE_TESSELLATION_EVALUATION_SHADER_BIT = 0x00000020,
	VK_PIPELINE_STAGE_GEOMETRY_SHADER_BIT = 0x00000040,
	VK_PIPELINE_STAGE_FRAGMENT_SHADER_BIT = 0x00000080,
	VK_PIPELINE_STAGE_EARLY_FRAGMENT_TESTS_BIT = 0x00000100,
	VK_PIPELINE_STAGE_LATE_FRAGMENT_TESTS_BIT = 0x00000200,
	VK_PIPELINE_STAGE_COLOR_ATTACHMENT_OUTPUT_BIT = 0x00000400,
	VK_PIPELINE_STAGE_COMPUTE_SHADER_BIT = 0x00000800,
	VK_PIPELINE_STAGE_TRANSFER_BIT = 0x00001000,
	VK_PIPELINE_STAGE_BOTTOM_OF_PIPE_BIT = 0x00002000,
	VK_PIPELINE_STAGE_HOST_BIT = 0x00004000,
	VK_PIPELINE_STAGE_ALL_GRAPHICS_BIT = 0x00008000,
	VK_PIPELINE_STAGE_ALL_COMMANDS_BIT = 0x00010000,
}
pub type VkPipelineStageFlags = VkFlags;
pub type VkMemoryMapFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageAspectFlagBits {
	VK_IMAGE_ASPECT_COLOR_BIT = 0x00000001,
	VK_IMAGE_ASPECT_DEPTH_BIT = 0x00000002,
	VK_IMAGE_ASPECT_STENCIL_BIT = 0x00000004,
	VK_IMAGE_ASPECT_METADATA_BIT = 0x00000008,
}
pub type VkImageAspectFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSparseImageFormatFlagBits {
	VK_SPARSE_IMAGE_FORMAT_SINGLE_MIPTAIL_BIT = 0x00000001,
	VK_SPARSE_IMAGE_FORMAT_ALIGNED_MIP_SIZE_BIT = 0x00000002,
	VK_SPARSE_IMAGE_FORMAT_NONSTANDARD_BLOCK_SIZE_BIT = 0x00000004,
}
pub type VkSparseImageFormatFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSparseMemoryBindFlagBits {
	DUMMY = 0,
	VK_SPARSE_MEMORY_BIND_METADATA_BIT = 0x00000001,
}
pub type VkSparseMemoryBindFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkFenceCreateFlagBits {
	DUMMY = 0,
	VK_FENCE_CREATE_SIGNALED_BIT = 0x00000001,
}
pub type VkFenceCreateFlags = VkFlags;
pub type VkSemaphoreCreateFlags = VkFlags;
pub type VkEventCreateFlags = VkFlags;
pub type VkQueryPoolCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkQueryPipelineStatisticFlagBits {
	VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_VERTICES_BIT = 0x00000001,
	VK_QUERY_PIPELINE_STATISTIC_INPUT_ASSEMBLY_PRIMITIVES_BIT = 0x00000002,
	VK_QUERY_PIPELINE_STATISTIC_VERTEX_SHADER_INVOCATIONS_BIT = 0x00000004,
	VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_INVOCATIONS_BIT = 0x00000008,
	VK_QUERY_PIPELINE_STATISTIC_GEOMETRY_SHADER_PRIMITIVES_BIT = 0x00000010,
	VK_QUERY_PIPELINE_STATISTIC_CLIPPING_INVOCATIONS_BIT = 0x00000020,
	VK_QUERY_PIPELINE_STATISTIC_CLIPPING_PRIMITIVES_BIT = 0x00000040,
	VK_QUERY_PIPELINE_STATISTIC_FRAGMENT_SHADER_INVOCATIONS_BIT = 0x00000080,
	VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_CONTROL_SHADER_PATCHES_BIT = 0x00000100,
	VK_QUERY_PIPELINE_STATISTIC_TESSELLATION_EVALUATION_SHADER_INVOCATIONS_BIT = 0x00000200,
	VK_QUERY_PIPELINE_STATISTIC_COMPUTE_SHADER_INVOCATIONS_BIT = 0x00000400,
}
pub type VkQueryPipelineStatisticFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkQueryResultFlagBits {
	VK_QUERY_RESULT_64_BIT = 0x00000001,
	VK_QUERY_RESULT_WAIT_BIT = 0x00000002,
	VK_QUERY_RESULT_WITH_AVAILABILITY_BIT = 0x00000004,
	VK_QUERY_RESULT_PARTIAL_BIT = 0x00000008,
}
pub type VkQueryResultFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkBufferCreateFlagBits {
	VK_BUFFER_CREATE_SPARSE_BINDING_BIT = 0x00000001,
	VK_BUFFER_CREATE_SPARSE_RESIDENCY_BIT = 0x00000002,
	VK_BUFFER_CREATE_SPARSE_ALIASED_BIT = 0x00000004,
}
pub type VkBufferCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkBufferUsageFlagBits {
	VK_BUFFER_USAGE_TRANSFER_SRC_BIT = 0x00000001,
	VK_BUFFER_USAGE_TRANSFER_DST_BIT = 0x00000002,
	VK_BUFFER_USAGE_UNIFORM_TEXEL_BUFFER_BIT = 0x00000004,
	VK_BUFFER_USAGE_STORAGE_TEXEL_BUFFER_BIT = 0x00000008,
	VK_BUFFER_USAGE_UNIFORM_BUFFER_BIT = 0x00000010,
	VK_BUFFER_USAGE_STORAGE_BUFFER_BIT = 0x00000020,
	VK_BUFFER_USAGE_INDEX_BUFFER_BIT = 0x00000040,
	VK_BUFFER_USAGE_VERTEX_BUFFER_BIT = 0x00000080,
	VK_BUFFER_USAGE_INDIRECT_BUFFER_BIT = 0x00000100,
}
pub type VkBufferUsageFlags = VkFlags;
pub type VkBufferViewCreateFlags = VkFlags;
pub type VkImageViewCreateFlags = VkFlags;
pub type VkShaderModuleCreateFlags = VkFlags;
pub type VkPipelineCacheCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPipelineCreateFlagBits {
	VK_PIPELINE_CREATE_DISABLE_OPTIMIZATION_BIT = 0x00000001,
	VK_PIPELINE_CREATE_ALLOW_DERIVATIVES_BIT = 0x00000002,
	VK_PIPELINE_CREATE_DERIVATIVE_BIT = 0x00000004,
}
pub type VkPipelineCreateFlags = VkFlags;
pub type VkPipelineShaderStageCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkShaderStageFlagBits {
	VK_SHADER_STAGE_VERTEX_BIT = 0x00000001,
	VK_SHADER_STAGE_TESSELLATION_CONTROL_BIT = 0x00000002,
	VK_SHADER_STAGE_TESSELLATION_EVALUATION_BIT = 0x00000004,
	VK_SHADER_STAGE_GEOMETRY_BIT = 0x00000008,
	VK_SHADER_STAGE_FRAGMENT_BIT = 0x00000010,
	VK_SHADER_STAGE_COMPUTE_BIT = 0x00000020,
	VK_SHADER_STAGE_ALL_GRAPHICS = 0x1F,
	VK_SHADER_STAGE_ALL = 0x7FFFFFFF,
}
pub type VkPipelineVertexInputStateCreateFlags = VkFlags;
pub type VkPipelineInputAssemblyStateCreateFlags = VkFlags;
pub type VkPipelineTessellationStateCreateFlags = VkFlags;
pub type VkPipelineViewportStateCreateFlags = VkFlags;
pub type VkPipelineRasterizationStateCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCullModeFlagBits {
	VK_CULL_MODE_NONE = 0,
	VK_CULL_MODE_FRONT_BIT = 0x00000001,
	VK_CULL_MODE_BACK_BIT = 0x00000002,
	VK_CULL_MODE_FRONT_AND_BACK = 0x3,
}
pub type VkCullModeFlags = VkFlags;
pub type VkPipelineMultisampleStateCreateFlags = VkFlags;
pub type VkPipelineDepthStencilStateCreateFlags = VkFlags;
pub type VkPipelineColorBlendStateCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkColorComponentFlagBits {
	VK_COLOR_COMPONENT_R_BIT = 0x00000001,
	VK_COLOR_COMPONENT_G_BIT = 0x00000002,
	VK_COLOR_COMPONENT_B_BIT = 0x00000004,
	VK_COLOR_COMPONENT_A_BIT = 0x00000008,
}
pub type VkColorComponentFlags = VkFlags;
pub type VkPipelineDynamicStateCreateFlags = VkFlags;
pub type VkPipelineLayoutCreateFlags = VkFlags;
pub type VkShaderStageFlags = VkFlags;
pub type VkSamplerCreateFlags = VkFlags;
pub type VkDescriptorSetLayoutCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkDescriptorPoolCreateFlagBits {
	DUMMY = 0,
	VK_DESCRIPTOR_POOL_CREATE_FREE_DESCRIPTOR_SET_BIT = 0x00000001,
}
pub type VkDescriptorPoolCreateFlags = VkFlags;
pub type VkDescriptorPoolResetFlags = VkFlags;
pub type VkFramebufferCreateFlags = VkFlags;
pub type VkRenderPassCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkAttachmentDescriptionFlagBits {
	DUMMY = 0,
	VK_ATTACHMENT_DESCRIPTION_MAY_ALIAS_BIT = 0x00000001,
}
pub type VkAttachmentDescriptionFlags = VkFlags;
pub type VkSubpassDescriptionFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkAccessFlagBits {
	VK_ACCESS_INDIRECT_COMMAND_READ_BIT = 0x00000001,
	VK_ACCESS_INDEX_READ_BIT = 0x00000002,
	VK_ACCESS_VERTEX_ATTRIBUTE_READ_BIT = 0x00000004,
	VK_ACCESS_UNIFORM_READ_BIT = 0x00000008,
	VK_ACCESS_INPUT_ATTACHMENT_READ_BIT = 0x00000010,
	VK_ACCESS_SHADER_READ_BIT = 0x00000020,
	VK_ACCESS_SHADER_WRITE_BIT = 0x00000040,
	VK_ACCESS_COLOR_ATTACHMENT_READ_BIT = 0x00000080,
	VK_ACCESS_COLOR_ATTACHMENT_WRITE_BIT = 0x00000100,
	VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_READ_BIT = 0x00000200,
	VK_ACCESS_DEPTH_STENCIL_ATTACHMENT_WRITE_BIT = 0x00000400,
	VK_ACCESS_TRANSFER_READ_BIT = 0x00000800,
	VK_ACCESS_TRANSFER_WRITE_BIT = 0x00001000,
	VK_ACCESS_HOST_READ_BIT = 0x00002000,
	VK_ACCESS_HOST_WRITE_BIT = 0x00004000,
	VK_ACCESS_MEMORY_READ_BIT = 0x00008000,
	VK_ACCESS_MEMORY_WRITE_BIT = 0x00010000,
}
pub type VkAccessFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkDependencyFlagBits {
	DUMMY = 0,
	VK_DEPENDENCY_BY_REGION_BIT = 0x00000001,
}
pub type VkDependencyFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCommandPoolCreateFlagBits {
	VK_COMMAND_POOL_CREATE_TRANSIENT_BIT = 0x00000001,
	VK_COMMAND_POOL_CREATE_RESET_COMMAND_BUFFER_BIT = 0x00000002,
}
pub type VkCommandPoolCreateFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCommandPoolResetFlagBits {
	DUMMY = 0,
	VK_COMMAND_POOL_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
}
pub type VkCommandPoolResetFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCommandBufferUsageFlagBits {
	VK_COMMAND_BUFFER_USAGE_ONE_TIME_SUBMIT_BIT = 0x00000001,
	VK_COMMAND_BUFFER_USAGE_RENDER_PASS_CONTINUE_BIT = 0x00000002,
	VK_COMMAND_BUFFER_USAGE_SIMULTANEOUS_USE_BIT = 0x00000004,
}
pub type VkCommandBufferUsageFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkQueryControlFlagBits {
	DUMMY = 0,
	VK_QUERY_CONTROL_PRECISE_BIT = 0x00000001,
}
pub type VkQueryControlFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCommandBufferResetFlagBits {
	DUMMY = 0,
	VK_COMMAND_BUFFER_RESET_RELEASE_RESOURCES_BIT = 0x00000001,
}
pub type VkCommandBufferResetFlags = VkFlags;

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkStencilFaceFlagBits {
	VK_STENCIL_FACE_FRONT_BIT = 0x00000001,
	VK_STENCIL_FACE_BACK_BIT = 0x00000002,
	VK_STENCIL_FRONT_AND_BACK = 0x3,
}
pub type VkStencilFaceFlags = VkFlags;

