extern crate libc;

use super::types::*;
use super::consts::*;
use super::enums::*;
use super::flags::*;

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkApplicationInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub pApplicationName: *const libc::c_uchar, // *const [libc::c_uchar],
	pub applicationVersion: u32,
	pub pEngineName: *const libc::c_uchar, // *const [libc::c_uchar],
	pub engineVersion: u32,
	pub apiVersion: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkInstanceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void, //::std::any::Any, // libc::c_void,
	pub flags: VkInstanceCreateFlags,
	pub pApplicationInfo: *const VkApplicationInfo,
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const libc::c_uchar, // *const [*const [libc::c_uchar]],
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const libc::c_uchar, // *const [*const [libc::c_uchar]],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkAllocationCallbacks {
	pub pUserData: *const libc::c_void,
	//PFN_vkAllocationFunction pfnAllocation,
// TODO	pub pfnAllocation: PFN_vkAllocationFunction,
	//PFN_vkReallocationFunction pfnReallocation,
// TODO	pub pfnReallocation: PFN_vkReallocationFunction,
	//PFN_vkFreeFunction pfnFree,
// TODO	pub pfnFree: PFN_vkFreeFunction,
	//PFN_vkInternalAllocationNotification pfnInternalAllocation,
// TODO	pub pfnInternalAllocation: PFN_vkInternalAllocationNotification,
	//PFN_vkInternalFreeNotification pfnInternalFree,
// TODO	pub pfnInternalFree: PFN_vkInternalFreeNotification,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkPhysicalDeviceFeatures {
	pub robustBufferAccess: VkBool32,
	pub fullDrawIndexUint32: VkBool32,
	pub imageCubeArray: VkBool32,
	pub independentBlend: VkBool32,
	pub geometryShader: VkBool32,
	pub tessellationShader: VkBool32,
	pub sampleRateShading: VkBool32,
	pub dualSrcBlend: VkBool32,
	pub logicOp: VkBool32,
	pub multiDrawIndirect: VkBool32,
	pub drawIndirectFirstInstance: VkBool32,
	pub depthClamp: VkBool32,
	pub depthBiasClamp: VkBool32,
	pub fillModeNonSolid: VkBool32,
	pub depthBounds: VkBool32,
	pub wideLines: VkBool32,
	pub largePoints: VkBool32,
	pub alphaToOne: VkBool32,
	pub multiViewport: VkBool32,
	pub samplerAnisotropy: VkBool32,
	pub textureCompressionETC2: VkBool32,
	pub textureCompressionASTC_LDR: VkBool32,
	pub textureCompressionBC: VkBool32,
	pub occlusionQueryPrecise: VkBool32,
	pub pipelineStatisticsQuery: VkBool32,
	pub vertexPipelineStoresAndAtomics: VkBool32,
	pub fragmentStoresAndAtomics: VkBool32,
	pub shaderTessellationAndGeometryPointSize: VkBool32,
	pub shaderImageGatherExtended: VkBool32,
	pub shaderStorageImageExtendedFormats: VkBool32,
	pub shaderStorageImageMultisample: VkBool32,
	pub shaderStorageImageReadWithoutFormat: VkBool32,
	pub shaderStorageImageWriteWithoutFormat: VkBool32,
	pub shaderUniformBufferArrayDynamicIndexing: VkBool32,
	pub shaderSampledImageArrayDynamicIndexing: VkBool32,
	pub shaderStorageBufferArrayDynamicIndexing: VkBool32,
	pub shaderStorageImageArrayDynamicIndexing: VkBool32,
	pub shaderClipDistance: VkBool32,
	pub shaderCullDistance: VkBool32,
	pub shaderFloat64: VkBool32,
	pub shaderInt64: VkBool32,
	pub shaderInt16: VkBool32,
	pub shaderResourceResidency: VkBool32,
	pub shaderResourceMinLod: VkBool32,
	pub sparseBinding: VkBool32,
	pub sparseResidencyBuffer: VkBool32,
	pub sparseResidencyImage2D: VkBool32,
	pub sparseResidencyImage3D: VkBool32,
	pub sparseResidency2Samples: VkBool32,
	pub sparseResidency4Samples: VkBool32,
	pub sparseResidency8Samples: VkBool32,
	pub sparseResidency16Samples: VkBool32,
	pub sparseResidencyAliased: VkBool32,
	pub variableMultisampleRate: VkBool32,
	pub inheritedQueries: VkBool32,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkFormatProperties {
	pub linearTilingFeatures: VkFormatFeatureFlags,
	pub optimalTilingFeatures: VkFormatFeatureFlags,
	pub bufferFeatures: VkFormatFeatureFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, Default)]
pub struct VkExtent3D {
	pub width: u32,
	pub height: u32,
	pub depth: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkImageFormatProperties {
	pub maxExtent: VkExtent3D,
	pub maxMipLevels: u32,
	pub maxArrayLayers: u32,
	pub sampleCounts: VkSampleCountFlags,
	pub maxResourceSize: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkPhysicalDeviceLimits {
	pub maxImageDimension1D: u32,
	pub maxImageDimension2D: u32,
	pub maxImageDimension3D: u32,
	pub maxImageDimensionCube: u32,
	pub maxImageArrayLayers: u32,
	pub maxTexelBufferElements: u32,
	pub maxUniformBufferRange: u32,
	pub maxStorageBufferRange: u32,
	pub maxPushConstantsSize: u32,
	pub maxMemoryAllocationCount: u32,
	pub maxSamplerAllocationCount: u32,
	pub bufferImageGranularity: VkDeviceSize,
	pub sparseAddressSpaceSize: VkDeviceSize,
	pub maxBoundDescriptorSets: u32,
	pub maxPerStageDescriptorSamplers: u32,
	pub maxPerStageDescriptorUniformBuffers: u32,
	pub maxPerStageDescriptorStorageBuffers: u32,
	pub maxPerStageDescriptorSampledImages: u32,
	pub maxPerStageDescriptorStorageImages: u32,
	pub maxPerStageDescriptorInputAttachments: u32,
	pub maxPerStageResources: u32,
	pub maxDescriptorSetSamplers: u32,
	pub maxDescriptorSetUniformBuffers: u32,
	pub maxDescriptorSetUniformBuffersDynamic: u32,
	pub maxDescriptorSetStorageBuffers: u32,
	pub maxDescriptorSetStorageBuffersDynamic: u32,
	pub maxDescriptorSetSampledImages: u32,
	pub maxDescriptorSetStorageImages: u32,
	pub maxDescriptorSetInputAttachments: u32,
	pub maxVertexInputAttributes: u32,
	pub maxVertexInputBindings: u32,
	pub maxVertexInputAttributeOffset: u32,
	pub maxVertexInputBindingStride: u32,
	pub maxVertexOutputComponents: u32,
	pub maxTessellationGenerationLevel: u32,
	pub maxTessellationPatchSize: u32,
	pub maxTessellationControlPerVertexInputComponents: u32,
	pub maxTessellationControlPerVertexOutputComponents: u32,
	pub maxTessellationControlPerPatchOutputComponents: u32,
	pub maxTessellationControlTotalOutputComponents: u32,
	pub maxTessellationEvaluationInputComponents: u32,
	pub maxTessellationEvaluationOutputComponents: u32,
	pub maxGeometryShaderInvocations: u32,
	pub maxGeometryInputComponents: u32,
	pub maxGeometryOutputComponents: u32,
	pub maxGeometryOutputVertices: u32,
	pub maxGeometryTotalOutputComponents: u32,
	pub maxFragmentInputComponents: u32,
	pub maxFragmentOutputAttachments: u32,
	pub maxFragmentDualSrcAttachments: u32,
	pub maxFragmentCombinedOutputResources: u32,
	pub maxComputeSharedMemorySize: u32,
	pub maxComputeWorkGroupCount: [u32; 3],
	pub maxComputeWorkGroupInvocations: u32,
	pub maxComputeWorkGroupSize: [u32; 3],
	pub subPixelPrecisionBits: u32,
	pub subTexelPrecisionBits: u32,
	pub mipmapPrecisionBits: u32,
	pub maxDrawIndexedIndexValue: u32,
	pub maxDrawIndirectCount: u32,
	pub maxSamplerLodBias: f32,
	pub maxSamplerAnisotropy: f32,
	pub maxViewports: u32,
	pub maxViewportDimensions: [u32; 2],
	pub viewportBoundsRange: [f32; 2],
	pub viewportSubPixelBits: u32,
	pub minMemoryMapAlignment: usize,
	pub minTexelBufferOffsetAlignment: VkDeviceSize,
	pub minUniformBufferOffsetAlignment: VkDeviceSize,
	pub minStorageBufferOffsetAlignment: VkDeviceSize,
	pub minTexelOffset: i32,
	pub maxTexelOffset: u32,
	pub minTexelGatherOffset: i32,
	pub maxTexelGatherOffset: u32,
	pub minInterpolationOffset: f32,
	pub maxInterpolationOffset: f32,
	pub subPixelInterpolationOffsetBits: u32,
	pub maxFramebufferWidth: u32,
	pub maxFramebufferHeight: u32,
	pub maxFramebufferLayers: u32,
	pub framebufferColorSampleCounts: VkSampleCountFlags,
	pub framebufferDepthSampleCounts: VkSampleCountFlags,
	pub framebufferStencilSampleCounts: VkSampleCountFlags,
	pub framebufferNoAttachmentsSampleCounts: VkSampleCountFlags,
	pub maxColorAttachments: u32,
	pub sampledImageColorSampleCounts: VkSampleCountFlags,
	pub sampledImageIntegerSampleCounts: VkSampleCountFlags,
	pub sampledImageDepthSampleCounts: VkSampleCountFlags,
	pub sampledImageStencilSampleCounts: VkSampleCountFlags,
	pub storageImageSampleCounts: VkSampleCountFlags,
	pub maxSampleMaskWords: u32,
	pub timestampComputeAndGraphics: VkBool32,
	pub timestampPeriod: f32,
	pub maxClipDistances: u32,
	pub maxCullDistances: u32,
	pub maxCombinedClipAndCullDistances: u32,
	pub discreteQueuePriorities: u32,
	pub pointSizeRange: [f32; 2],
	pub lineWidthRange: [f32; 2],
	pub pointSizeGranularity: f32,
	pub lineWidthGranularity: f32,
	pub strictLines: VkBool32,
	pub standardSampleLocations: VkBool32,
	pub optimalBufferCopyOffsetAlignment: VkDeviceSize,
	pub optimalBufferCopyRowPitchAlignment: VkDeviceSize,
	pub nonCoherentAtomSize: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkPhysicalDeviceSparseProperties {
	pub residencyStandard2DBlockShape: VkBool32,
	pub residencyStandard2DMultisampleBlockShape: VkBool32,
	pub residencyStandard3DBlockShape: VkBool32,
	pub residencyAlignedMipSize: VkBool32,
	pub residencyNonResidentStrict: VkBool32,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkPhysicalDeviceProperties {
	pub apiVersion: u32,
	pub driverVersion: u32,
	pub vendorID: u32,
	pub deviceID: u32,
	pub deviceType: VkPhysicalDeviceType,
	pub deviceName: VkPhysicalDeviceProperties_DeviceNameSlice, //[libc::c_uchar; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	pub pipelineCacheUUID: [u8; VK_UUID_SIZE],
	pub limits: VkPhysicalDeviceLimits,
	pub sparseProperties: VkPhysicalDeviceSparseProperties,
}

#[repr(C)]
pub struct VkPhysicalDeviceProperties_DeviceNameSlice(pub [libc::c_uchar; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]);

impl Default for VkPhysicalDeviceProperties_DeviceNameSlice {
    fn default() -> Self { VkPhysicalDeviceProperties_DeviceNameSlice([libc::c_uchar::default(); VK_MAX_PHYSICAL_DEVICE_NAME_SIZE]) }
}


#[repr(C)]
#[allow(non_snake_case)]
#[derive(Clone, Default)]
pub struct VkQueueFamilyProperties {
	pub queueFlags: VkQueueFlags,
	pub queueCount: u32,
	pub timestampValidBits: u32,
	pub minImageTransferGranularity: VkExtent3D,
}
	
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Copy)]
pub struct VkMemoryType {
	pub propertyFlags: VkMemoryPropertyFlags,
	pub heapIndex: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default, Debug, Clone, Copy)]
pub struct VkMemoryHeap {
	pub size: VkDeviceSize,
	pub flags: VkMemoryHeapFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default)]
pub struct VkPhysicalDeviceMemoryProperties {
	pub memoryTypeCount: u32,
	pub memoryTypes: VkPhysicalDeviceMemoryProperties_MemoryTypesSlice, //[VkMemoryType; VK_MAX_MEMORY_TYPES],
	pub memoryHeapCount: u32,
	pub memoryHeaps: VkPhysicalDeviceMemoryProperties_MemoryHeapsSlice, //[VkMemoryHeap; VK_MAX_MEMORY_HEAPS],
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties_MemoryTypesSlice(pub [VkMemoryType; VK_MAX_MEMORY_TYPES]);

impl Default for VkPhysicalDeviceMemoryProperties_MemoryTypesSlice {
    fn default() -> Self { VkPhysicalDeviceMemoryProperties_MemoryTypesSlice([VkMemoryType::default(); VK_MAX_MEMORY_TYPES]) }
}

#[repr(C)]
pub struct VkPhysicalDeviceMemoryProperties_MemoryHeapsSlice(pub [VkMemoryHeap; VK_MAX_MEMORY_HEAPS]);

impl Default for VkPhysicalDeviceMemoryProperties_MemoryHeapsSlice {
    fn default() -> Self { VkPhysicalDeviceMemoryProperties_MemoryHeapsSlice([VkMemoryHeap::default(); VK_MAX_MEMORY_HEAPS]) }
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDeviceQueueCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkDeviceQueueCreateFlags,
	pub queueFamilyIndex: u32,
	pub queueCount: u32,
	pub pQueuePriorities: *const f32, // *const [f32],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDeviceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkDeviceCreateFlags,
	pub queueCreateInfoCount: u32,
	pub pQueueCreateInfos: *const VkDeviceQueueCreateInfo, // *const [VkDeviceQueueCreateInfo],
	pub enabledLayerCount: u32,
	pub ppEnabledLayerNames: *const *const libc::c_uchar, // *const [*const [libc::c_uchar]],
	pub enabledExtensionCount: u32,
	pub ppEnabledExtensionNames: *const *const libc::c_uchar, // *const [*const [libc::c_uchar]],
	pub pEnabledFeatures: *const VkPhysicalDeviceFeatures,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default, Clone)]
pub struct VkExtensionProperties {
	pub extensionName: VkExtensionProperties_ExtensionNameSlice, //[libc::c_uchar; VK_MAX_EXTENSION_NAME_SIZE],
	pub specVersion: u32,
}

#[repr(C)]
pub struct VkExtensionProperties_ExtensionNameSlice(pub [libc::c_uchar; VK_MAX_EXTENSION_NAME_SIZE]);

impl Default for VkExtensionProperties_ExtensionNameSlice {
    fn default() -> Self { VkExtensionProperties_ExtensionNameSlice([libc::c_uchar::default(); VK_MAX_EXTENSION_NAME_SIZE]) }
}

// TODO isn't there an easier way?
impl Clone for VkExtensionProperties_ExtensionNameSlice {
    fn clone(&self) -> Self {
    	//*self
    	let mut result = Self::default().0;
    	result.clone_from_slice(&self.0);
    	VkExtensionProperties_ExtensionNameSlice(result)
    }
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default, Clone)]
pub struct VkLayerProperties {
	pub layerName: VkLayerProperties_LayerNameSlice, //[libc::c_uchar; VK_MAX_EXTENSION_NAME_SIZE],
	pub specVersion: u32,
	pub implementationVersion: u32,
	pub description: VkLayerProperties_DescriptionSlice, //[libc::c_uchar; VK_MAX_DESCRIPTION_SIZE],
}

#[repr(C)]
pub struct VkLayerProperties_LayerNameSlice(pub [libc::c_uchar; VK_MAX_EXTENSION_NAME_SIZE]);

impl Default for VkLayerProperties_LayerNameSlice {
    fn default() -> Self { VkLayerProperties_LayerNameSlice([libc::c_uchar::default(); VK_MAX_EXTENSION_NAME_SIZE]) }
}

// TODO isn't there an easier way?
impl Clone for VkLayerProperties_LayerNameSlice {
    fn clone(&self) -> Self {
    	//*self
    	let mut result = [libc::c_uchar::default(); VK_MAX_EXTENSION_NAME_SIZE];
    	result.clone_from_slice(&self.0);
    	VkLayerProperties_LayerNameSlice(result)
    }
}

#[repr(C)]
pub struct VkLayerProperties_DescriptionSlice(pub [libc::c_uchar; VK_MAX_DESCRIPTION_SIZE]);

impl Default for VkLayerProperties_DescriptionSlice {
    fn default() -> Self { VkLayerProperties_DescriptionSlice([libc::c_uchar::default(); VK_MAX_DESCRIPTION_SIZE]) }
}

// TODO isn't there an easier way?
impl Clone for VkLayerProperties_DescriptionSlice {
    fn clone(&self) -> Self {
    	let mut result = [libc::c_uchar::default(); VK_MAX_EXTENSION_NAME_SIZE];
    	result.clone_from_slice(&self.0);
    	VkLayerProperties_DescriptionSlice(result)
    }
}



#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSubmitInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore, // *const [VkSemaphore],
	pub pWaitDstStageMask: *const VkPipelineStageFlags,
	pub commandBufferCount: u32,
	pub pCommandBuffers: *const VkCommandBuffer, // *const [VkCommandBuffer],
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: *const VkSemaphore, // *const [VkSemaphore],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkMemoryAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub allocationSize: VkDeviceSize,
	pub memoryTypeIndex: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkMappedMemoryRange {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub memory: VkDeviceMemory,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkMemoryRequirements {
	pub size: VkDeviceSize,
	pub alignment: VkDeviceSize,
	pub memoryTypeBits: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
#[derive(Default, Clone)]
pub struct VkSparseImageFormatProperties {
	pub aspectMask: VkImageAspectFlags,
	pub imageGranularity: VkExtent3D,
	pub flags: VkSparseImageFormatFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSparseImageMemoryRequirements {
	pub formatProperties: VkSparseImageFormatProperties,
	pub imageMipTailFirstLod: u32,
	pub imageMipTailSize: VkDeviceSize,
	pub imageMipTailOffset: VkDeviceSize,
	pub imageMipTailStride: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSparseMemoryBind {
	pub resourceOffset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSparseBufferMemoryBindInfo {
	pub buffer: VkBuffer,
	pub bindCount: u32,
	pub pBinds: *const VkSparseMemoryBind, // *const [VkSparseMemoryBind],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSparseImageOpaqueMemoryBindInfo {
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: *const VkSparseMemoryBind, // *const [VkSparseMemoryBind],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageSubresource {
	pub aspectMask: VkImageAspectFlags,
	pub mipLevel: u32,
	pub arrayLayer: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkOffset3D {
	pub x: i32,
	pub y: i32,
	pub z: i32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSparseImageMemoryBind {
	pub subresource: VkImageSubresource,
	pub offset: VkOffset3D,
	pub extent: VkExtent3D,
	pub memory: VkDeviceMemory,
	pub memoryOffset: VkDeviceSize,
	pub flags: VkSparseMemoryBindFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSparseImageMemoryBindInfo {
	pub image: VkImage,
	pub bindCount: u32,
	pub pBinds: *const VkSparseImageMemoryBind, // *const [VkSparseImageMemoryBind],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkBindSparseInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub waitSemaphoreCount: u32,
	pub pWaitSemaphores: *const VkSemaphore, // *const [VkSemaphore],
	pub bufferBindCount: u32,
	pub pBufferBinds: *const VkSparseBufferMemoryBindInfo, // *const [VkSparseBufferMemoryBindInfo],
	pub imageOpaqueBindCount: u32,
	pub pImageOpaqueBinds: *const VkSparseImageOpaqueMemoryBindInfo, // *const [VkSparseImageOpaqueMemoryBindInfo],
	pub imageBindCount: u32,
	pub pImageBinds: *const VkSparseImageMemoryBindInfo, // *const [VkSparseImageMemoryBindInfo],
	pub signalSemaphoreCount: u32,
	pub pSignalSemaphores: *const VkSemaphore, // *const [VkSemaphore],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkFenceCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkFenceCreateFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSemaphoreCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkSemaphoreCreateFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkEventCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkEventCreateFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkQueryPoolCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkQueryPoolCreateFlags,
	pub queryType: VkQueryType,
	pub queryCount: u32,
	pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkBufferCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkBufferCreateFlags,
	pub size: VkDeviceSize,
	pub usage: VkBufferUsageFlags,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32, // *const [u32],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkBufferViewCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkBufferViewCreateFlags,
	pub buffer: VkBuffer,
	pub format: VkFormat,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkImageCreateFlags,
	pub imageType: VkImageType,
	pub format: VkFormat,
	pub extent: VkExtent3D,
	pub mipLevels: u32,
	pub arrayLayers: u32,
	pub samples: VkSampleCountFlags,
	pub tiling: VkImageTiling,
	pub usage: VkImageUsageFlags,
	pub sharingMode: VkSharingMode,
	pub queueFamilyIndexCount: u32,
	pub pQueueFamilyIndices: *const u32, // *const [u32],
	pub initialLayout: VkImageLayout,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSubresourceLayout {
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
	pub rowPitch: VkDeviceSize,
	pub arrayPitch: VkDeviceSize,
	pub depthPitch: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkComponentMapping {
	pub r: VkComponentSwizzle,
	pub g: VkComponentSwizzle,
	pub b: VkComponentSwizzle,
	pub a: VkComponentSwizzle,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageSubresourceRange {
	pub aspectMask: VkImageAspectFlags,
	pub baseMipLevel: u32,
	pub levelCount: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageViewCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkImageViewCreateFlags,
	pub image: VkImage,
	pub viewType: VkImageViewType,
	pub format: VkFormat,
	pub components: VkComponentMapping,
	pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkShaderModuleCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkShaderModuleCreateFlags,
	pub codeSize: usize,
	pub pCode: *const u32, // *const [u32],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineCacheCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineCacheCreateFlags,
	pub initialDataSize: usize,
	pub pInitialData: *const libc::c_void,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSpecializationMapEntry {
	pub constantID: u32,
	pub offset: u32,
	pub size: usize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSpecializationInfo {
	pub mapEntryCount: u32,
	pub pMapEntries: *const VkSpecializationMapEntry, // *const [VkSpecializationMapEntry],
	pub dataSize: usize,
	pub pData: *const libc::c_void,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineShaderStageCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineShaderStageCreateFlags,
	pub stage: VkShaderStageFlagBits,
	pub module: VkShaderModule,
	pub pName: *const libc::c_uchar, // *const [libc::c_uchar],
	pub pSpecializationInfo: *const VkSpecializationInfo,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkVertexInputBindingDescription {
	pub binding: u32,
	pub stride: u32,
	pub inputRate: VkVertexInputRate,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkVertexInputAttributeDescription {
	pub location: u32,
	pub binding: u32,
	pub format: VkFormat,
	pub offset: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineVertexInputStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineVertexInputStateCreateFlags,
	pub vertexBindingDescriptionCount: u32,
	pub pVertexBindingDescriptions: *const VkVertexInputBindingDescription, // *const [VkVertexInputBindingDescription],
	pub vertexAttributeDescriptionCount: u32,
	pub pVertexAttributeDescriptions: *const VkVertexInputAttributeDescription, // *const [VkVertexInputAttributeDescription],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineInputAssemblyStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineInputAssemblyStateCreateFlags,
	pub topology: VkPrimitiveTopology,
	pub primitiveRestartEnable: VkBool32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineTessellationStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineTessellationStateCreateFlags,
	pub patchControlPoints: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkViewport {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
	pub minDepth: f32,
	pub maxDepth: f32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkOffset2D {
	pub x: i32,
	pub y: i32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkExtent2D {
	pub width: u32,
	pub height: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkRect2D {
	pub offset: VkOffset2D,
	pub extent: VkExtent2D,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineViewportStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineViewportStateCreateFlags,
	pub viewportCount: u32,
	pub pViewports: *const VkViewport, // *const [VkViewport],
	pub scissorCount: u32,
	pub pScissors: *const VkRect2D, // *const [VkRect2D],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineRasterizationStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineRasterizationStateCreateFlags,
	pub depthClampEnable: VkBool32,
	pub rasterizerDiscardEnable: VkBool32,
	pub polygonMode: VkPolygonMode,
	pub cullMode: VkCullModeFlags,
	pub frontFace: VkFrontFace,
	pub depthBiasEnable: VkBool32,
	pub depthBiasConstantFactor: f32,
	pub depthBiasClamp: f32,
	pub depthBiasSlopeFactor: f32,
	pub lineWidth: f32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineMultisampleStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineMultisampleStateCreateFlags,
	pub rasterizationSamples: VkSampleCountFlags,
	pub sampleShadingEnable: VkBool32,
	pub minSampleShading: f32,
	pub pSampleMask: *const VkSampleMask,
	pub alphaToCoverageEnable: VkBool32,
	pub alphaToOneEnable: VkBool32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkStencilOpState {
	pub failOp: VkStencilOp,
	pub passOp: VkStencilOp,
	pub depthFailOp: VkStencilOp,
	pub compareOp: VkCompareOp,
	pub compareMask: u32,
	pub writeMask: u32,
	pub reference: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineDepthStencilStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineDepthStencilStateCreateFlags,
	pub depthTestEnable :VkBool32,
	pub depthWriteEnable: VkBool32,
	pub depthCompareOp: VkCompareOp,
	pub depthBoundsTestEnable: VkBool32,
	pub stencilTestEnable: VkBool32,
	pub front: VkStencilOpState,
	pub back: VkStencilOpState,
	pub minDepthBounds: f32,
	pub maxDepthBounds: f32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineColorBlendAttachmentState {
	pub blendEnable: VkBool32,
	pub srcColorBlendFactor: VkBlendFactor,
	pub dstColorBlendFactor: VkBlendFactor,
	pub colorBlendOp: VkBlendOp,
	pub srcAlphaBlendFactor: VkBlendFactor,
	pub dstAlphaBlendFactor: VkBlendFactor,
	pub alphaBlendOp: VkBlendOp,
	pub colorWriteMask: VkColorComponentFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineColorBlendStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineColorBlendStateCreateFlags,
	pub logicOpEnable: VkBool32,
	pub logicOp: VkLogicOp,
	pub attachmentCount: u32,
	pub pAttachments: *const VkPipelineColorBlendAttachmentState, // *const [VkPipelineColorBlendAttachmentState],
	pub blendConstants: [f32; 4],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineDynamicStateCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineDynamicStateCreateFlags,
	pub dynamicStateCount: u32,
	pub pDynamicStates: *const VkDynamicState, // *const [VkDynamicState],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkGraphicsPipelineCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineCreateFlags,
	pub stageCount: u32,
	pub pStages: *const VkPipelineShaderStageCreateInfo, // *const [VkPipelineShaderStageCreateInfo],
	pub pVertexInputState: *const VkPipelineVertexInputStateCreateInfo,
	pub pInputAssemblyState: *const VkPipelineInputAssemblyStateCreateInfo,
	pub pTessellationState: *const VkPipelineTessellationStateCreateInfo,
	pub pViewportState: *const VkPipelineViewportStateCreateInfo,
	pub pRasterizationState: *const VkPipelineRasterizationStateCreateInfo,
	pub pMultisampleState: *const VkPipelineMultisampleStateCreateInfo,
	pub pDepthStencilState: *const VkPipelineDepthStencilStateCreateInfo,
	pub pColorBlendState: *const VkPipelineColorBlendStateCreateInfo,
	pub pDynamicState: *const VkPipelineDynamicStateCreateInfo,
	pub layout: VkPipelineLayout,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkComputePipelineCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineCreateFlags,
	pub stage: VkPipelineShaderStageCreateInfo,
	pub layout: VkPipelineLayout,
	pub basePipelineHandle: VkPipeline,
	pub basePipelineIndex: i32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPushConstantRange {
	pub stageFlags: VkShaderStageFlags,
	pub offset: u32,
	pub size: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkPipelineLayoutCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkPipelineLayoutCreateFlags,
	pub setLayoutCount: u32,
	pub pSetLayouts: *const VkDescriptorSetLayout, // *const [VkDescriptorSetLayout],
	pub pushConstantRangeCount: u32,
	pub pPushConstantRanges: *const VkPushConstantRange, // *const [VkPushConstantRange],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSamplerCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkSamplerCreateFlags,
	pub magFilter: VkFilter,
	pub minFilter: VkFilter,
	pub mipmapMode: VkSamplerMipmapMode,
	pub addressModeU: VkSamplerAddressMode,
	pub addressModeV: VkSamplerAddressMode,
	pub addressModeW: VkSamplerAddressMode,
	pub mipLodBias: f32,
	pub anisotropyEnable: VkBool32,
	pub maxAnisotropy: f32,
	pub compareEnable: VkBool32,
	pub compareOp: VkCompareOp,
	pub minLod: f32,
	pub maxLod: f32,
	pub borderColor: VkBorderColor,
	pub unnormalizedCoordinates: VkBool32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorSetLayoutBinding {
	pub binding: u32,
	pub descriptorType: VkDescriptorType,
	pub descriptorCount: u32,
	pub stageFlags: VkShaderStageFlags,
	pub pImmutableSamplers:  *const VkSampler, // *const [VkSampler], // Where is count?
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorSetLayoutCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkDescriptorSetLayoutCreateFlags,
	pub bindingCount: u32,
	pub pBindings: *const VkDescriptorSetLayoutBinding, // *const [VkDescriptorSetLayoutBinding],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorPoolSize {
	pub type_: VkDescriptorType,
	pub descriptorCount: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorPoolCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkDescriptorPoolCreateFlags,
	pub maxSets: u32,
	pub poolSizeCount: u32,
	pub pPoolSizes: *const VkDescriptorPoolSize, // *const [VkDescriptorPoolSize],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorSetAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub descriptorPool: VkDescriptorPool,
	pub descriptorSetCount: u32,
	pub pSetLayouts: *const VkDescriptorSetLayout, // *const [VkDescriptorSetLayout],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorImageInfo {
	pub sampler: VkSampler,
	pub imageView: VkImageView,
	pub imageLayout: VkImageLayout,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDescriptorBufferInfo {
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub range: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkWriteDescriptorSet {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
	pub descriptorType: VkDescriptorType,
	pub pImageInfo: *const VkDescriptorImageInfo,
	pub pBufferInfo: *const VkDescriptorBufferInfo,
	pub pTexelBufferView: *const VkBufferView,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkCopyDescriptorSet {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub srcSet: VkDescriptorSet,
	pub srcBinding: u32,
	pub srcArrayElement: u32,
	pub dstSet: VkDescriptorSet,
	pub dstBinding: u32,
	pub dstArrayElement: u32,
	pub descriptorCount: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkFramebufferCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkFramebufferCreateFlags,
	pub renderPass: VkRenderPass,
	pub attachmentCount: u32,
	pub pAttachments: *const VkImageView, // *const [VkImageView],
	pub width: u32,
	pub height: u32,
	pub layers: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkAttachmentDescription {
	pub flags: VkAttachmentDescriptionFlags,
	pub format: VkFormat,
	pub samples: VkSampleCountFlags,
	pub loadOp: VkAttachmentLoadOp,
	pub storeOp: VkAttachmentStoreOp,
	pub stencilLoadOp: VkAttachmentLoadOp,
	pub stencilStoreOp: VkAttachmentStoreOp,
	pub initialLayout: VkImageLayout,
	pub finalLayout: VkImageLayout,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkAttachmentReference {
	pub attachment: u32,
	pub layout: VkImageLayout,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSubpassDescription {
	pub flags: VkSubpassDescriptionFlags,
	pub pipelineBindPoint: VkPipelineBindPoint,
	pub inputAttachmentCount: u32,
	pub pInputAttachments: *const VkAttachmentReference, // *const [VkAttachmentReference],
	pub colorAttachmentCount: u32,
	pub pColorAttachments: *const VkAttachmentReference, // *const [VkAttachmentReference],
	pub pResolveAttachments: *const VkAttachmentReference, // *const [VkAttachmentReference],
	pub pDepthStencilAttachment: *const VkAttachmentReference,
	pub preserveAttachmentCount: u32,
	pub pPreserveAttachments: *const u32, // *const [u32],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkSubpassDependency {
	pub srcSubpass: u32,
	pub dstSubpass: u32,
	pub srcStageMask: VkPipelineStageFlags,
	pub dstStageMask: VkPipelineStageFlags,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub dependencyFlags: VkDependencyFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkRenderPassCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkRenderPassCreateFlags,
	pub attachmentCount: u32,
	pub pAttachments: *const VkAttachmentDescription, // *const [VkAttachmentDescription],
	pub subpassCount: u32,
	pub pSubpasses: *const VkSubpassDescription, // *const [VkSubpassDescription],
	pub dependencyCount: u32,
	pub pDependencies: *const VkSubpassDependency, // *const [VkSubpassDependency],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkCommandPoolCreateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub flags: VkCommandPoolCreateFlags,
	pub queueFamilyIndex: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkCommandBufferAllocateInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub commandPool: VkCommandPool,
	pub level: VkCommandBufferLevel,
	pub commandBufferCount: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkCommandBufferInheritanceInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub renderPass: VkRenderPass,
	pub subpass: u32,
	pub framebuffer: VkFramebuffer,
	pub occlusionQueryEnable: VkBool32,
	pub queryFlags: VkQueryControlFlags,
	pub pipelineStatistics: VkQueryPipelineStatisticFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkCommandBufferBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub pipelineStatistics: VkCommandBufferUsageFlags,
	pub pInheritanceInfo: *const VkCommandBufferInheritanceInfo,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkBufferCopy {
	pub srcOffset: VkDeviceSize,
	pub dstOffset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageSubresourceLayers {
	pub aspectMask: VkImageAspectFlags,
	pub mipLevel: u32,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageCopy {
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageBlit {
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffsets: [VkOffset3D; 2],
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffsets: [VkOffset3D; 2],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkBufferImageCopy {
	pub bufferOffset: VkDeviceSize,
	pub bufferRowLength: u32,
	pub bufferImageHeight: u32,
	pub imageSubresource: VkImageSubresourceLayers,
	pub imageOffset: VkOffset3D,
	pub imageExtent: VkExtent3D,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearColorValue { //typedef union VkClearColorValue {
	//f32 float32[4], see VkClearColorValueAsFloat32
	//i32 int32[4], see VkClearColorValueAsInt32
	//u32 uint32[4], see VkClearColorValueAsUint32
	union: [u8; 16],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearColorValueAsFloat32 {
	pub float32: [f32; 4],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearColorValueAsInt32 {
	pub int32: [i32; 4],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearColorValueAsUint32 {
	pub uint32: [u32; 4],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearDepthStencilValue {
	pub depth: f32,
	pub stencil: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
// TODO replace by unsafe enum as soon as available; see https://github.com/rust-lang/rfcs/issues/877
pub struct VkClearValue { //typedef union VkClearValue {
	//VkClearColorValue color, see VkClearValueAsColor
	//VkClearDepthStencilValue depthStencil, see VkClearValueAsDepthStencil
	union: [u8; 16],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearValueAsColor {
	pub color: VkClearColorValue,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearValueAsDepthStencil {
	pub depthStencil: VkClearDepthStencilValue,
}



#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearAttachment {
	pub aspectMask: VkImageAspectFlags,
	pub colorAttachment: u32,
	pub clearValue: VkClearValue,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkClearRect {
	pub rect: VkRect2D,
	pub baseArrayLayer: u32,
	pub layerCount: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageResolve {
	pub srcSubresource: VkImageSubresourceLayers,
	pub srcOffset: VkOffset3D,
	pub dstSubresource: VkImageSubresourceLayers,
	pub dstOffset: VkOffset3D,
	pub extent: VkExtent3D,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkMemoryBarrier {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkBufferMemoryBarrier {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub buffer: VkBuffer,
	pub offset: VkDeviceSize,
	pub size: VkDeviceSize,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkImageMemoryBarrier {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub srcAccessMask: VkAccessFlags,
	pub dstAccessMask: VkAccessFlags,
	pub oldLayout: VkImageLayout,
	pub newLayout: VkImageLayout,
	pub srcQueueFamilyIndex: u32,
	pub dstQueueFamilyIndex: u32,
	pub image: VkImage,
	pub subresourceRange: VkImageSubresourceRange,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkRenderPassBeginInfo {
	pub sType: VkStructureType,
	pub pNext: *const libc::c_void,
	pub renderPass: VkRenderPass,
	pub framebuffer: VkFramebuffer,
	pub renderArea: VkRect2D,
	pub clearValueCount: u32,
	pub pClearValues: *const VkClearValue, // *const [VkClearValue],
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDispatchIndirectCommand {
	pub x: u32,
	pub y: u32,
	pub z: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDrawIndexedIndirectCommand {
	pub indexCount: u32,
	pub instanceCount: u32,
	pub firstIndex: u32,
	pub vertexOffset: i32,
	pub firstInstance: u32,
}

#[repr(C)]
#[allow(non_snake_case)]
pub struct VkDrawIndirectCommand {
	pub vertexCount: u32,
	pub instanceCount: u32,
	pub firstVertex: u32,
	pub firstInstance: u32,
}

