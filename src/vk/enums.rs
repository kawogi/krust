


#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPipelineCacheHeaderVersion {
	DUMMY = 0,
	VK_PIPELINE_CACHE_HEADER_VERSION_ONE = 1,
	//VK_PIPELINE_CACHE_HEADER_VERSION_BEGIN_RANGE = VK_PIPELINE_CACHE_HEADER_VERSION_ONE,
	//VK_PIPELINE_CACHE_HEADER_VERSION_END_RANGE = VK_PIPELINE_CACHE_HEADER_VERSION_ONE,
	//VK_PIPELINE_CACHE_HEADER_VERSION_RANGE_SIZE = (VK_PIPELINE_CACHE_HEADER_VERSION_ONE - VK_PIPELINE_CACHE_HEADER_VERSION_ONE + 1),
	//VK_PIPELINE_CACHE_HEADER_VERSION_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkResult {
	VK_SUCCESS = 0,
	VK_NOT_READY = 1,
	VK_TIMEOUT = 2,
	VK_EVENT_SET = 3,
	VK_EVENT_RESET = 4,
	VK_INCOMPLETE = 5,
	VK_ERROR_OUT_OF_HOST_MEMORY = -1,
	VK_ERROR_OUT_OF_DEVICE_MEMORY = -2,
	VK_ERROR_INITIALIZATION_FAILED = -3,
	VK_ERROR_DEVICE_LOST = -4,
	VK_ERROR_MEMORY_MAP_FAILED = -5,
	VK_ERROR_LAYER_NOT_PRESENT = -6,
	VK_ERROR_EXTENSION_NOT_PRESENT = -7,
	VK_ERROR_FEATURE_NOT_PRESENT = -8,
	VK_ERROR_INCOMPATIBLE_DRIVER = -9,
	VK_ERROR_TOO_MANY_OBJECTS = -10,
	VK_ERROR_FORMAT_NOT_SUPPORTED = -11,
	VK_ERROR_SURFACE_LOST_KHR = -1000000000,
	VK_ERROR_NATIVE_WINDOW_IN_USE_KHR = -1000000001,
	VK_SUBOPTIMAL_KHR = 1000001003,
	VK_ERROR_OUT_OF_DATE_KHR = -1000001004,
	VK_ERROR_INCOMPATIBLE_DISPLAY_KHR = -1000003001,
	VK_ERROR_VALIDATION_FAILED_EXT = -1000011001,
	//VK_RESULT_BEGIN_RANGE = VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED,
	//VK_RESULT_END_RANGE = VkResult::VK_INCOMPLETE,
	//VK_RESULT_RANGE_SIZE = (VkResult::VK_INCOMPLETE - VkResult::VK_ERROR_FORMAT_NOT_SUPPORTED + 1),
	//VK_RESULT_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkStructureType {
	VK_STRUCTURE_TYPE_APPLICATION_INFO = 0,
	VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO = 1,
	VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO = 2,
	VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO = 3,
	VK_STRUCTURE_TYPE_SUBMIT_INFO = 4,
	VK_STRUCTURE_TYPE_MEMORY_ALLOCATE_INFO = 5,
	VK_STRUCTURE_TYPE_MAPPED_MEMORY_RANGE = 6,
	VK_STRUCTURE_TYPE_BIND_SPARSE_INFO = 7,
	VK_STRUCTURE_TYPE_FENCE_CREATE_INFO = 8,
	VK_STRUCTURE_TYPE_SEMAPHORE_CREATE_INFO = 9,
	VK_STRUCTURE_TYPE_EVENT_CREATE_INFO = 10,
	VK_STRUCTURE_TYPE_QUERY_POOL_CREATE_INFO = 11,
	VK_STRUCTURE_TYPE_BUFFER_CREATE_INFO = 12,
	VK_STRUCTURE_TYPE_BUFFER_VIEW_CREATE_INFO = 13,
	VK_STRUCTURE_TYPE_IMAGE_CREATE_INFO = 14,
	VK_STRUCTURE_TYPE_IMAGE_VIEW_CREATE_INFO = 15,
	VK_STRUCTURE_TYPE_SHADER_MODULE_CREATE_INFO = 16,
	VK_STRUCTURE_TYPE_PIPELINE_CACHE_CREATE_INFO = 17,
	VK_STRUCTURE_TYPE_PIPELINE_SHADER_STAGE_CREATE_INFO = 18,
	VK_STRUCTURE_TYPE_PIPELINE_VERTEX_INPUT_STATE_CREATE_INFO = 19,
	VK_STRUCTURE_TYPE_PIPELINE_INPUT_ASSEMBLY_STATE_CREATE_INFO = 20,
	VK_STRUCTURE_TYPE_PIPELINE_TESSELLATION_STATE_CREATE_INFO = 21,
	VK_STRUCTURE_TYPE_PIPELINE_VIEWPORT_STATE_CREATE_INFO = 22,
	VK_STRUCTURE_TYPE_PIPELINE_RASTERIZATION_STATE_CREATE_INFO = 23,
	VK_STRUCTURE_TYPE_PIPELINE_MULTISAMPLE_STATE_CREATE_INFO = 24,
	VK_STRUCTURE_TYPE_PIPELINE_DEPTH_STENCIL_STATE_CREATE_INFO = 25,
	VK_STRUCTURE_TYPE_PIPELINE_COLOR_BLEND_STATE_CREATE_INFO = 26,
	VK_STRUCTURE_TYPE_PIPELINE_DYNAMIC_STATE_CREATE_INFO = 27,
	VK_STRUCTURE_TYPE_GRAPHICS_PIPELINE_CREATE_INFO = 28,
	VK_STRUCTURE_TYPE_COMPUTE_PIPELINE_CREATE_INFO = 29,
	VK_STRUCTURE_TYPE_PIPELINE_LAYOUT_CREATE_INFO = 30,
	VK_STRUCTURE_TYPE_SAMPLER_CREATE_INFO = 31,
	VK_STRUCTURE_TYPE_DESCRIPTOR_SET_LAYOUT_CREATE_INFO = 32,
	VK_STRUCTURE_TYPE_DESCRIPTOR_POOL_CREATE_INFO = 33,
	VK_STRUCTURE_TYPE_DESCRIPTOR_SET_ALLOCATE_INFO = 34,
	VK_STRUCTURE_TYPE_WRITE_DESCRIPTOR_SET = 35,
	VK_STRUCTURE_TYPE_COPY_DESCRIPTOR_SET = 36,
	VK_STRUCTURE_TYPE_FRAMEBUFFER_CREATE_INFO = 37,
	VK_STRUCTURE_TYPE_RENDER_PASS_CREATE_INFO = 38,
	VK_STRUCTURE_TYPE_COMMAND_POOL_CREATE_INFO = 39,
	VK_STRUCTURE_TYPE_COMMAND_BUFFER_ALLOCATE_INFO = 40,
	VK_STRUCTURE_TYPE_COMMAND_BUFFER_INHERITANCE_INFO = 41,
	VK_STRUCTURE_TYPE_COMMAND_BUFFER_BEGIN_INFO = 42,
	VK_STRUCTURE_TYPE_RENDER_PASS_BEGIN_INFO = 43,
	VK_STRUCTURE_TYPE_BUFFER_MEMORY_BARRIER = 44,
	VK_STRUCTURE_TYPE_IMAGE_MEMORY_BARRIER = 45,
	VK_STRUCTURE_TYPE_MEMORY_BARRIER = 46,
	VK_STRUCTURE_TYPE_LOADER_INSTANCE_CREATE_INFO = 47,
	VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO = 48,
	VK_STRUCTURE_TYPE_SWAPCHAIN_CREATE_INFO_KHR = 1000001000,
	VK_STRUCTURE_TYPE_PRESENT_INFO_KHR = 1000001001,
	VK_STRUCTURE_TYPE_DISPLAY_MODE_CREATE_INFO_KHR = 1000002000,
	VK_STRUCTURE_TYPE_DISPLAY_SURFACE_CREATE_INFO_KHR = 1000002001,
	VK_STRUCTURE_TYPE_DISPLAY_PRESENT_INFO_KHR = 1000003000,
	VK_STRUCTURE_TYPE_XLIB_SURFACE_CREATE_INFO_KHR = 1000004000,
	VK_STRUCTURE_TYPE_XCB_SURFACE_CREATE_INFO_KHR = 1000005000,
	VK_STRUCTURE_TYPE_WAYLAND_SURFACE_CREATE_INFO_KHR = 1000006000,
	VK_STRUCTURE_TYPE_MIR_SURFACE_CREATE_INFO_KHR = 1000007000,
	VK_STRUCTURE_TYPE_ANDROID_SURFACE_CREATE_INFO_KHR = 1000008000,
	VK_STRUCTURE_TYPE_WIN32_SURFACE_CREATE_INFO_KHR = 1000009000,
	VK_STRUCTURE_TYPE_DEBUG_REPORT_CREATE_INFO_EXT = 1000011000,
	//VK_STRUCTURE_TYPE_BEGIN_RANGE = VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO,
	//VK_STRUCTURE_TYPE_END_RANGE = VkStructureType::VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO,
	//VK_STRUCTURE_TYPE_RANGE_SIZE = (VkStructureType::VK_STRUCTURE_TYPE_LOADER_DEVICE_CREATE_INFO - VkStructureType::VK_STRUCTURE_TYPE_APPLICATION_INFO + 1),
	//VK_STRUCTURE_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSystemAllocationScope {
	VK_SYSTEM_ALLOCATION_SCOPE_COMMAND = 0,
	VK_SYSTEM_ALLOCATION_SCOPE_OBJECT = 1,
	VK_SYSTEM_ALLOCATION_SCOPE_CACHE = 2,
	VK_SYSTEM_ALLOCATION_SCOPE_DEVICE = 3,
	VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE = 4,
	//VK_SYSTEM_ALLOCATION_SCOPE_BEGIN_RANGE = VK_SYSTEM_ALLOCATION_SCOPE_COMMAND,
	//VK_SYSTEM_ALLOCATION_SCOPE_END_RANGE = VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE,
	//VK_SYSTEM_ALLOCATION_SCOPE_RANGE_SIZE = (VK_SYSTEM_ALLOCATION_SCOPE_INSTANCE - VK_SYSTEM_ALLOCATION_SCOPE_COMMAND + 1),
	//VK_SYSTEM_ALLOCATION_SCOPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkInternalAllocationType {
	DUMMY = 1,
	VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE = 0,
	//VK_INTERNAL_ALLOCATION_TYPE_BEGIN_RANGE = VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE,
	//VK_INTERNAL_ALLOCATION_TYPE_END_RANGE = VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE,
	//VK_INTERNAL_ALLOCATION_TYPE_RANGE_SIZE = (VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE - VK_INTERNAL_ALLOCATION_TYPE_EXECUTABLE + 1),
	//VK_INTERNAL_ALLOCATION_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkFormat {
	VK_FORMAT_UNDEFINED = 0,
	VK_FORMAT_R4G4_UNORM_PACK8 = 1,
	VK_FORMAT_R4G4B4A4_UNORM_PACK16 = 2,
	VK_FORMAT_B4G4R4A4_UNORM_PACK16 = 3,
	VK_FORMAT_R5G6B5_UNORM_PACK16 = 4,
	VK_FORMAT_B5G6R5_UNORM_PACK16 = 5,
	VK_FORMAT_R5G5B5A1_UNORM_PACK16 = 6,
	VK_FORMAT_B5G5R5A1_UNORM_PACK16 = 7,
	VK_FORMAT_A1R5G5B5_UNORM_PACK16 = 8,
	VK_FORMAT_R8_UNORM = 9,
	VK_FORMAT_R8_SNORM = 10,
	VK_FORMAT_R8_USCALED = 11,
	VK_FORMAT_R8_SSCALED = 12,
	VK_FORMAT_R8_UINT = 13,
	VK_FORMAT_R8_SINT = 14,
	VK_FORMAT_R8_SRGB = 15,
	VK_FORMAT_R8G8_UNORM = 16,
	VK_FORMAT_R8G8_SNORM = 17,
	VK_FORMAT_R8G8_USCALED = 18,
	VK_FORMAT_R8G8_SSCALED = 19,
	VK_FORMAT_R8G8_UINT = 20,
	VK_FORMAT_R8G8_SINT = 21,
	VK_FORMAT_R8G8_SRGB = 22,
	VK_FORMAT_R8G8B8_UNORM = 23,
	VK_FORMAT_R8G8B8_SNORM = 24,
	VK_FORMAT_R8G8B8_USCALED = 25,
	VK_FORMAT_R8G8B8_SSCALED = 26,
	VK_FORMAT_R8G8B8_UINT = 27,
	VK_FORMAT_R8G8B8_SINT = 28,
	VK_FORMAT_R8G8B8_SRGB = 29,
	VK_FORMAT_B8G8R8_UNORM = 30,
	VK_FORMAT_B8G8R8_SNORM = 31,
	VK_FORMAT_B8G8R8_USCALED = 32,
	VK_FORMAT_B8G8R8_SSCALED = 33,
	VK_FORMAT_B8G8R8_UINT = 34,
	VK_FORMAT_B8G8R8_SINT = 35,
	VK_FORMAT_B8G8R8_SRGB = 36,
	VK_FORMAT_R8G8B8A8_UNORM = 37,
	VK_FORMAT_R8G8B8A8_SNORM = 38,
	VK_FORMAT_R8G8B8A8_USCALED = 39,
	VK_FORMAT_R8G8B8A8_SSCALED = 40,
	VK_FORMAT_R8G8B8A8_UINT = 41,
	VK_FORMAT_R8G8B8A8_SINT = 42,
	VK_FORMAT_R8G8B8A8_SRGB = 43,
	VK_FORMAT_B8G8R8A8_UNORM = 44,
	VK_FORMAT_B8G8R8A8_SNORM = 45,
	VK_FORMAT_B8G8R8A8_USCALED = 46,
	VK_FORMAT_B8G8R8A8_SSCALED = 47,
	VK_FORMAT_B8G8R8A8_UINT = 48,
	VK_FORMAT_B8G8R8A8_SINT = 49,
	VK_FORMAT_B8G8R8A8_SRGB = 50,
	VK_FORMAT_A8B8G8R8_UNORM_PACK32 = 51,
	VK_FORMAT_A8B8G8R8_SNORM_PACK32 = 52,
	VK_FORMAT_A8B8G8R8_USCALED_PACK32 = 53,
	VK_FORMAT_A8B8G8R8_SSCALED_PACK32 = 54,
	VK_FORMAT_A8B8G8R8_UINT_PACK32 = 55,
	VK_FORMAT_A8B8G8R8_SINT_PACK32 = 56,
	VK_FORMAT_A8B8G8R8_SRGB_PACK32 = 57,
	VK_FORMAT_A2R10G10B10_UNORM_PACK32 = 58,
	VK_FORMAT_A2R10G10B10_SNORM_PACK32 = 59,
	VK_FORMAT_A2R10G10B10_USCALED_PACK32 = 60,
	VK_FORMAT_A2R10G10B10_SSCALED_PACK32 = 61,
	VK_FORMAT_A2R10G10B10_UINT_PACK32 = 62,
	VK_FORMAT_A2R10G10B10_SINT_PACK32 = 63,
	VK_FORMAT_A2B10G10R10_UNORM_PACK32 = 64,
	VK_FORMAT_A2B10G10R10_SNORM_PACK32 = 65,
	VK_FORMAT_A2B10G10R10_USCALED_PACK32 = 66,
	VK_FORMAT_A2B10G10R10_SSCALED_PACK32 = 67,
	VK_FORMAT_A2B10G10R10_UINT_PACK32 = 68,
	VK_FORMAT_A2B10G10R10_SINT_PACK32 = 69,
	VK_FORMAT_R16_UNORM = 70,
	VK_FORMAT_R16_SNORM = 71,
	VK_FORMAT_R16_USCALED = 72,
	VK_FORMAT_R16_SSCALED = 73,
	VK_FORMAT_R16_UINT = 74,
	VK_FORMAT_R16_SINT = 75,
	VK_FORMAT_R16_SFLOAT = 76,
	VK_FORMAT_R16G16_UNORM = 77,
	VK_FORMAT_R16G16_SNORM = 78,
	VK_FORMAT_R16G16_USCALED = 79,
	VK_FORMAT_R16G16_SSCALED = 80,
	VK_FORMAT_R16G16_UINT = 81,
	VK_FORMAT_R16G16_SINT = 82,
	VK_FORMAT_R16G16_SFLOAT = 83,
	VK_FORMAT_R16G16B16_UNORM = 84,
	VK_FORMAT_R16G16B16_SNORM = 85,
	VK_FORMAT_R16G16B16_USCALED = 86,
	VK_FORMAT_R16G16B16_SSCALED = 87,
	VK_FORMAT_R16G16B16_UINT = 88,
	VK_FORMAT_R16G16B16_SINT = 89,
	VK_FORMAT_R16G16B16_SFLOAT = 90,
	VK_FORMAT_R16G16B16A16_UNORM = 91,
	VK_FORMAT_R16G16B16A16_SNORM = 92,
	VK_FORMAT_R16G16B16A16_USCALED = 93,
	VK_FORMAT_R16G16B16A16_SSCALED = 94,
	VK_FORMAT_R16G16B16A16_UINT = 95,
	VK_FORMAT_R16G16B16A16_SINT = 96,
	VK_FORMAT_R16G16B16A16_SFLOAT = 97,
	VK_FORMAT_R32_UINT = 98,
	VK_FORMAT_R32_SINT = 99,
	VK_FORMAT_R32_SFLOAT = 100,
	VK_FORMAT_R32G32_UINT = 101,
	VK_FORMAT_R32G32_SINT = 102,
	VK_FORMAT_R32G32_SFLOAT = 103,
	VK_FORMAT_R32G32B32_UINT = 104,
	VK_FORMAT_R32G32B32_SINT = 105,
	VK_FORMAT_R32G32B32_SFLOAT = 106,
	VK_FORMAT_R32G32B32A32_UINT = 107,
	VK_FORMAT_R32G32B32A32_SINT = 108,
	VK_FORMAT_R32G32B32A32_SFLOAT = 109,
	VK_FORMAT_R64_UINT = 110,
	VK_FORMAT_R64_SINT = 111,
	VK_FORMAT_R64_SFLOAT = 112,
	VK_FORMAT_R64G64_UINT = 113,
	VK_FORMAT_R64G64_SINT = 114,
	VK_FORMAT_R64G64_SFLOAT = 115,
	VK_FORMAT_R64G64B64_UINT = 116,
	VK_FORMAT_R64G64B64_SINT = 117,
	VK_FORMAT_R64G64B64_SFLOAT = 118,
	VK_FORMAT_R64G64B64A64_UINT = 119,
	VK_FORMAT_R64G64B64A64_SINT = 120,
	VK_FORMAT_R64G64B64A64_SFLOAT = 121,
	VK_FORMAT_B10G11R11_UFLOAT_PACK32 = 122,
	VK_FORMAT_E5B9G9R9_UFLOAT_PACK32 = 123,
	VK_FORMAT_D16_UNORM = 124,
	VK_FORMAT_X8_D24_UNORM_PACK32 = 125,
	VK_FORMAT_D32_SFLOAT = 126,
	VK_FORMAT_S8_UINT = 127,
	VK_FORMAT_D16_UNORM_S8_UINT = 128,
	VK_FORMAT_D24_UNORM_S8_UINT = 129,
	VK_FORMAT_D32_SFLOAT_S8_UINT = 130,
	VK_FORMAT_BC1_RGB_UNORM_BLOCK = 131,
	VK_FORMAT_BC1_RGB_SRGB_BLOCK = 132,
	VK_FORMAT_BC1_RGBA_UNORM_BLOCK = 133,
	VK_FORMAT_BC1_RGBA_SRGB_BLOCK = 134,
	VK_FORMAT_BC2_UNORM_BLOCK = 135,
	VK_FORMAT_BC2_SRGB_BLOCK = 136,
	VK_FORMAT_BC3_UNORM_BLOCK = 137,
	VK_FORMAT_BC3_SRGB_BLOCK = 138,
	VK_FORMAT_BC4_UNORM_BLOCK = 139,
	VK_FORMAT_BC4_SNORM_BLOCK = 140,
	VK_FORMAT_BC5_UNORM_BLOCK = 141,
	VK_FORMAT_BC5_SNORM_BLOCK = 142,
	VK_FORMAT_BC6H_UFLOAT_BLOCK = 143,
	VK_FORMAT_BC6H_SFLOAT_BLOCK = 144,
	VK_FORMAT_BC7_UNORM_BLOCK = 145,
	VK_FORMAT_BC7_SRGB_BLOCK = 146,
	VK_FORMAT_ETC2_R8G8B8_UNORM_BLOCK = 147,
	VK_FORMAT_ETC2_R8G8B8_SRGB_BLOCK = 148,
	VK_FORMAT_ETC2_R8G8B8A1_UNORM_BLOCK = 149,
	VK_FORMAT_ETC2_R8G8B8A1_SRGB_BLOCK = 150,
	VK_FORMAT_ETC2_R8G8B8A8_UNORM_BLOCK = 151,
	VK_FORMAT_ETC2_R8G8B8A8_SRGB_BLOCK = 152,
	VK_FORMAT_EAC_R11_UNORM_BLOCK = 153,
	VK_FORMAT_EAC_R11_SNORM_BLOCK = 154,
	VK_FORMAT_EAC_R11G11_UNORM_BLOCK = 155,
	VK_FORMAT_EAC_R11G11_SNORM_BLOCK = 156,
	VK_FORMAT_ASTC_4x4_UNORM_BLOCK = 157,
	VK_FORMAT_ASTC_4x4_SRGB_BLOCK = 158,
	VK_FORMAT_ASTC_5x4_UNORM_BLOCK = 159,
	VK_FORMAT_ASTC_5x4_SRGB_BLOCK = 160,
	VK_FORMAT_ASTC_5x5_UNORM_BLOCK = 161,
	VK_FORMAT_ASTC_5x5_SRGB_BLOCK = 162,
	VK_FORMAT_ASTC_6x5_UNORM_BLOCK = 163,
	VK_FORMAT_ASTC_6x5_SRGB_BLOCK = 164,
	VK_FORMAT_ASTC_6x6_UNORM_BLOCK = 165,
	VK_FORMAT_ASTC_6x6_SRGB_BLOCK = 166,
	VK_FORMAT_ASTC_8x5_UNORM_BLOCK = 167,
	VK_FORMAT_ASTC_8x5_SRGB_BLOCK = 168,
	VK_FORMAT_ASTC_8x6_UNORM_BLOCK = 169,
	VK_FORMAT_ASTC_8x6_SRGB_BLOCK = 170,
	VK_FORMAT_ASTC_8x8_UNORM_BLOCK = 171,
	VK_FORMAT_ASTC_8x8_SRGB_BLOCK = 172,
	VK_FORMAT_ASTC_10x5_UNORM_BLOCK = 173,
	VK_FORMAT_ASTC_10x5_SRGB_BLOCK = 174,
	VK_FORMAT_ASTC_10x6_UNORM_BLOCK = 175,
	VK_FORMAT_ASTC_10x6_SRGB_BLOCK = 176,
	VK_FORMAT_ASTC_10x8_UNORM_BLOCK = 177,
	VK_FORMAT_ASTC_10x8_SRGB_BLOCK = 178,
	VK_FORMAT_ASTC_10x10_UNORM_BLOCK = 179,
	VK_FORMAT_ASTC_10x10_SRGB_BLOCK = 180,
	VK_FORMAT_ASTC_12x10_UNORM_BLOCK = 181,
	VK_FORMAT_ASTC_12x10_SRGB_BLOCK = 182,
	VK_FORMAT_ASTC_12x12_UNORM_BLOCK = 183,
	VK_FORMAT_ASTC_12x12_SRGB_BLOCK = 184,
	//VK_FORMAT_BEGIN_RANGE = VK_FORMAT_UNDEFINED,
	//VK_FORMAT_END_RANGE = VK_FORMAT_ASTC_12x12_SRGB_BLOCK,
	//VK_FORMAT_RANGE_SIZE = (VK_FORMAT_ASTC_12x12_SRGB_BLOCK - VK_FORMAT_UNDEFINED + 1),
	//VK_FORMAT_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageType {
	VK_IMAGE_TYPE_1D = 0,
	VK_IMAGE_TYPE_2D = 1,
	VK_IMAGE_TYPE_3D = 2,
	//VK_IMAGE_TYPE_BEGIN_RANGE = VK_IMAGE_TYPE_1D,
	//VK_IMAGE_TYPE_END_RANGE = VK_IMAGE_TYPE_3D,
	//VK_IMAGE_TYPE_RANGE_SIZE = (VK_IMAGE_TYPE_3D - VK_IMAGE_TYPE_1D + 1),
	//VK_IMAGE_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageTiling {
	VK_IMAGE_TILING_OPTIMAL = 0,
	VK_IMAGE_TILING_LINEAR = 1,
	//VK_IMAGE_TILING_BEGIN_RANGE = VK_IMAGE_TILING_OPTIMAL,
	//VK_IMAGE_TILING_END_RANGE = VK_IMAGE_TILING_LINEAR,
	//VK_IMAGE_TILING_RANGE_SIZE = (VK_IMAGE_TILING_LINEAR - VK_IMAGE_TILING_OPTIMAL + 1),
	//VK_IMAGE_TILING_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPhysicalDeviceType {
	VK_PHYSICAL_DEVICE_TYPE_OTHER = 0,
	VK_PHYSICAL_DEVICE_TYPE_INTEGRATED_GPU = 1,
	VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU = 2,
	VK_PHYSICAL_DEVICE_TYPE_VIRTUAL_GPU = 3,
	VK_PHYSICAL_DEVICE_TYPE_CPU = 4,
	//VK_PHYSICAL_DEVICE_TYPE_BEGIN_RANGE = VK_PHYSICAL_DEVICE_TYPE_OTHER,
	//VK_PHYSICAL_DEVICE_TYPE_END_RANGE = VK_PHYSICAL_DEVICE_TYPE_CPU,
	//VK_PHYSICAL_DEVICE_TYPE_RANGE_SIZE = (VK_PHYSICAL_DEVICE_TYPE_CPU - VK_PHYSICAL_DEVICE_TYPE_OTHER + 1),
	//VK_PHYSICAL_DEVICE_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkQueryType {
	VK_QUERY_TYPE_OCCLUSION = 0,
	VK_QUERY_TYPE_PIPELINE_STATISTICS = 1,
	VK_QUERY_TYPE_TIMESTAMP = 2,
	//VK_QUERY_TYPE_BEGIN_RANGE = VK_QUERY_TYPE_OCCLUSION,
	//VK_QUERY_TYPE_END_RANGE = VK_QUERY_TYPE_TIMESTAMP,
	//VK_QUERY_TYPE_RANGE_SIZE = (VK_QUERY_TYPE_TIMESTAMP - VK_QUERY_TYPE_OCCLUSION + 1),
	//VK_QUERY_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSharingMode {
	VK_SHARING_MODE_EXCLUSIVE = 0,
	VK_SHARING_MODE_CONCURRENT = 1,
	//VK_SHARING_MODE_BEGIN_RANGE = VK_SHARING_MODE_EXCLUSIVE,
	//VK_SHARING_MODE_END_RANGE = VK_SHARING_MODE_CONCURRENT,
	//VK_SHARING_MODE_RANGE_SIZE = (VK_SHARING_MODE_CONCURRENT - VK_SHARING_MODE_EXCLUSIVE + 1),
	//VK_SHARING_MODE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageLayout {
	VK_IMAGE_LAYOUT_UNDEFINED = 0,
	VK_IMAGE_LAYOUT_GENERAL = 1,
	VK_IMAGE_LAYOUT_COLOR_ATTACHMENT_OPTIMAL = 2,
	VK_IMAGE_LAYOUT_DEPTH_STENCIL_ATTACHMENT_OPTIMAL = 3,
	VK_IMAGE_LAYOUT_DEPTH_STENCIL_READ_ONLY_OPTIMAL = 4,
	VK_IMAGE_LAYOUT_SHADER_READ_ONLY_OPTIMAL = 5,
	VK_IMAGE_LAYOUT_TRANSFER_SRC_OPTIMAL = 6,
	VK_IMAGE_LAYOUT_TRANSFER_DST_OPTIMAL = 7,
	VK_IMAGE_LAYOUT_PREINITIALIZED = 8,
	VK_IMAGE_LAYOUT_PRESENT_SRC_KHR = 1000001002,
	//VK_IMAGE_LAYOUT_BEGIN_RANGE = VK_IMAGE_LAYOUT_UNDEFINED,
	//VK_IMAGE_LAYOUT_END_RANGE = VK_IMAGE_LAYOUT_PREINITIALIZED,
	//VK_IMAGE_LAYOUT_RANGE_SIZE = (VK_IMAGE_LAYOUT_PREINITIALIZED - VK_IMAGE_LAYOUT_UNDEFINED + 1),
	//VK_IMAGE_LAYOUT_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkImageViewType {
	VK_IMAGE_VIEW_TYPE_1D = 0,
	VK_IMAGE_VIEW_TYPE_2D = 1,
	VK_IMAGE_VIEW_TYPE_3D = 2,
	VK_IMAGE_VIEW_TYPE_CUBE = 3,
	VK_IMAGE_VIEW_TYPE_1D_ARRAY = 4,
	VK_IMAGE_VIEW_TYPE_2D_ARRAY = 5,
	VK_IMAGE_VIEW_TYPE_CUBE_ARRAY = 6,
	//VK_IMAGE_VIEW_TYPE_BEGIN_RANGE = VK_IMAGE_VIEW_TYPE_1D,
	//VK_IMAGE_VIEW_TYPE_END_RANGE = VK_IMAGE_VIEW_TYPE_CUBE_ARRAY,
	//VK_IMAGE_VIEW_TYPE_RANGE_SIZE = (VK_IMAGE_VIEW_TYPE_CUBE_ARRAY - VK_IMAGE_VIEW_TYPE_1D + 1),
	//VK_IMAGE_VIEW_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkComponentSwizzle {
	VK_COMPONENT_SWIZZLE_IDENTITY = 0,
	VK_COMPONENT_SWIZZLE_ZERO = 1,
	VK_COMPONENT_SWIZZLE_ONE = 2,
	VK_COMPONENT_SWIZZLE_R = 3,
	VK_COMPONENT_SWIZZLE_G = 4,
	VK_COMPONENT_SWIZZLE_B = 5,
	VK_COMPONENT_SWIZZLE_A = 6,
	//VK_COMPONENT_SWIZZLE_BEGIN_RANGE = VK_COMPONENT_SWIZZLE_IDENTITY,
	//VK_COMPONENT_SWIZZLE_END_RANGE = VK_COMPONENT_SWIZZLE_A,
	//VK_COMPONENT_SWIZZLE_RANGE_SIZE = (VK_COMPONENT_SWIZZLE_A - VK_COMPONENT_SWIZZLE_IDENTITY + 1),
	//VK_COMPONENT_SWIZZLE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkVertexInputRate {
	VK_VERTEX_INPUT_RATE_VERTEX = 0,
	VK_VERTEX_INPUT_RATE_INSTANCE = 1,
	//VK_VERTEX_INPUT_RATE_BEGIN_RANGE = VK_VERTEX_INPUT_RATE_VERTEX,
	//VK_VERTEX_INPUT_RATE_END_RANGE = VK_VERTEX_INPUT_RATE_INSTANCE,
	//VK_VERTEX_INPUT_RATE_RANGE_SIZE = (VK_VERTEX_INPUT_RATE_INSTANCE - VK_VERTEX_INPUT_RATE_VERTEX + 1),
	//VK_VERTEX_INPUT_RATE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPrimitiveTopology {
	VK_PRIMITIVE_TOPOLOGY_POINT_LIST = 0,
	VK_PRIMITIVE_TOPOLOGY_LINE_LIST = 1,
	VK_PRIMITIVE_TOPOLOGY_LINE_STRIP = 2,
	VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST = 3,
	VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP = 4,
	VK_PRIMITIVE_TOPOLOGY_TRIANGLE_FAN = 5,
	VK_PRIMITIVE_TOPOLOGY_LINE_LIST_WITH_ADJACENCY = 6,
	VK_PRIMITIVE_TOPOLOGY_LINE_STRIP_WITH_ADJACENCY = 7,
	VK_PRIMITIVE_TOPOLOGY_TRIANGLE_LIST_WITH_ADJACENCY = 8,
	VK_PRIMITIVE_TOPOLOGY_TRIANGLE_STRIP_WITH_ADJACENCY = 9,
	VK_PRIMITIVE_TOPOLOGY_PATCH_LIST = 10,
	//VK_PRIMITIVE_TOPOLOGY_BEGIN_RANGE = VK_PRIMITIVE_TOPOLOGY_POINT_LIST,
	//VK_PRIMITIVE_TOPOLOGY_END_RANGE = VK_PRIMITIVE_TOPOLOGY_PATCH_LIST,
	//VK_PRIMITIVE_TOPOLOGY_RANGE_SIZE = (VK_PRIMITIVE_TOPOLOGY_PATCH_LIST - VK_PRIMITIVE_TOPOLOGY_POINT_LIST + 1),
	//VK_PRIMITIVE_TOPOLOGY_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPolygonMode {
	VK_POLYGON_MODE_FILL = 0,
	VK_POLYGON_MODE_LINE = 1,
	VK_POLYGON_MODE_POINT = 2,
	//VK_POLYGON_MODE_BEGIN_RANGE = VK_POLYGON_MODE_FILL,
	//VK_POLYGON_MODE_END_RANGE = VK_POLYGON_MODE_POINT,
	//VK_POLYGON_MODE_RANGE_SIZE = (VK_POLYGON_MODE_POINT - VK_POLYGON_MODE_FILL + 1),
	//VK_POLYGON_MODE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkFrontFace {
	VK_FRONT_FACE_COUNTER_CLOCKWISE = 0,
	VK_FRONT_FACE_CLOCKWISE = 1,
	//VK_FRONT_FACE_BEGIN_RANGE = VK_FRONT_FACE_COUNTER_CLOCKWISE,
	//VK_FRONT_FACE_END_RANGE = VK_FRONT_FACE_CLOCKWISE,
	//VK_FRONT_FACE_RANGE_SIZE = (VK_FRONT_FACE_CLOCKWISE - VK_FRONT_FACE_COUNTER_CLOCKWISE + 1),
	//VK_FRONT_FACE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCompareOp {
	VK_COMPARE_OP_NEVER = 0,
	VK_COMPARE_OP_LESS = 1,
	VK_COMPARE_OP_EQUAL = 2,
	VK_COMPARE_OP_LESS_OR_EQUAL = 3,
	VK_COMPARE_OP_GREATER = 4,
	VK_COMPARE_OP_NOT_EQUAL = 5,
	VK_COMPARE_OP_GREATER_OR_EQUAL = 6,
	VK_COMPARE_OP_ALWAYS = 7,
	//VK_COMPARE_OP_BEGIN_RANGE = VK_COMPARE_OP_NEVER,
	//VK_COMPARE_OP_END_RANGE = VK_COMPARE_OP_ALWAYS,
	//VK_COMPARE_OP_RANGE_SIZE = (VK_COMPARE_OP_ALWAYS - VK_COMPARE_OP_NEVER + 1),
	//VK_COMPARE_OP_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkStencilOp {
	VK_STENCIL_OP_KEEP = 0,
	VK_STENCIL_OP_ZERO = 1,
	VK_STENCIL_OP_REPLACE = 2,
	VK_STENCIL_OP_INCREMENT_AND_CLAMP = 3,
	VK_STENCIL_OP_DECREMENT_AND_CLAMP = 4,
	VK_STENCIL_OP_INVERT = 5,
	VK_STENCIL_OP_INCREMENT_AND_WRAP = 6,
	VK_STENCIL_OP_DECREMENT_AND_WRAP = 7,
	//VK_STENCIL_OP_BEGIN_RANGE = VK_STENCIL_OP_KEEP,
	//VK_STENCIL_OP_END_RANGE = VK_STENCIL_OP_DECREMENT_AND_WRAP,
	//VK_STENCIL_OP_RANGE_SIZE = (VK_STENCIL_OP_DECREMENT_AND_WRAP - VK_STENCIL_OP_KEEP + 1),
	//VK_STENCIL_OP_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkLogicOp {
	VK_LOGIC_OP_CLEAR = 0,
	VK_LOGIC_OP_AND = 1,
	VK_LOGIC_OP_AND_REVERSE = 2,
	VK_LOGIC_OP_COPY = 3,
	VK_LOGIC_OP_AND_INVERTED = 4,
	VK_LOGIC_OP_NO_OP = 5,
	VK_LOGIC_OP_XOR = 6,
	VK_LOGIC_OP_OR = 7,
	VK_LOGIC_OP_NOR = 8,
	VK_LOGIC_OP_EQUIVALENT = 9,
	VK_LOGIC_OP_INVERT = 10,
	VK_LOGIC_OP_OR_REVERSE = 11,
	VK_LOGIC_OP_COPY_INVERTED = 12,
	VK_LOGIC_OP_OR_INVERTED = 13,
	VK_LOGIC_OP_NAND = 14,
	VK_LOGIC_OP_SET = 15,
	//VK_LOGIC_OP_BEGIN_RANGE = VK_LOGIC_OP_CLEAR,
	//VK_LOGIC_OP_END_RANGE = VK_LOGIC_OP_SET,
	//VK_LOGIC_OP_RANGE_SIZE = (VK_LOGIC_OP_SET - VK_LOGIC_OP_CLEAR + 1),
	//VK_LOGIC_OP_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkBlendFactor {
	VK_BLEND_FACTOR_ZERO = 0,
	VK_BLEND_FACTOR_ONE = 1,
	VK_BLEND_FACTOR_SRC_COLOR = 2,
	VK_BLEND_FACTOR_ONE_MINUS_SRC_COLOR = 3,
	VK_BLEND_FACTOR_DST_COLOR = 4,
	VK_BLEND_FACTOR_ONE_MINUS_DST_COLOR = 5,
	VK_BLEND_FACTOR_SRC_ALPHA = 6,
	VK_BLEND_FACTOR_ONE_MINUS_SRC_ALPHA = 7,
	VK_BLEND_FACTOR_DST_ALPHA = 8,
	VK_BLEND_FACTOR_ONE_MINUS_DST_ALPHA = 9,
	VK_BLEND_FACTOR_CONSTANT_COLOR = 10,
	VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_COLOR = 11,
	VK_BLEND_FACTOR_CONSTANT_ALPHA = 12,
	VK_BLEND_FACTOR_ONE_MINUS_CONSTANT_ALPHA = 13,
	VK_BLEND_FACTOR_SRC_ALPHA_SATURATE = 14,
	VK_BLEND_FACTOR_SRC1_COLOR = 15,
	VK_BLEND_FACTOR_ONE_MINUS_SRC1_COLOR = 16,
	VK_BLEND_FACTOR_SRC1_ALPHA = 17,
	VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA = 18,
	//VK_BLEND_FACTOR_BEGIN_RANGE = VK_BLEND_FACTOR_ZERO,
	//VK_BLEND_FACTOR_END_RANGE = VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA,
	//VK_BLEND_FACTOR_RANGE_SIZE = (VK_BLEND_FACTOR_ONE_MINUS_SRC1_ALPHA - VK_BLEND_FACTOR_ZERO + 1),
	//VK_BLEND_FACTOR_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkBlendOp {
	VK_BLEND_OP_ADD = 0,
	VK_BLEND_OP_SUBTRACT = 1,
	VK_BLEND_OP_REVERSE_SUBTRACT = 2,
	VK_BLEND_OP_MIN = 3,
	VK_BLEND_OP_MAX = 4,
	//VK_BLEND_OP_BEGIN_RANGE = VK_BLEND_OP_ADD,
	//VK_BLEND_OP_END_RANGE = VK_BLEND_OP_MAX,
	//VK_BLEND_OP_RANGE_SIZE = (VK_BLEND_OP_MAX - VK_BLEND_OP_ADD + 1),
	//VK_BLEND_OP_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkDynamicState {
	VK_DYNAMIC_STATE_VIEWPORT = 0,
	VK_DYNAMIC_STATE_SCISSOR = 1,
	VK_DYNAMIC_STATE_LINE_WIDTH = 2,
	VK_DYNAMIC_STATE_DEPTH_BIAS = 3,
	VK_DYNAMIC_STATE_BLEND_CONSTANTS = 4,
	VK_DYNAMIC_STATE_DEPTH_BOUNDS = 5,
	VK_DYNAMIC_STATE_STENCIL_COMPARE_MASK = 6,
	VK_DYNAMIC_STATE_STENCIL_WRITE_MASK = 7,
	VK_DYNAMIC_STATE_STENCIL_REFERENCE = 8,
	//VK_DYNAMIC_STATE_BEGIN_RANGE = VK_DYNAMIC_STATE_VIEWPORT,
	//VK_DYNAMIC_STATE_END_RANGE = VK_DYNAMIC_STATE_STENCIL_REFERENCE,
	//VK_DYNAMIC_STATE_RANGE_SIZE = (VK_DYNAMIC_STATE_STENCIL_REFERENCE - VK_DYNAMIC_STATE_VIEWPORT + 1),
	//VK_DYNAMIC_STATE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkFilter {
	VK_FILTER_NEAREST = 0,
	VK_FILTER_LINEAR = 1,
	//VK_FILTER_BEGIN_RANGE = VK_FILTER_NEAREST,
	//VK_FILTER_END_RANGE = VK_FILTER_LINEAR,
	//VK_FILTER_RANGE_SIZE = (VK_FILTER_LINEAR - VK_FILTER_NEAREST + 1),
	//VK_FILTER_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSamplerMipmapMode {
	VK_SAMPLER_MIPMAP_MODE_NEAREST = 0,
	VK_SAMPLER_MIPMAP_MODE_LINEAR = 1,
	//VK_SAMPLER_MIPMAP_MODE_BEGIN_RANGE = VK_SAMPLER_MIPMAP_MODE_NEAREST,
	//VK_SAMPLER_MIPMAP_MODE_END_RANGE = VK_SAMPLER_MIPMAP_MODE_LINEAR,
	//VK_SAMPLER_MIPMAP_MODE_RANGE_SIZE = (VK_SAMPLER_MIPMAP_MODE_LINEAR - VK_SAMPLER_MIPMAP_MODE_NEAREST + 1),
	//VK_SAMPLER_MIPMAP_MODE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSamplerAddressMode {
	VK_SAMPLER_ADDRESS_MODE_REPEAT = 0,
	VK_SAMPLER_ADDRESS_MODE_MIRRORED_REPEAT = 1,
	VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_EDGE = 2,
	VK_SAMPLER_ADDRESS_MODE_CLAMP_TO_BORDER = 3,
	VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE = 4,
	//VK_SAMPLER_ADDRESS_MODE_BEGIN_RANGE = VK_SAMPLER_ADDRESS_MODE_REPEAT,
	//VK_SAMPLER_ADDRESS_MODE_END_RANGE = VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE,
	//VK_SAMPLER_ADDRESS_MODE_RANGE_SIZE = (VK_SAMPLER_ADDRESS_MODE_MIRROR_CLAMP_TO_EDGE - VK_SAMPLER_ADDRESS_MODE_REPEAT + 1),
	//VK_SAMPLER_ADDRESS_MODE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkBorderColor {
	VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK = 0,
	VK_BORDER_COLOR_INT_TRANSPARENT_BLACK = 1,
	VK_BORDER_COLOR_FLOAT_OPAQUE_BLACK = 2,
	VK_BORDER_COLOR_INT_OPAQUE_BLACK = 3,
	VK_BORDER_COLOR_FLOAT_OPAQUE_WHITE = 4,
	VK_BORDER_COLOR_INT_OPAQUE_WHITE = 5,
	//VK_BORDER_COLOR_BEGIN_RANGE = VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK,
	//VK_BORDER_COLOR_END_RANGE = VK_BORDER_COLOR_INT_OPAQUE_WHITE,
	//VK_BORDER_COLOR_RANGE_SIZE = (VK_BORDER_COLOR_INT_OPAQUE_WHITE - VK_BORDER_COLOR_FLOAT_TRANSPARENT_BLACK + 1),
	//VK_BORDER_COLOR_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkDescriptorType {
	VK_DESCRIPTOR_TYPE_SAMPLER = 0,
	VK_DESCRIPTOR_TYPE_COMBINED_IMAGE_SAMPLER = 1,
	VK_DESCRIPTOR_TYPE_SAMPLED_IMAGE = 2,
	VK_DESCRIPTOR_TYPE_STORAGE_IMAGE = 3,
	VK_DESCRIPTOR_TYPE_UNIFORM_TEXEL_BUFFER = 4,
	VK_DESCRIPTOR_TYPE_STORAGE_TEXEL_BUFFER = 5,
	VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER = 6,
	VK_DESCRIPTOR_TYPE_STORAGE_BUFFER = 7,
	VK_DESCRIPTOR_TYPE_UNIFORM_BUFFER_DYNAMIC = 8,
	VK_DESCRIPTOR_TYPE_STORAGE_BUFFER_DYNAMIC = 9,
	VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT = 10,
	//VK_DESCRIPTOR_TYPE_BEGIN_RANGE = VK_DESCRIPTOR_TYPE_SAMPLER,
	//VK_DESCRIPTOR_TYPE_END_RANGE = VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT,
	//VK_DESCRIPTOR_TYPE_RANGE_SIZE = (VK_DESCRIPTOR_TYPE_INPUT_ATTACHMENT - VK_DESCRIPTOR_TYPE_SAMPLER + 1),
	//VK_DESCRIPTOR_TYPE_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkAttachmentLoadOp {
	VK_ATTACHMENT_LOAD_OP_LOAD = 0,
	VK_ATTACHMENT_LOAD_OP_CLEAR = 1,
	VK_ATTACHMENT_LOAD_OP_DONT_CARE = 2,
	//VK_ATTACHMENT_LOAD_OP_BEGIN_RANGE = VK_ATTACHMENT_LOAD_OP_LOAD,
	//VK_ATTACHMENT_LOAD_OP_END_RANGE = VK_ATTACHMENT_LOAD_OP_DONT_CARE,
	//VK_ATTACHMENT_LOAD_OP_RANGE_SIZE = (VK_ATTACHMENT_LOAD_OP_DONT_CARE - VK_ATTACHMENT_LOAD_OP_LOAD + 1),
	//VK_ATTACHMENT_LOAD_OP_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkAttachmentStoreOp {
	VK_ATTACHMENT_STORE_OP_STORE = 0,
	VK_ATTACHMENT_STORE_OP_DONT_CARE = 1,
	//VK_ATTACHMENT_STORE_OP_BEGIN_RANGE = VK_ATTACHMENT_STORE_OP_STORE,
	//VK_ATTACHMENT_STORE_OP_END_RANGE = VK_ATTACHMENT_STORE_OP_DONT_CARE,
	//VK_ATTACHMENT_STORE_OP_RANGE_SIZE = (VK_ATTACHMENT_STORE_OP_DONT_CARE - VK_ATTACHMENT_STORE_OP_STORE + 1),
	//VK_ATTACHMENT_STORE_OP_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkPipelineBindPoint {
	VK_PIPELINE_BIND_POINT_GRAPHICS = 0,
	VK_PIPELINE_BIND_POINT_COMPUTE = 1,
	//VK_PIPELINE_BIND_POINT_BEGIN_RANGE = VK_PIPELINE_BIND_POINT_GRAPHICS,
	//VK_PIPELINE_BIND_POINT_END_RANGE = VK_PIPELINE_BIND_POINT_COMPUTE,
	//VK_PIPELINE_BIND_POINT_RANGE_SIZE = (VK_PIPELINE_BIND_POINT_COMPUTE - VK_PIPELINE_BIND_POINT_GRAPHICS + 1),
	//VK_PIPELINE_BIND_POINT_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkCommandBufferLevel {
	VK_COMMAND_BUFFER_LEVEL_PRIMARY = 0,
	VK_COMMAND_BUFFER_LEVEL_SECONDARY = 1,
	//VK_COMMAND_BUFFER_LEVEL_BEGIN_RANGE = VK_COMMAND_BUFFER_LEVEL_PRIMARY,
	//VK_COMMAND_BUFFER_LEVEL_END_RANGE = VK_COMMAND_BUFFER_LEVEL_SECONDARY,
	//VK_COMMAND_BUFFER_LEVEL_RANGE_SIZE = (VK_COMMAND_BUFFER_LEVEL_SECONDARY - VK_COMMAND_BUFFER_LEVEL_PRIMARY + 1),
	//VK_COMMAND_BUFFER_LEVEL_MAX_ENUM = 0x7FFFFFFF
}

#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkIndexType {
	VK_INDEX_TYPE_UINT16 = 0,
	VK_INDEX_TYPE_UINT32 = 1,
	//VK_INDEX_TYPE_BEGIN_RANGE = VK_INDEX_TYPE_UINT16,
	//VK_INDEX_TYPE_END_RANGE = VK_INDEX_TYPE_UINT32,
	//VK_INDEX_TYPE_RANGE_SIZE = (VK_INDEX_TYPE_UINT32 - VK_INDEX_TYPE_UINT16 + 1),
	//VK_INDEX_TYPE_MAX_ENUM = 0x7FFFFFFF
}


#[repr(C)]
#[allow(dead_code)]
#[allow(non_camel_case_types)]
pub enum VkSubpassContents {
	VK_SUBPASS_CONTENTS_INLINE = 0,
	VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS = 1,
	//VK_SUBPASS_CONTENTS_BEGIN_RANGE = VK_SUBPASS_CONTENTS_INLINE,
	//VK_SUBPASS_CONTENTS_END_RANGE = VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS,
	//VK_SUBPASS_CONTENTS_RANGE_SIZE = (VK_SUBPASS_CONTENTS_SECONDARY_COMMAND_BUFFERS - VK_SUBPASS_CONTENTS_INLINE + 1),
	//VK_SUBPASS_CONTENTS_MAX_ENUM = 0x7FFFFFFF
}