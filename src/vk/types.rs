
pub type VkBool32 = u32;
pub type VkDeviceSize = u64;
pub type VkSampleMask = u32;

//#define VK_DEFINE_HANDLE(object) typedef struct object##_T* object;
pub type DispatchableHandle = u32;

//VK_DEFINE_HANDLE(VkInstance)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkInstance(pub DispatchableHandle);

//VK_DEFINE_HANDLE(VkPhysicalDevice)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPhysicalDevice(pub DispatchableHandle);

//VK_DEFINE_HANDLE(VkDevice)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDevice(pub DispatchableHandle);

//VK_DEFINE_HANDLE(VkQueue)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueue(pub DispatchableHandle);

//VK_DEFINE_HANDLE(VkCommandBuffer)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandBuffer(pub DispatchableHandle);




//#if defined(__LP64__) || defined(_WIN64) || defined(__x86_64__) || defined(_M_X64) || defined(__ia64) || defined (_M_IA64) || defined(__aarch64__) || defined(__powerpc64__)
//        #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef struct object##_T *object;
//#else
//        #define VK_DEFINE_NON_DISPATCHABLE_HANDLE(object) typedef u64 object;
//#endif
pub type NonDispatchableHandle = u32;

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSemaphore)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSemaphore(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkFence)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFence(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDeviceMemory)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDeviceMemory(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBuffer)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBuffer(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImage)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImage(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkEvent)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkEvent(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkQueryPool)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkQueryPool(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkBufferView)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkBufferView(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkImageView)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkImageView(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkShaderModule)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkShaderModule(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipelineCache)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineCache(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipelineLayout)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipelineLayout(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkRenderPass)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkRenderPass(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkPipeline)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkPipeline(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorSetLayout)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSetLayout(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkSampler)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkSampler(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorPool)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorPool(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkDescriptorSet)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkDescriptorSet(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkFramebuffer)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkFramebuffer(pub NonDispatchableHandle);

//VK_DEFINE_NON_DISPATCHABLE_HANDLE(VkCommandPool)
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct VkCommandPool(pub NonDispatchableHandle);



//typedef void* (*PFN_vkAllocationFunction)(
//    void* pUserData,
//    usize size,
//    usize alignment,
//    VkSystemAllocationScope                     allocationScope);
//
//typedef void* (*PFN_vkReallocationFunction)(
//    void* pUserData,
//    void* pOriginal,
//    usize size,
//    usize alignment,
//    VkSystemAllocationScope                     allocationScope);
//
//typedef void (*PFN_vkFreeFunction)(
//    void* pUserData,
//    void* pMemory);
//
//typedef void (*PFN_vkInternalAllocationNotification)(
//    void* pUserData,
//    usize size,
//    VkInternalAllocationType allocationType,
//    VkSystemAllocationScope                     allocationScope);
//
//typedef void (*PFN_vkInternalFreeNotification)(
//    void* pUserData,
//    usize size,
//    VkInternalAllocationType allocationType,
//    VkSystemAllocationScope                     allocationScope);
//
//typedef void (*PFN_vkVoidFunction)(void);



//typedef VkResult (*PFN_vkCreateInstance)(const VkInstanceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkInstance* pInstance);
//typedef void (*PFN_vkDestroyInstance)(VkInstance instance, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkEnumeratePhysicalDevices)(VkInstance instance, u32* pPhysicalDeviceCount, VkPhysicalDevice* pPhysicalDevices);
//typedef void (*PFN_vkGetPhysicalDeviceFeatures)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceFeatures* pFeatures);
//typedef void (*PFN_vkGetPhysicalDeviceFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkFormatProperties* pFormatProperties);
//typedef VkResult (*PFN_vkGetPhysicalDeviceImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkImageTiling tiling, VkImageUsageFlags usage, VkImageCreateFlags flags, VkImageFormatProperties* pImageFormatProperties);
//typedef void (*PFN_vkGetPhysicalDeviceProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceProperties* pProperties);
//typedef void (*PFN_vkGetPhysicalDeviceQueueFamilyProperties)(VkPhysicalDevice physicalDevice, u32* pQueueFamilyPropertyCount, VkQueueFamilyProperties* pQueueFamilyProperties);
//typedef void (*PFN_vkGetPhysicalDeviceMemoryProperties)(VkPhysicalDevice physicalDevice, VkPhysicalDeviceMemoryProperties* pMemoryProperties);
//typedef PFN_vkVoidFunction (*PFN_vkGetInstanceProcAddr)(VkInstance instance, const char* pName);
//typedef PFN_vkVoidFunction (*PFN_vkGetDeviceProcAddr)(VkDevice device, const char* pName);
//typedef VkResult (*PFN_vkCreateDevice)(VkPhysicalDevice physicalDevice, const VkDeviceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDevice* pDevice);
//typedef void (*PFN_vkDestroyDevice)(VkDevice device, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkEnumerateInstanceExtensionProperties)(const char* pLayerName, u32* pPropertyCount, VkExtensionProperties* pProperties);
//typedef VkResult (*PFN_vkEnumerateDeviceExtensionProperties)(VkPhysicalDevice physicalDevice, const char* pLayerName, u32* pPropertyCount, VkExtensionProperties* pProperties);
//typedef VkResult (*PFN_vkEnumerateInstanceLayerProperties)(u32* pPropertyCount, VkLayerProperties* pProperties);
//typedef VkResult (*PFN_vkEnumerateDeviceLayerProperties)(VkPhysicalDevice physicalDevice, u32* pPropertyCount, VkLayerProperties* pProperties);
//typedef void (*PFN_vkGetDeviceQueue)(VkDevice device, u32 queueFamilyIndex, u32 queueIndex, VkQueue* pQueue);
//typedef VkResult (*PFN_vkQueueSubmit)(VkQueue queue, u32 submitCount, const VkSubmitInfo* pSubmits, VkFence fence);
//typedef VkResult (*PFN_vkQueueWaitIdle)(VkQueue queue);
//typedef VkResult (*PFN_vkDeviceWaitIdle)(VkDevice device);
//typedef VkResult (*PFN_vkAllocateMemory)(VkDevice device, const VkMemoryAllocateInfo* pAllocateInfo, const VkAllocationCallbacks* pAllocator, VkDeviceMemory* pMemory);
//typedef void (*PFN_vkFreeMemory)(VkDevice device, VkDeviceMemory memory, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkMapMemory)(VkDevice device, VkDeviceMemory memory, VkDeviceSize offset, VkDeviceSize size, VkMemoryMapFlags flags, void** ppData);
//typedef void (*PFN_vkUnmapMemory)(VkDevice device, VkDeviceMemory memory);
//typedef VkResult (*PFN_vkFlushMappedMemoryRanges)(VkDevice device, u32 memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);
//typedef VkResult (*PFN_vkInvalidateMappedMemoryRanges)(VkDevice device, u32 memoryRangeCount, const VkMappedMemoryRange* pMemoryRanges);
//typedef void (*PFN_vkGetDeviceMemoryCommitment)(VkDevice device, VkDeviceMemory memory, VkDeviceSize* pCommittedMemoryInBytes);
//typedef VkResult (*PFN_vkBindBufferMemory)(VkDevice device, VkBuffer buffer, VkDeviceMemory memory, VkDeviceSize memoryOffset);
//typedef VkResult (*PFN_vkBindImageMemory)(VkDevice device, VkImage image, VkDeviceMemory memory, VkDeviceSize memoryOffset);
//typedef void (*PFN_vkGetBufferMemoryRequirements)(VkDevice device, VkBuffer buffer, VkMemoryRequirements* pMemoryRequirements);
//typedef void (*PFN_vkGetImageMemoryRequirements)(VkDevice device, VkImage image, VkMemoryRequirements* pMemoryRequirements);
//typedef void (*PFN_vkGetImageSparseMemoryRequirements)(VkDevice device, VkImage image, u32* pSparseMemoryRequirementCount, VkSparseImageMemoryRequirements* pSparseMemoryRequirements);
//typedef void (*PFN_vkGetPhysicalDeviceSparseImageFormatProperties)(VkPhysicalDevice physicalDevice, VkFormat format, VkImageType type, VkSampleCountFlagBits samples, VkImageUsageFlags usage, VkImageTiling tiling, u32* pPropertyCount, VkSparseImageFormatProperties* pProperties);
//typedef VkResult (*PFN_vkQueueBindSparse)(VkQueue queue, u32 bindInfoCount, const VkBindSparseInfo* pBindInfo, VkFence fence);
//typedef VkResult (*PFN_vkCreateFence)(VkDevice device, const VkFenceCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFence* pFence);
//typedef void (*PFN_vkDestroyFence)(VkDevice device, VkFence fence, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkResetFences)(VkDevice device, u32 fenceCount, const VkFence* pFences);
//typedef VkResult (*PFN_vkGetFenceStatus)(VkDevice device, VkFence fence);
//typedef VkResult (*PFN_vkWaitForFences)(VkDevice device, u32 fenceCount, const VkFence* pFences, VkBool32 waitAll, u64 timeout);
//typedef VkResult (*PFN_vkCreateSemaphore)(VkDevice device, const VkSemaphoreCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSemaphore* pSemaphore);
//typedef void (*PFN_vkDestroySemaphore)(VkDevice device, VkSemaphore semaphore, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateEvent)(VkDevice device, const VkEventCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkEvent* pEvent);
//typedef void (*PFN_vkDestroyEvent)(VkDevice device, VkEvent event, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkGetEventStatus)(VkDevice device, VkEvent event);
//typedef VkResult (*PFN_vkSetEvent)(VkDevice device, VkEvent event);
//typedef VkResult (*PFN_vkResetEvent)(VkDevice device, VkEvent event);
//typedef VkResult (*PFN_vkCreateQueryPool)(VkDevice device, const VkQueryPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkQueryPool* pQueryPool);
//typedef void (*PFN_vkDestroyQueryPool)(VkDevice device, VkQueryPool queryPool, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkGetQueryPoolResults)(VkDevice device, VkQueryPool queryPool, u32 firstQuery, u32 queryCount, usize dataSize, void* pData, VkDeviceSize stride, VkQueryResultFlags flags);
//typedef VkResult (*PFN_vkCreateBuffer)(VkDevice device, const VkBufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBuffer* pBuffer);
//typedef void (*PFN_vkDestroyBuffer)(VkDevice device, VkBuffer buffer, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateBufferView)(VkDevice device, const VkBufferViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkBufferView* pView);
//typedef void (*PFN_vkDestroyBufferView)(VkDevice device, VkBufferView bufferView, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateImage)(VkDevice device, const VkImageCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImage* pImage);
//typedef void (*PFN_vkDestroyImage)(VkDevice device, VkImage image, const VkAllocationCallbacks* pAllocator);
//typedef void (*PFN_vkGetImageSubresourceLayout)(VkDevice device, VkImage image, const VkImageSubresource* pSubresource, VkSubresourceLayout* pLayout);
//typedef VkResult (*PFN_vkCreateImageView)(VkDevice device, const VkImageViewCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkImageView* pView);
//typedef void (*PFN_vkDestroyImageView)(VkDevice device, VkImageView imageView, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateShaderModule)(VkDevice device, const VkShaderModuleCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkShaderModule* pShaderModule);
//typedef void (*PFN_vkDestroyShaderModule)(VkDevice device, VkShaderModule shaderModule, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreatePipelineCache)(VkDevice device, const VkPipelineCacheCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineCache* pPipelineCache);
//typedef void (*PFN_vkDestroyPipelineCache)(VkDevice device, VkPipelineCache pipelineCache, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkGetPipelineCacheData)(VkDevice device, VkPipelineCache pipelineCache, usize* pDataSize, void* pData);
//typedef VkResult (*PFN_vkMergePipelineCaches)(VkDevice device, VkPipelineCache dstCache, u32 srcCacheCount, const VkPipelineCache* pSrcCaches);
//typedef VkResult (*PFN_vkCreateGraphicsPipelines)(VkDevice device, VkPipelineCache pipelineCache, u32 createInfoCount, const VkGraphicsPipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);
//typedef VkResult (*PFN_vkCreateComputePipelines)(VkDevice device, VkPipelineCache pipelineCache, u32 createInfoCount, const VkComputePipelineCreateInfo* pCreateInfos, const VkAllocationCallbacks* pAllocator, VkPipeline* pPipelines);
//typedef void (*PFN_vkDestroyPipeline)(VkDevice device, VkPipeline pipeline, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreatePipelineLayout)(VkDevice device, const VkPipelineLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkPipelineLayout* pPipelineLayout);
//typedef void (*PFN_vkDestroyPipelineLayout)(VkDevice device, VkPipelineLayout pipelineLayout, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateSampler)(VkDevice device, const VkSamplerCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkSampler* pSampler);
//typedef void (*PFN_vkDestroySampler)(VkDevice device, VkSampler sampler, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateDescriptorSetLayout)(VkDevice device, const VkDescriptorSetLayoutCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorSetLayout* pSetLayout);
//typedef void (*PFN_vkDestroyDescriptorSetLayout)(VkDevice device, VkDescriptorSetLayout descriptorSetLayout, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateDescriptorPool)(VkDevice device, const VkDescriptorPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkDescriptorPool* pDescriptorPool);
//typedef void (*PFN_vkDestroyDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkResetDescriptorPool)(VkDevice device, VkDescriptorPool descriptorPool, VkDescriptorPoolResetFlags flags);
//typedef VkResult (*PFN_vkAllocateDescriptorSets)(VkDevice device, const VkDescriptorSetAllocateInfo* pAllocateInfo, VkDescriptorSet* pDescriptorSets);
//typedef VkResult (*PFN_vkFreeDescriptorSets)(VkDevice device, VkDescriptorPool descriptorPool, u32 descriptorSetCount, const VkDescriptorSet* pDescriptorSets);
//typedef void (*PFN_vkUpdateDescriptorSets)(VkDevice device, u32 descriptorWriteCount, const VkWriteDescriptorSet* pDescriptorWrites, u32 descriptorCopyCount, const VkCopyDescriptorSet* pDescriptorCopies);
//typedef VkResult (*PFN_vkCreateFramebuffer)(VkDevice device, const VkFramebufferCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkFramebuffer* pFramebuffer);
//typedef void (*PFN_vkDestroyFramebuffer)(VkDevice device, VkFramebuffer framebuffer, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkCreateRenderPass)(VkDevice device, const VkRenderPassCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkRenderPass* pRenderPass);
//typedef void (*PFN_vkDestroyRenderPass)(VkDevice device, VkRenderPass renderPass, const VkAllocationCallbacks* pAllocator);
//typedef void (*PFN_vkGetRenderAreaGranularity)(VkDevice device, VkRenderPass renderPass, VkExtent2D* pGranularity);
//typedef VkResult (*PFN_vkCreateCommandPool)(VkDevice device, const VkCommandPoolCreateInfo* pCreateInfo, const VkAllocationCallbacks* pAllocator, VkCommandPool* pCommandPool);
//typedef void (*PFN_vkDestroyCommandPool)(VkDevice device, VkCommandPool commandPool, const VkAllocationCallbacks* pAllocator);
//typedef VkResult (*PFN_vkResetCommandPool)(VkDevice device, VkCommandPool commandPool, VkCommandPoolResetFlags flags);
//typedef VkResult (*PFN_vkAllocateCommandBuffers)(VkDevice device, const VkCommandBufferAllocateInfo* pAllocateInfo, VkCommandBuffer* pCommandBuffers);
//typedef void (*PFN_vkFreeCommandBuffers)(VkDevice device, VkCommandPool commandPool, u32 commandBufferCount, const VkCommandBuffer* pCommandBuffers);
//typedef VkResult (*PFN_vkBeginCommandBuffer)(VkCommandBuffer commandBuffer, const VkCommandBufferBeginInfo* pBeginInfo);
//typedef VkResult (*PFN_vkEndCommandBuffer)(VkCommandBuffer commandBuffer);
//typedef VkResult (*PFN_vkResetCommandBuffer)(VkCommandBuffer commandBuffer, VkCommandBufferResetFlags flags);
//typedef void (*PFN_vkCmdBindPipeline)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipeline pipeline);
//typedef void (*PFN_vkCmdSetViewport)(VkCommandBuffer commandBuffer, u32 firstViewport, u32 viewportCount, const VkViewport* pViewports);
//typedef void (*PFN_vkCmdSetScissor)(VkCommandBuffer commandBuffer, u32 firstScissor, u32 scissorCount, const VkRect2D* pScissors);
//typedef void (*PFN_vkCmdSetLineWidth)(VkCommandBuffer commandBuffer, float lineWidth);
//typedef void (*PFN_vkCmdSetDepthBias)(VkCommandBuffer commandBuffer, float depthBiasConstantFactor, float depthBiasClamp, float depthBiasSlopeFactor);
//typedef void (*PFN_vkCmdSetBlendConstants)(VkCommandBuffer commandBuffer, const float blendConstants[4]);
//typedef void (*PFN_vkCmdSetDepthBounds)(VkCommandBuffer commandBuffer, float minDepthBounds, float maxDepthBounds);
//typedef void (*PFN_vkCmdSetStencilCompareMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, u32 compareMask);
//typedef void (*PFN_vkCmdSetStencilWriteMask)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, u32 writeMask);
//typedef void (*PFN_vkCmdSetStencilReference)(VkCommandBuffer commandBuffer, VkStencilFaceFlags faceMask, u32 reference);
//typedef void (*PFN_vkCmdBindDescriptorSets)(VkCommandBuffer commandBuffer, VkPipelineBindPoint pipelineBindPoint, VkPipelineLayout layout, u32 firstSet, u32 descriptorSetCount, const VkDescriptorSet* pDescriptorSets, u32 dynamicOffsetCount, const u32* pDynamicOffsets);
//typedef void (*PFN_vkCmdBindIndexBuffer)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, VkIndexType indexType);
//typedef void (*PFN_vkCmdBindVertexBuffers)(VkCommandBuffer commandBuffer, u32 firstBinding, u32 bindingCount, const VkBuffer* pBuffers, const VkDeviceSize* pOffsets);
//typedef void (*PFN_vkCmdDraw)(VkCommandBuffer commandBuffer, u32 vertexCount, u32 instanceCount, u32 firstVertex, u32 firstInstance);
//typedef void (*PFN_vkCmdDrawIndexed)(VkCommandBuffer commandBuffer, u32 indexCount, u32 instanceCount, u32 firstIndex, i32 vertexOffset, u32 firstInstance);
//typedef void (*PFN_vkCmdDrawIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, u32 drawCount, u32 stride);
//typedef void (*PFN_vkCmdDrawIndexedIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset, u32 drawCount, u32 stride);
//typedef void (*PFN_vkCmdDispatch)(VkCommandBuffer commandBuffer, u32 x, u32 y, u32 z);
//typedef void (*PFN_vkCmdDispatchIndirect)(VkCommandBuffer commandBuffer, VkBuffer buffer, VkDeviceSize offset);
//typedef void (*PFN_vkCmdCopyBuffer)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkBuffer dstBuffer, u32 regionCount, const VkBufferCopy* pRegions);
//typedef void (*PFN_vkCmdCopyImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, u32 regionCount, const VkImageCopy* pRegions);
//typedef void (*PFN_vkCmdBlitImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, u32 regionCount, const VkImageBlit* pRegions, VkFilter filter);
//typedef void (*PFN_vkCmdCopyBufferToImage)(VkCommandBuffer commandBuffer, VkBuffer srcBuffer, VkImage dstImage, VkImageLayout dstImageLayout, u32 regionCount, const VkBufferImageCopy* pRegions);
//typedef void (*PFN_vkCmdCopyImageToBuffer)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkBuffer dstBuffer, u32 regionCount, const VkBufferImageCopy* pRegions);
//typedef void (*PFN_vkCmdUpdateBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize dataSize, const u32* pData);
//typedef void (*PFN_vkCmdFillBuffer)(VkCommandBuffer commandBuffer, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize size, u32 data);
//typedef void (*PFN_vkCmdClearColorImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearColorValue* pColor, u32 rangeCount, const VkImageSubresourceRange* pRanges);
//typedef void (*PFN_vkCmdClearDepthStencilImage)(VkCommandBuffer commandBuffer, VkImage image, VkImageLayout imageLayout, const VkClearDepthStencilValue* pDepthStencil, u32 rangeCount, const VkImageSubresourceRange* pRanges);
//typedef void (*PFN_vkCmdClearAttachments)(VkCommandBuffer commandBuffer, u32 attachmentCount, const VkClearAttachment* pAttachments, u32 rectCount, const VkClearRect* pRects);
//typedef void (*PFN_vkCmdResolveImage)(VkCommandBuffer commandBuffer, VkImage srcImage, VkImageLayout srcImageLayout, VkImage dstImage, VkImageLayout dstImageLayout, u32 regionCount, const VkImageResolve* pRegions);
//typedef void (*PFN_vkCmdSetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);
//typedef void (*PFN_vkCmdResetEvent)(VkCommandBuffer commandBuffer, VkEvent event, VkPipelineStageFlags stageMask);
//typedef void (*PFN_vkCmdWaitEvents)(VkCommandBuffer commandBuffer, u32 eventCount, const VkEvent* pEvents, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, u32 memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, u32 bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, u32 imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);
//typedef void (*PFN_vkCmdPipelineBarrier)(VkCommandBuffer commandBuffer, VkPipelineStageFlags srcStageMask, VkPipelineStageFlags dstStageMask, VkDependencyFlags dependencyFlags, u32 memoryBarrierCount, const VkMemoryBarrier* pMemoryBarriers, u32 bufferMemoryBarrierCount, const VkBufferMemoryBarrier* pBufferMemoryBarriers, u32 imageMemoryBarrierCount, const VkImageMemoryBarrier* pImageMemoryBarriers);
//typedef void (*PFN_vkCmdBeginQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, u32 query, VkQueryControlFlags flags);
//typedef void (*PFN_vkCmdEndQuery)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, u32 query);
//typedef void (*PFN_vkCmdResetQueryPool)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, u32 firstQuery, u32 queryCount);
//typedef void (*PFN_vkCmdWriteTimestamp)(VkCommandBuffer commandBuffer, VkPipelineStageFlagBits pipelineStage, VkQueryPool queryPool, u32 query);
//typedef void (*PFN_vkCmdCopyQueryPoolResults)(VkCommandBuffer commandBuffer, VkQueryPool queryPool, u32 firstQuery, u32 queryCount, VkBuffer dstBuffer, VkDeviceSize dstOffset, VkDeviceSize stride, VkQueryResultFlags flags);
//typedef void (*PFN_vkCmdPushConstants)(VkCommandBuffer commandBuffer, VkPipelineLayout layout, VkShaderStageFlags stageFlags, u32 offset, u32 size, const void* pValues);
//typedef void (*PFN_vkCmdBeginRenderPass)(VkCommandBuffer commandBuffer, const VkRenderPassBeginInfo* pRenderPassBegin, VkSubpassContents contents);
//typedef void (*PFN_vkCmdNextSubpass)(VkCommandBuffer commandBuffer, VkSubpassContents contents);
//typedef void (*PFN_vkCmdEndRenderPass)(VkCommandBuffer commandBuffer);
//typedef void (*PFN_vkCmdExecuteCommands)(VkCommandBuffer commandBuffer, u32 commandBufferCount, const VkCommandBuffer* pCommandBuffers);

