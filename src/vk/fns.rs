extern crate libc;

use super::enums::*;
use super::types::*;
use super::flags::*;
use super::structs::*;


//#[cfg(all(target_os = "win32", target_arch = "x86"))]
#[link(name = "vulkan-1")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern "stdcall" {
	
	//VkResult vkCreateInstance(
	//    const VkInstanceCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkInstance* pInstance);
	/// instance::Instance::create
    pub fn vkCreateInstance(pCreateInfo: *const VkInstanceCreateInfo, pAllocator: *const VkAllocationCallbacks, pInstance: *mut VkInstance) -> VkResult;
    
	//void vkDestroyInstance(
	//    VkInstance instance,
	//    const VkAllocationCallbacks* pAllocator);
	/// instance::Instance::destroy
	pub fn vkDestroyInstance(instance: VkInstance, pAllocator: *const VkAllocationCallbacks);
    
	//VkResult vkEnumeratePhysicalDevices(
	//    VkInstance instance,
	//    u32* pPhysicalDeviceCount,
	//    VkPhysicalDevice* pPhysicalDevices);
	/// instance::Instance::enumerate_physical_devices
    pub fn vkEnumeratePhysicalDevices(instance: VkInstance, pPhysicalDeviceCount: *mut u32, pPhysicalDevices: *mut VkPhysicalDevice) -> VkResult;

	//void vkGetPhysicalDeviceFeatures(
	//    VkPhysicalDevice physicalDevice,
	//    VkPhysicalDeviceFeatures* pFeatures);
	/// physical_device::PhysicalDevice::create(handle: VkPhysicalDevice) -> Self
	pub fn vkGetPhysicalDeviceFeatures(physicalDevice: VkPhysicalDevice, pFeatures: *mut VkPhysicalDeviceFeatures);
    
	//void vkGetPhysicalDeviceFormatProperties(
	//    VkPhysicalDevice physicalDevice,
	//    VkFormat format,
	//    VkFormatProperties* pFormatProperties);
	/// TODO add to PhysicalDevice 
	pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
	
	//VkResult vkGetPhysicalDeviceImageFormatProperties(
	//    VkPhysicalDevice physicalDevice,
	//    VkFormat format,
	//    VkImageType type,
	//    VkImageTiling tiling,
	//    VkImageUsageFlags usage,
	//    VkImageCreateFlags flags,
	//    VkImageFormatProperties* pImageFormatProperties);
	/// TODO add to PhysicalDevice 
	pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
	
	//void vkGetPhysicalDeviceProperties(
	//    VkPhysicalDevice physicalDevice,
	//    VkPhysicalDeviceProperties* pProperties);
	/// physical_device::PhysicalDevice::create(handle: VkPhysicalDevice) -> Self
	pub fn vkGetPhysicalDeviceProperties(physicalDevice: VkPhysicalDevice, pProperties: *mut VkPhysicalDeviceProperties);
	
	//void vkGetPhysicalDeviceQueueFamilyProperties(
	//    VkPhysicalDevice physicalDevice,
	//    u32* pQueueFamilyPropertyCount,
	//    VkQueueFamilyProperties* pQueueFamilyProperties);
	/// physical_device::PhysicalDevice::create(handle: VkPhysicalDevice) -> Self
	///     physical_device::PhysicalDevice::get_queue_family_properties
	pub fn vkGetPhysicalDeviceQueueFamilyProperties(physicalDevice: VkPhysicalDevice, pQueueFamilyPropertyCount: *mut u32, pQueueFamilyProperties: *mut VkQueueFamilyProperties);
	
	//void vkGetPhysicalDeviceMemoryProperties(
	//    VkPhysicalDevice physicalDevice,
	//    VkPhysicalDeviceMemoryProperties* pMemoryProperties);
	/// physical_device::PhysicalDevice::create(handle: VkPhysicalDevice) -> Self
	pub fn vkGetPhysicalDeviceMemoryProperties(physicalDevice: VkPhysicalDevice, pMemoryProperties: *mut VkPhysicalDeviceMemoryProperties);
	
	//PFN_vkVoidFunction vkGetInstanceProcAddr(
	//    VkInstance instance,
	//    const char* pName);
	/// instance::Instance::get_proc_addr
	//pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const u8) -> PFN_vkVoidFunction;
	
	//PFN_vkVoidFunction vkGetDeviceProcAddr(
	//    VkDevice device,
	//    const char* pName);
	//pub fn vkGetDeviceProcAddr(device: VkDevice, pName: *const u8) -> PFN_vkVoidFunction;
	
	//VkResult vkCreateDevice(
	//    VkPhysicalDevice physicalDevice,
	//    const VkDeviceCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkDevice* pDevice);
	/// TODO add to PhysicalDevice 
	pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;
	
	//void vkDestroyDevice(
	//    VkDevice device,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyDevice(device: VkDevice, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkEnumerateInstanceExtensionProperties(
	//    const char* pLayerName,
	//    u32* pPropertyCount,
	//    VkExtensionProperties* pProperties);
	pub fn vkEnumerateInstanceExtensionProperties(pLayerName: *const u8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
	
	//VkResult vkEnumerateDeviceExtensionProperties(
	//    VkPhysicalDevice physicalDevice,
	//    const char* pLayerName,
	//    u32* pPropertyCount,
	//    VkExtensionProperties* pProperties);
	/// TODO add to PhysicalDevice 
	pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const u8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
	
	//VkResult vkEnumerateInstanceLayerProperties(
	//    u32* pPropertyCount,
	//    VkLayerProperties* pProperties);
	pub fn vkEnumerateInstanceLayerProperties(pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
	
	//VkResult vkEnumerateDeviceLayerProperties(
	//    VkPhysicalDevice physicalDevice,
	//    u32* pPropertyCount,
	//    VkLayerProperties* pProperties);
	/// TODO add to PhysicalDevice 
	pub fn vkEnumerateDeviceLayerProperties(physicalDevice: VkPhysicalDevice, pPropertyCount: *mut u32, pProperties: *mut VkLayerProperties) -> VkResult;
	
	//void vkGetDeviceQueue(
	//    VkDevice device,
	//    u32 queueFamilyIndex,
	//    u32 queueIndex,
	//    VkQueue* pQueue);
	pub fn vkGetDeviceQueue(device: VkDevice, queueFamilyIndex: u32, queueIndex: u32, pQueue: *mut VkQueue);
	
	//VkResult vkQueueSubmit(
	//    VkQueue                                     queue,
	//    u32 submitCount,
	//    const VkSubmitInfo* pSubmits,
	//    VkFence                                     fence);
	pub fn vkQueueSubmit(queue: VkQueue, submitCount: u32, pSubmits: *const VkSubmitInfo, fence: VkFence) -> VkResult;
	
	//VkResult vkQueueWaitIdle(
	//    VkQueue                                     queue);
	pub fn vkQueueWaitIdle(queue: VkQueue) -> VkResult;
	
	//VkResult vkDeviceWaitIdle(
	//    VkDevice device);
	pub fn vkDeviceWaitIdle(device: VkDevice) -> VkResult;
	
	//VkResult vkAllocateMemory(
	//    VkDevice device,
	//    const VkMemoryAllocateInfo* pAllocateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkDeviceMemory* pMemory);
	pub fn vkAllocateMemory(device: VkDevice, pAllocateInfo: *const VkMemoryAllocateInfo, pAllocator: *const VkAllocationCallbacks, pMemory: *mut VkDeviceMemory) -> VkResult;
	
	//void vkFreeMemory(
	//    VkDevice device,
	//    VkDeviceMemory                              memory,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkFreeMemory(device: VkDevice, memory: VkDeviceMemory, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkMapMemory(
	//    VkDevice device,
	//    VkDeviceMemory                              memory,
	//    VkDeviceSize offset,
	//    VkDeviceSize size,
	//    VkMemoryMapFlags flags,
	//    void** ppData);
	//pub fn vkMapMemory(device: VkDevice, memory: VkDeviceMemory, offset: VkDeviceSize, size: VkDeviceSize, flags: VkMemoryMapFlags, ppData: *mut *mut void) -> VkResult;
	
	//void vkUnmapMemory(
	//    VkDevice device,
	//    VkDeviceMemory                              memory);
	pub fn vkUnmapMemory(device: VkDevice, memory: VkDeviceMemory);
	
	//VkResult vkFlushMappedMemoryRanges(
	//    VkDevice device,
	//    u32 memoryRangeCount,
	//    const VkMappedMemoryRange* pMemoryRanges);
	pub fn vkFlushMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
	
	//VkResult vkInvalidateMappedMemoryRanges(
	//    VkDevice device,
	//    u32 memoryRangeCount,
	//    const VkMappedMemoryRange* pMemoryRanges);
	pub fn vkInvalidateMappedMemoryRanges(device: VkDevice, memoryRangeCount: u32, pMemoryRanges: *const VkMappedMemoryRange) -> VkResult;
	
	//void vkGetDeviceMemoryCommitment(
	//    VkDevice device,
	//    VkDeviceMemory                              memory,
	//    VkDeviceSize* pCommittedMemoryInBytes);
	pub fn vkGetDeviceMemoryCommitment(device: VkDevice, memory: VkDeviceMemory, pCommittedMemoryInBytes: *mut VkDeviceSize);
	
	//VkResult vkBindBufferMemory(
	//    VkDevice device,
	//    VkBuffer buffer,
	//    VkDeviceMemory                              memory,
	//    VkDeviceSize memoryOffset);
	pub fn vkBindBufferMemory(device: VkDevice, buffer: VkBuffer, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
	
	//VkResult vkBindImageMemory(
	//    VkDevice device,
	//    VkImage image,
	//    VkDeviceMemory                              memory,
	//    VkDeviceSize memoryOffset);
	pub fn vkBindImageMemory(device: VkDevice, image: VkImage, memory: VkDeviceMemory, memoryOffset: VkDeviceSize) -> VkResult;
	
	//void vkGetBufferMemoryRequirements(
	//    VkDevice device,
	//    VkBuffer buffer,
	//    VkMemoryRequirements* pMemoryRequirements);
	pub fn vkGetBufferMemoryRequirements(device: VkDevice, buffer: VkBuffer, pMemoryRequirements: *mut VkMemoryRequirements);
	
	//void vkGetImageMemoryRequirements(
	//    VkDevice device,
	//    VkImage image,
	//    VkMemoryRequirements* pMemoryRequirements);
	pub fn vkGetImageMemoryRequirements(device: VkDevice, image: VkImage, pMemoryRequirements: *mut VkMemoryRequirements);
	
	//void vkGetImageSparseMemoryRequirements(
	//    VkDevice device,
	//    VkImage image,
	//    u32* pSparseMemoryRequirementCount,
	//    VkSparseImageMemoryRequirements* pSparseMemoryRequirements);
	pub fn vkGetImageSparseMemoryRequirements(device: VkDevice, image: VkImage, pSparseMemoryRequirementCount: *mut u32, pSparseMemoryRequirements: *mut VkSparseImageMemoryRequirements);
	
	//void vkGetPhysicalDeviceSparseImageFormatProperties(
	//    VkPhysicalDevice physicalDevice,
	//    VkFormat format,
	//    VkImageType type,
	//    VkSampleCountFlagBits samples,
	//    VkImageUsageFlags usage,
	//    VkImageTiling                               tiling,
	//    u32* pPropertyCount,
	//    VkSparseImageFormatProperties* pProperties);
	/// TODO add to PhysicalDevice 
	pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
	
	//VkResult vkQueueBindSparse(
	//    VkQueue                                     queue,
	//    u32 bindInfoCount,
	//    const VkBindSparseInfo* pBindInfo,
	//    VkFence                                     fence);
	pub fn vkQueueBindSparse(queue: VkQueue, bindInfoCount: u32, pBindInfo: *const VkBindSparseInfo, fence: VkFence) -> VkResult;
	
	//VkResult vkCreateFence(
	//    VkDevice device,
	//    const VkFenceCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkFence* pFence);
	pub fn vkCreateFence(device: VkDevice, pCreateInfo: *const VkFenceCreateInfo, pAllocator: *const VkAllocationCallbacks, pFence: *mut VkFence) -> VkResult;
	
	//void vkDestroyFence(
	//    VkDevice device,
	//    VkFence                                     fence,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyFence(device: VkDevice, fence: VkFence, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkResetFences(
	//    VkDevice device,
	//    u32 fenceCount,
	//    const VkFence* pFences);
	pub fn vkResetFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence) -> VkResult;
	
	//VkResult vkGetFenceStatus(
	//    VkDevice device,
	//    VkFence                                     fence);
	pub fn vkGetFenceStatus(device: VkDevice, fence: VkFence) -> VkResult;
	
	//VkResult vkWaitForFences(
	//    VkDevice device,
	//    u32 fenceCount,
	//    const VkFence* pFences,
	//    VkBool32 waitAll,
	//    u64 timeout);
	pub fn vkWaitForFences(device: VkDevice, fenceCount: u32, pFences: *const VkFence, waitAll: VkBool32, timeout: u64) -> VkResult;
	
	//VkResult vkCreateSemaphore(
	//    VkDevice device,
	//    const VkSemaphoreCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkSemaphore* pSemaphore);
	pub fn vkCreateSemaphore(device: VkDevice, pCreateInfo: *const VkSemaphoreCreateInfo, pAllocator: *const VkAllocationCallbacks, pSemaphore: *mut VkSemaphore) -> VkResult;
	
	//void vkDestroySemaphore(
	//    VkDevice device,
	//    VkSemaphore                                 semaphore,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroySemaphore(device: VkDevice, semaphore: VkSemaphore, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateEvent(
	//    VkDevice device,
	//    const VkEventCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkEvent* pEvent);
	pub fn vkCreateEvent(device: VkDevice, pCreateInfo: *const VkEventCreateInfo, pAllocator: *const VkAllocationCallbacks, pEvent: *mut VkEvent) -> VkResult;
	
	//void vkDestroyEvent(
	//    VkDevice device,
	//    VkEvent                                     event,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyEvent(device: VkDevice, event: VkEvent, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkGetEventStatus(
	//    VkDevice device,
	//    VkEvent                                     event);
	pub fn vkGetEventStatus(device: VkDevice, event: VkEvent) -> VkResult;
	
	//VkResult vkSetEvent(
	//    VkDevice device,
	//    VkEvent                                     event);
	pub fn vkSetEvent(device: VkDevice, event: VkEvent) -> VkResult;
	
	//VkResult vkResetEvent(
	//    VkDevice device,
	//    VkEvent                                     event);
	pub fn vkResetEvent(device: VkDevice, event: VkEvent) -> VkResult;
	
	//VkResult vkCreateQueryPool(
	//    VkDevice device,
	//    const VkQueryPoolCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkQueryPool* pQueryPool);
	pub fn vkCreateQueryPool(device: VkDevice, pCreateInfo: *const VkQueryPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pQueryPool: *mut VkQueryPool) -> VkResult;
	
	//void vkDestroyQueryPool(
	//    VkDevice device,
	//    VkQueryPool queryPool,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyQueryPool(device: VkDevice, queryPool: VkQueryPool, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkGetQueryPoolResults(
	//    VkDevice device,
	//    VkQueryPool queryPool,
	//    u32 firstQuery,
	//    u32 queryCount,
	//    usize dataSize,
	//    void* pData,
	//    VkDeviceSize stride,
	//    VkQueryResultFlags flags);
	//pub fn vkGetQueryPoolResults(device: VkDevice, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dataSize: usize, pData: *mut void, stride: VkDeviceSize, flags: VkQueryResultFlags) -> VkResult;
	
	//VkResult vkCreateBuffer(
	//    VkDevice device,
	//    const VkBufferCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkBuffer* pBuffer);
	pub fn vkCreateBuffer(device: VkDevice, pCreateInfo: *const VkBufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pBuffer: *mut VkBuffer) -> VkResult;
	
	//void vkDestroyBuffer(
	//    VkDevice device,
	//    VkBuffer buffer,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyBuffer(device: VkDevice, buffer: VkBuffer, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateBufferView(
	//    VkDevice device,
	//    const VkBufferViewCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkBufferView* pView);
	pub fn vkCreateBufferView(device: VkDevice, pCreateInfo: *const VkBufferViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkBufferView) -> VkResult;
	
	//void vkDestroyBufferView(
	//    VkDevice device,
	//    VkBufferView                                bufferView,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyBufferView(device: VkDevice, bufferView: VkBufferView, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateImage(
	//    VkDevice device,
	//    const VkImageCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkImage* pImage);
	pub fn vkCreateImage(device: VkDevice, pCreateInfo: *const VkImageCreateInfo, pAllocator: *const VkAllocationCallbacks, pImage: *mut VkImage) -> VkResult;
	
	//void vkDestroyImage(
	//    VkDevice device,
	//    VkImage image,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyImage(device: VkDevice, image: VkImage, pAllocator: *const VkAllocationCallbacks);
	
	//void vkGetImageSubresourceLayout(
	//    VkDevice device,
	//    VkImage image,
	//    const VkImageSubresource* pSubresource,
	//    VkSubresourceLayout* pLayout);
	pub fn vkGetImageSubresourceLayout(device: VkDevice, image: VkImage, pSubresource: *const VkImageSubresource, pLayout: *mut VkSubresourceLayout);
	
	//VkResult vkCreateImageView(
	//    VkDevice device,
	//    const VkImageViewCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkImageView* pView);
	pub fn vkCreateImageView(device: VkDevice, pCreateInfo: *const VkImageViewCreateInfo, pAllocator: *const VkAllocationCallbacks, pView: *mut VkImageView) -> VkResult;
	
	//void vkDestroyImageView(
	//    VkDevice device,
	//    VkImageView                                 imageView,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyImageView(device: VkDevice, imageView: VkImageView, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateShaderModule(
	//    VkDevice device,
	//    const VkShaderModuleCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkShaderModule* pShaderModule);
	pub fn vkCreateShaderModule(device: VkDevice, pCreateInfo: *const VkShaderModuleCreateInfo, pAllocator: *const VkAllocationCallbacks, pShaderModule: *mut VkShaderModule) -> VkResult;
	
	//void vkDestroyShaderModule(
	//    VkDevice device,
	//    VkShaderModule                              shaderModule,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyShaderModule(device: VkDevice, shaderModule: VkShaderModule, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreatePipelineCache(
	//    VkDevice device,
	//    const VkPipelineCacheCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkPipelineCache* pPipelineCache);
	pub fn vkCreatePipelineCache(device: VkDevice, pCreateInfo: *const VkPipelineCacheCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineCache: *mut VkPipelineCache) -> VkResult;
	
	//void vkDestroyPipelineCache(
	//    VkDevice device,
	//    VkPipelineCache                             pipelineCache,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyPipelineCache(device: VkDevice, pipelineCache: VkPipelineCache, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkGetPipelineCacheData(
	//    VkDevice device,
	//    VkPipelineCache                             pipelineCache,
	//    usize* pDataSize,
	//    void* pData);
	// pub fn vkGetPipelineCacheData(device: VkDevice, pipelineCache: VkPipelineCache, pDataSize: *mut usize, pData: *mut void) -> VkResult;
	
	//VkResult vkMergePipelineCaches(
	//    VkDevice device,
	//    VkPipelineCache                             dstCache,
	//    u32 srcCacheCount,
	//    const VkPipelineCache* pSrcCaches);
	pub fn vkMergePipelineCaches(device: VkDevice, dstCache: VkPipelineCache, srcCacheCount: u32, pSrcCaches: *const VkPipelineCache) -> VkResult;
	
	//VkResult vkCreateGraphicsPipelines(
	//    VkDevice device,
	//    VkPipelineCache                             pipelineCache,
	//    u32 createInfoCount,
	//    const VkGraphicsPipelineCreateInfo* pCreateInfos,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkPipeline* pPipelines);
	pub fn vkCreateGraphicsPipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkGraphicsPipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
	
	//VkResult vkCreateComputePipelines(
	//    VkDevice device,
	//    VkPipelineCache                             pipelineCache,
	//    u32 createInfoCount,
	//    const VkComputePipelineCreateInfo* pCreateInfos,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkPipeline* pPipelines);
	pub fn vkCreateComputePipelines(device: VkDevice, pipelineCache: VkPipelineCache, createInfoCount: u32, pCreateInfos: *const VkComputePipelineCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelines: *mut VkPipeline) -> VkResult;
	
	//void vkDestroyPipeline(
	//    VkDevice device,
	//    VkPipeline                                  pipeline,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyPipeline(device: VkDevice, pipeline: VkPipeline, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreatePipelineLayout(
	//    VkDevice device,
	//    const VkPipelineLayoutCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkPipelineLayout* pPipelineLayout);
	pub fn vkCreatePipelineLayout(device: VkDevice, pCreateInfo: *const VkPipelineLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pPipelineLayout: *mut VkPipelineLayout) -> VkResult;
	
	//void vkDestroyPipelineLayout(
	//    VkDevice device,
	//    VkPipelineLayout pipelineLayout,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyPipelineLayout(device: VkDevice, pipelineLayout: VkPipelineLayout, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateSampler(
	//    VkDevice device,
	//    const VkSamplerCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkSampler* pSampler);
	pub fn vkCreateSampler(device: VkDevice, pCreateInfo: *const VkSamplerCreateInfo, pAllocator: *const VkAllocationCallbacks, pSampler: *mut VkSampler) -> VkResult;
	
	//void vkDestroySampler(
	//    VkDevice device,
	//    VkSampler                                   sampler,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroySampler(device: VkDevice, sampler: VkSampler, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateDescriptorSetLayout(
	//    VkDevice device,
	//    const VkDescriptorSetLayoutCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkDescriptorSetLayout* pSetLayout);
	pub fn vkCreateDescriptorSetLayout(device: VkDevice, pCreateInfo: *const VkDescriptorSetLayoutCreateInfo, pAllocator: *const VkAllocationCallbacks, pSetLayout: *mut VkDescriptorSetLayout) -> VkResult;
	
	//void vkDestroyDescriptorSetLayout(
	//    VkDevice device,
	//    VkDescriptorSetLayout descriptorSetLayout,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyDescriptorSetLayout(device: VkDevice, descriptorSetLayout: VkDescriptorSetLayout, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateDescriptorPool(
	//    VkDevice device,
	//    const VkDescriptorPoolCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkDescriptorPool* pDescriptorPool);
	pub fn vkCreateDescriptorPool(device: VkDevice, pCreateInfo: *const VkDescriptorPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pDescriptorPool: *mut VkDescriptorPool) -> VkResult;
	
	//void vkDestroyDescriptorPool(
	//    VkDevice device,
	//    VkDescriptorPool descriptorPool,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkResetDescriptorPool(
	//    VkDevice device,
	//    VkDescriptorPool descriptorPool,
	//    VkDescriptorPoolResetFlags flags);
	pub fn vkResetDescriptorPool(device: VkDevice, descriptorPool: VkDescriptorPool, flags: VkDescriptorPoolResetFlags) -> VkResult;
	
	//VkResult vkAllocateDescriptorSets(
	//    VkDevice device,
	//    const VkDescriptorSetAllocateInfo* pAllocateInfo,
	//    VkDescriptorSet* pDescriptorSets);
	pub fn vkAllocateDescriptorSets(device: VkDevice, pAllocateInfo: *const VkDescriptorSetAllocateInfo, pDescriptorSets: *mut VkDescriptorSet) -> VkResult;
	
	//VkResult vkFreeDescriptorSets(
	//    VkDevice device,
	//    VkDescriptorPool descriptorPool,
	//    u32 descriptorSetCount,
	//    const VkDescriptorSet* pDescriptorSets);
	pub fn vkFreeDescriptorSets(device: VkDevice, descriptorPool: VkDescriptorPool, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet) -> VkResult;
	
	//void vkUpdateDescriptorSets(
	//    VkDevice device,
	//    u32 descriptorWriteCount,
	//    const VkWriteDescriptorSet* pDescriptorWrites,
	//    u32 descriptorCopyCount,
	//    const VkCopyDescriptorSet* pDescriptorCopies);
	pub fn vkUpdateDescriptorSets(device: VkDevice, descriptorWriteCount: u32, pDescriptorWrites: *const VkWriteDescriptorSet, descriptorCopyCount: u32, pDescriptorCopies: *const VkCopyDescriptorSet);
	
	//VkResult vkCreateFramebuffer(
	//    VkDevice device,
	//    const VkFramebufferCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkFramebuffer* pFramebuffer);
	pub fn vkCreateFramebuffer(device: VkDevice, pCreateInfo: *const VkFramebufferCreateInfo, pAllocator: *const VkAllocationCallbacks, pFramebuffer: *mut VkFramebuffer) -> VkResult;
	
	//void vkDestroyFramebuffer(
	//    VkDevice device,
	//    VkFramebuffer framebuffer,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyFramebuffer(device: VkDevice, framebuffer: VkFramebuffer, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkCreateRenderPass(
	//    VkDevice device,
	//    const VkRenderPassCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkRenderPass* pRenderPass);
	pub fn vkCreateRenderPass(device: VkDevice, pCreateInfo: *const VkRenderPassCreateInfo, pAllocator: *const VkAllocationCallbacks, pRenderPass: *mut VkRenderPass) -> VkResult;
	
	//void vkDestroyRenderPass(
	//    VkDevice device,
	//    VkRenderPass renderPass,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyRenderPass(device: VkDevice, renderPass: VkRenderPass, pAllocator: *const VkAllocationCallbacks);
	
	//void vkGetRenderAreaGranularity(
	//    VkDevice device,
	//    VkRenderPass renderPass,
	//    VkExtent2D* pGranularity);
	pub fn vkGetRenderAreaGranularity(device: VkDevice, renderPass: VkRenderPass, pGranularity: *mut VkExtent2D);
	
	//VkResult vkCreateCommandPool(
	//    VkDevice device,
	//    const VkCommandPoolCreateInfo* pCreateInfo,
	//    const VkAllocationCallbacks* pAllocator,
	//    VkCommandPool* pCommandPool);
	pub fn vkCreateCommandPool(device: VkDevice, pCreateInfo: *const VkCommandPoolCreateInfo, pAllocator: *const VkAllocationCallbacks, pCommandPool: *mut VkCommandPool) -> VkResult;
	
	//void vkDestroyCommandPool(
	//    VkDevice device,
	//    VkCommandPool commandPool,
	//    const VkAllocationCallbacks* pAllocator);
	pub fn vkDestroyCommandPool(device: VkDevice, commandPool: VkCommandPool, pAllocator: *const VkAllocationCallbacks);
	
	//VkResult vkResetCommandPool(
	//    VkDevice device,
	//    VkCommandPool commandPool,
	//    VkCommandPoolResetFlags flags);
	pub fn vkResetCommandPool(device: VkDevice, commandPool: VkCommandPool, flags: VkCommandPoolResetFlags) -> VkResult;
	
	//VkResult vkAllocateCommandBuffers(
	//    VkDevice device,
	//    const VkCommandBufferAllocateInfo* pAllocateInfo,
	//    VkCommandBuffer* pCommandBuffers);
	pub fn vkAllocateCommandBuffers(device: VkDevice, pAllocateInfo: *const VkCommandBufferAllocateInfo, pCommandBuffers: *mut VkCommandBuffer) -> VkResult;
	
	//void vkFreeCommandBuffers(
	//    VkDevice device,
	//    VkCommandPool commandPool,
	//    u32 commandBufferCount,
	//    const VkCommandBuffer* pCommandBuffers);
	pub fn vkFreeCommandBuffers(device: VkDevice, commandPool: VkCommandPool, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);
	
	//VkResult vkBeginCommandBuffer(
	//    VkCommandBuffer commandBuffer,
	//    const VkCommandBufferBeginInfo* pBeginInfo);
	pub fn vkBeginCommandBuffer(commandBuffer: VkCommandBuffer, pBeginInfo: *const VkCommandBufferBeginInfo) -> VkResult;
	
	//VkResult vkEndCommandBuffer(
	//    VkCommandBuffer commandBuffer);
	pub fn vkEndCommandBuffer(commandBuffer: VkCommandBuffer) -> VkResult;
	
	//VkResult vkResetCommandBuffer(
	//    VkCommandBuffer commandBuffer,
	//    VkCommandBufferResetFlags flags);
	pub fn vkResetCommandBuffer(commandBuffer: VkCommandBuffer, flags: VkCommandBufferResetFlags) -> VkResult;
	
	//void vkCmdBindPipeline(
	//    VkCommandBuffer commandBuffer,
	//    VkPipelineBindPoint                         pipelineBindPoint,
	//    VkPipeline                                  pipeline);
	pub fn vkCmdBindPipeline(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, pipeline: VkPipeline);
	
	//void vkCmdSetViewport(
	//    VkCommandBuffer commandBuffer,
	//    u32 firstViewport,
	//    u32 viewportCount,
	//    const VkViewport* pViewports);
	pub fn vkCmdSetViewport(commandBuffer: VkCommandBuffer, firstViewport: u32, viewportCount: u32, pViewports: *const VkViewport);
	
	//void vkCmdSetScissor(
	//    VkCommandBuffer commandBuffer,
	//    u32 firstScissor,
	//    u32 scissorCount,
	//    const VkRect2D* pScissors);
	pub fn vkCmdSetScissor(commandBuffer: VkCommandBuffer, firstScissor: u32, scissorCount: u32, pScissors: *const VkRect2D);
	
	//void vkCmdSetLineWidth(
	//    VkCommandBuffer commandBuffer,
	//    f32 lineWidth);
	pub fn vkCmdSetLineWidth(commandBuffer: VkCommandBuffer, lineWidth: f32);
	
	//void vkCmdSetDepthBias(
	//    VkCommandBuffer commandBuffer,
	//    f32 depthBiasConstantFactor,
	//    f32 depthBiasClamp,
	//    f32 depthBiasSlopeFactor);
	pub fn vkCmdSetDepthBias(commandBuffer: VkCommandBuffer, depthBiasConstantFactor: f32, depthBiasClamp: f32, depthBiasSlopeFactor: f32);
	
	//void vkCmdSetBlendConstants(
	//    VkCommandBuffer commandBuffer,
	//    const f32 blendConstants[4]);
	pub fn vkCmdSetBlendConstants(commandBuffer: VkCommandBuffer, blendConstants: [f32; 4]);
	
	//void vkCmdSetDepthBounds(
	//    VkCommandBuffer commandBuffer,
	//    f32 minDepthBounds,
	//    f32 maxDepthBounds);
	pub fn vkCmdSetDepthBounds(commandBuffer: VkCommandBuffer, minDepthBounds: f32, maxDepthBounds: f32);
	
	//void vkCmdSetStencilCompareMask(
	//    VkCommandBuffer commandBuffer,
	//    VkStencilFaceFlags faceMask,
	//    u32 compareMask);
	pub fn vkCmdSetStencilCompareMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, compareMask: u32);
	
	//void vkCmdSetStencilWriteMask(
	//    VkCommandBuffer commandBuffer,
	//    VkStencilFaceFlags faceMask,
	//    u32 writeMask);
	pub fn vkCmdSetStencilWriteMask(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, writeMask: u32);
	
	//void vkCmdSetStencilReference(
	//    VkCommandBuffer commandBuffer,
	//    VkStencilFaceFlags faceMask,
	//    u32 reference);
	pub fn vkCmdSetStencilReference(commandBuffer: VkCommandBuffer, faceMask: VkStencilFaceFlags, reference: u32);
	
	//void vkCmdBindDescriptorSets(
	//    VkCommandBuffer commandBuffer,
	//    VkPipelineBindPoint                         pipelineBindPoint,
	//    VkPipelineLayout layout,
	//    u32 firstSet,
	//    u32 descriptorSetCount,
	//    const VkDescriptorSet* pDescriptorSets,
	//    u32 dynamicOffsetCount,
	//    const u32* pDynamicOffsets);
	//pub fn vkCmdBindDescriptorSets(commandBuffer: VkCommandBuffer, pipelineBindPoint: VkPipelineBindPoint, layout: VkPipelineLayout, firstSet: u32, descriptorSetCount: u32, pDescriptorSets: *const VkDescriptorSet, dynamicOffsetCount: u32, pDynamicOffsets: *const [u32]);
	
	//void vkCmdBindIndexBuffer(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer buffer,
	//    VkDeviceSize offset,
	//    VkIndexType indexType);
	//
	pub fn vkCmdBindIndexBuffer(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, indexType: VkIndexType);
	
	//void vkCmdBindVertexBuffers(
	//    VkCommandBuffer commandBuffer,
	//    u32 firstBinding,
	//    u32 bindingCount,
	//    const VkBuffer* pBuffers,
	//    const VkDeviceSize* pOffsets);
	pub fn vkCmdBindVertexBuffers(commandBuffer: VkCommandBuffer, firstBinding: u32, bindingCount: u32, pBuffers: *const VkBuffer, pOffsets: *const VkDeviceSize);
	
	//void vkCmdDraw(
	//    VkCommandBuffer commandBuffer,
	//    u32 vertexCount,
	//    u32 instanceCount,
	//    u32 firstVertex,
	//    u32 firstInstance);
	pub fn vkCmdDraw(commandBuffer: VkCommandBuffer, vertexCount: u32, instanceCount: u32, firstVertex: u32, firstInstance: u32);
	
	//void vkCmdDrawIndexed(
	//    VkCommandBuffer commandBuffer,
	//    u32 indexCount,
	//    u32 instanceCount,
	//    u32 firstIndex,
	//    i32 vertexOffset,
	//    u32 firstInstance);
	pub fn vkCmdDrawIndexed(commandBuffer: VkCommandBuffer, indexCount: u32, instanceCount: u32, firstIndex: u32, vertexOffset: i32, firstInstance: u32);
	
	//void vkCmdDrawIndirect(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer buffer,
	//    VkDeviceSize offset,
	//    u32 drawCount,
	//    u32 stride);
	pub fn vkCmdDrawIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
	
	//void vkCmdDrawIndexedIndirect(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer buffer,
	//    VkDeviceSize offset,
	//    u32 drawCount,
	//    u32 stride);
	pub fn vkCmdDrawIndexedIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize, drawCount: u32, stride: u32);
	
	//void vkCmdDispatch(
	//    VkCommandBuffer commandBuffer,
	//    u32 x,
	//    u32 y,
	//    u32 z);
	pub fn vkCmdDispatch(commandBuffer: VkCommandBuffer, x: u32, y: u32, z: u32);
	
	//void vkCmdDispatchIndirect(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer buffer,
	//    VkDeviceSize offset);
	pub fn vkCmdDispatchIndirect(commandBuffer: VkCommandBuffer, buffer: VkBuffer, offset: VkDeviceSize);
	
	//void vkCmdCopyBuffer(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer srcBuffer,
	//    VkBuffer dstBuffer,
	//    u32 regionCount,
	//    const VkBufferCopy* pRegions);
	pub fn vkCmdCopyBuffer(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferCopy);
	
	//void vkCmdCopyImage(
	//    VkCommandBuffer commandBuffer,
	//    VkImage srcImage,
	//    VkImageLayout srcImageLayout,
	//    VkImage dstImage,
	//    VkImageLayout dstImageLayout,
	//    u32 regionCount,
	//    const VkImageCopy* pRegions);
	pub fn vkCmdCopyImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageCopy);
	
	//void vkCmdBlitImage(
	//    VkCommandBuffer commandBuffer,
	//    VkImage srcImage,
	//    VkImageLayout srcImageLayout,
	//    VkImage dstImage,
	//    VkImageLayout dstImageLayout,
	//    u32 regionCount,
	//    const VkImageBlit* pRegions,
	//    VkFilter                                    filter);
	pub fn vkCmdBlitImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageBlit, filter: VkFilter);
	
	//void vkCmdCopyBufferToImage(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer srcBuffer,
	//    VkImage dstImage,
	//    VkImageLayout dstImageLayout,
	//    u32 regionCount,
	//    const VkBufferImageCopy* pRegions);
	pub fn vkCmdCopyBufferToImage(commandBuffer: VkCommandBuffer, srcBuffer: VkBuffer, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkBufferImageCopy);
	
	//void vkCmdCopyImageToBuffer(
	//    VkCommandBuffer commandBuffer,
	//    VkImage srcImage,
	//    VkImageLayout srcImageLayout,
	//    VkBuffer dstBuffer,
	//    u32 regionCount,
	//    const VkBufferImageCopy* pRegions);
	pub fn vkCmdCopyImageToBuffer(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstBuffer: VkBuffer, regionCount: u32, pRegions: *const VkBufferImageCopy);
	
	//void vkCmdUpdateBuffer(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer dstBuffer,
	//    VkDeviceSize dstOffset,
	//    VkDeviceSize dataSize,
	//    const u32* pData);
	//pub fn vkCmdUpdateBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, dataSize: VkDeviceSize, pData: *const [u32]);
	
	//void vkCmdFillBuffer(
	//    VkCommandBuffer commandBuffer,
	//    VkBuffer dstBuffer,
	//    VkDeviceSize dstOffset,
	//    VkDeviceSize size,
	//    u32 data);
	pub fn vkCmdFillBuffer(commandBuffer: VkCommandBuffer, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, size: VkDeviceSize, data: u32);
	
	//void vkCmdClearColorImage(
	//    VkCommandBuffer commandBuffer,
	//    VkImage image,
	//    VkImageLayout imageLayout,
	//    const VkClearColorValue* pColor,
	//    u32 rangeCount,
	//    const VkImageSubresourceRange* pRanges);
	//pub fn vkCmdClearColorImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pColor: *const VkClearColorValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
	
	//void vkCmdClearDepthStencilImage(
	//    VkCommandBuffer commandBuffer,
	//    VkImage image,
	//    VkImageLayout imageLayout,
	//    const VkClearDepthStencilValue* pDepthStencil,
	//    u32 rangeCount,
	//    const VkImageSubresourceRange* pRanges);
	pub fn vkCmdClearDepthStencilImage(commandBuffer: VkCommandBuffer, image: VkImage, imageLayout: VkImageLayout, pDepthStencil: *const VkClearDepthStencilValue, rangeCount: u32, pRanges: *const VkImageSubresourceRange);
	
	//void vkCmdClearAttachments(
	//    VkCommandBuffer commandBuffer,
	//    u32 attachmentCount,
	//    const VkClearAttachment* pAttachments,
	//    u32 rectCount,
	//    const VkClearRect* pRects);
	pub fn vkCmdClearAttachments(commandBuffer: VkCommandBuffer, attachmentCount: u32, pAttachments: *const VkClearAttachment, rectCount: u32, pRects: *const VkClearRect);
	
	//void vkCmdResolveImage(
	//    VkCommandBuffer commandBuffer,
	//    VkImage srcImage,
	//    VkImageLayout srcImageLayout,
	//    VkImage dstImage,
	//    VkImageLayout dstImageLayout,
	//    u32 regionCount,
	//    const VkImageResolve* pRegions);
	pub fn vkCmdResolveImage(commandBuffer: VkCommandBuffer, srcImage: VkImage, srcImageLayout: VkImageLayout, dstImage: VkImage, dstImageLayout: VkImageLayout, regionCount: u32, pRegions: *const VkImageResolve);
	
	//void vkCmdSetEvent(
	//    VkCommandBuffer commandBuffer,
	//    VkEvent                                     event,
	//    VkPipelineStageFlags stageMask);
	pub fn vkCmdSetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
	
	//void vkCmdResetEvent(
	//    VkCommandBuffer commandBuffer,
	//    VkEvent                                     event,
	//    VkPipelineStageFlags stageMask);
	pub fn vkCmdResetEvent(commandBuffer: VkCommandBuffer, event: VkEvent, stageMask: VkPipelineStageFlags);
	
	//void vkCmdWaitEvents(
	//    VkCommandBuffer commandBuffer,
	//    u32 eventCount,
	//    const VkEvent* pEvents,
	//    VkPipelineStageFlags srcStageMask,
	//    VkPipelineStageFlags dstStageMask,
	//    u32 memoryBarrierCount,
	//    const VkMemoryBarrier* pMemoryBarriers,
	//    u32 bufferMemoryBarrierCount,
	//    const VkBufferMemoryBarrier* pBufferMemoryBarriers,
	//    u32 imageMemoryBarrierCount,
	//    const VkImageMemoryBarrier* pImageMemoryBarriers);
	pub fn vkCmdWaitEvents(commandBuffer: VkCommandBuffer, eventCount: u32, pEvents: *const VkEvent, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
	
	//void vkCmdPipelineBarrier(
	//    VkCommandBuffer commandBuffer,
	//    VkPipelineStageFlags srcStageMask,
	//    VkPipelineStageFlags dstStageMask,
	//    VkDependencyFlags dependencyFlags,
	//    u32 memoryBarrierCount,
	//    const VkMemoryBarrier* pMemoryBarriers,
	//    u32 bufferMemoryBarrierCount,
	//    const VkBufferMemoryBarrier* pBufferMemoryBarriers,
	//    u32 imageMemoryBarrierCount,
	//    const VkImageMemoryBarrier* pImageMemoryBarriers);
	pub fn vkCmdPipelineBarrier(commandBuffer: VkCommandBuffer, srcStageMask: VkPipelineStageFlags, dstStageMask: VkPipelineStageFlags, dependencyFlags: VkDependencyFlags, memoryBarrierCount: u32, pMemoryBarriers: *const VkMemoryBarrier, bufferMemoryBarrierCount: u32, pBufferMemoryBarriers: *const VkBufferMemoryBarrier, imageMemoryBarrierCount: u32, pImageMemoryBarriers: *const VkImageMemoryBarrier);
	
	//void vkCmdBeginQuery(
	//    VkCommandBuffer commandBuffer,
	//    VkQueryPool queryPool,
	//    u32 query,
	//    VkQueryControlFlags flags);
	pub fn vkCmdBeginQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32, flags: VkQueryControlFlags);
	
	//void vkCmdEndQuery(
	//    VkCommandBuffer commandBuffer,
	//    VkQueryPool queryPool,
	//    u32 query);
	pub fn vkCmdEndQuery(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, query: u32);
	
	//void vkCmdResetQueryPool(
	//    VkCommandBuffer commandBuffer,
	//    VkQueryPool queryPool,
	//    u32 firstQuery,
	//    u32 queryCount);
	pub fn vkCmdResetQueryPool(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32);
	
	//void vkCmdWriteTimestamp(
	//    VkCommandBuffer commandBuffer,
	//    VkPipelineStageFlagBits pipelineStage,
	//    VkQueryPool queryPool,
	//    u32 query);
	pub fn vkCmdWriteTimestamp(commandBuffer: VkCommandBuffer, pipelineStage: VkPipelineStageFlagBits, queryPool: VkQueryPool, query: u32);
	
	//void vkCmdCopyQueryPoolResults(
	//    VkCommandBuffer commandBuffer,
	//    VkQueryPool queryPool,
	//    u32 firstQuery,
	//    u32 queryCount,
	//    VkBuffer dstBuffer,
	//    VkDeviceSize dstOffset,
	//    VkDeviceSize stride,
	//    VkQueryResultFlags flags);
	pub fn vkCmdCopyQueryPoolResults(commandBuffer: VkCommandBuffer, queryPool: VkQueryPool, firstQuery: u32, queryCount: u32, dstBuffer: VkBuffer, dstOffset: VkDeviceSize, stride: VkDeviceSize, flags: VkQueryResultFlags);
	
	//void vkCmdPushConstants(
	//    VkCommandBuffer commandBuffer,
	//    VkPipelineLayout layout,
	//    VkShaderStageFlags stageFlags,
	//    u32 offset,
	//    u32 size,
	//    const void* pValues);
	//pub fn vkCmdPushConstants(commandBuffer: VkCommandBuffer, layout: VkPipelineLayout, stageFlags: VkShaderStageFlags, offset: u32, size: u32, pValues: *const void);
	
	//void vkCmdBeginRenderPass(
	//    VkCommandBuffer commandBuffer,
	//    const VkRenderPassBeginInfo* pRenderPassBegin,
	//    VkSubpassContents                           contents);
	pub fn vkCmdBeginRenderPass(commandBuffer: VkCommandBuffer, pRenderPassBegin: *const VkRenderPassBeginInfo, contents: VkSubpassContents);
	
	//void vkCmdNextSubpass(
	//    VkCommandBuffer commandBuffer,
	//    VkSubpassContents                           contents);
	pub fn vkCmdNextSubpass(commandBuffer: VkCommandBuffer, contents: VkSubpassContents);
	
	//void vkCmdEndRenderPass(
	//    VkCommandBuffer commandBuffer);
	pub fn vkCmdEndRenderPass(commandBuffer: VkCommandBuffer);
	
	//void vkCmdExecuteCommands(
	//    VkCommandBuffer commandBuffer,
	//    u32 commandBufferCount,
	//    const VkCommandBuffer* pCommandBuffers);
	pub fn vkCmdExecuteCommands(commandBuffer: VkCommandBuffer, commandBufferCount: u32, pCommandBuffers: *const VkCommandBuffer);

}

