use super::types::*;

//#define VK_LOD_CLAMP_NONE 1000.0f
#[allow(dead_code)]
pub const VK_LOD_CLAMP_NONE: f32 = 1000.0f32;

//#define VK_REMAINING_MIP_LEVELS (~0U)
#[allow(dead_code)]
pub const VK_REMAINING_MIP_LEVELS: u32 = !0u32;

//#define VK_REMAINING_ARRAY_LAYERS (~0U)
#[allow(dead_code)]
pub const VK_REMAINING_ARRAY_LAYERS: u32 = !0u32;

//#define VK_WHOLE_SIZE (~0ULL)
#[allow(dead_code)]
pub const VK_WHOLE_SIZE: VkDeviceSize = !0u64;

//#define VK_ATTACHMENT_UNUSED (~0U)
#[allow(dead_code)]
pub const VK_ATTACHMENT_UNUSED: u32 = !0u32; // used as alternative null pointer

//#define VK_TRUE 1
#[allow(dead_code)]
pub const VK_TRUE: VkBool32 = 1;

//#define VK_FALSE 0
#[allow(dead_code)]
pub const VK_FALSE: VkBool32 = 0;

//#define VK_QUEUE_FAMILY_IGNORED (~0U)
#[allow(dead_code)]
pub const VK_QUEUE_FAMILY_IGNORED: u32 = !0u32;

//#define VK_SUBPASS_EXTERNAL (~0U)
#[allow(dead_code)]
pub const VK_SUBPASS_EXTERNAL: u32 = !0u32;

//#define VK_MAX_PHYSICAL_DEVICE_NAME_SIZE 256
#[allow(dead_code)]
pub const VK_MAX_PHYSICAL_DEVICE_NAME_SIZE: usize = 256;

//#define VK_UUID_SIZE 16
#[allow(dead_code)]
pub const VK_UUID_SIZE: usize = 16;

//#define VK_MAX_MEMORY_TYPES 32
#[allow(dead_code)]
pub const VK_MAX_MEMORY_TYPES: usize = 32;

//#define VK_MAX_MEMORY_HEAPS 16
#[allow(dead_code)]
pub const VK_MAX_MEMORY_HEAPS: usize = 16;

//#define VK_MAX_EXTENSION_NAME_SIZE 256
#[allow(dead_code)]
pub const VK_MAX_EXTENSION_NAME_SIZE: usize = 256;

//#define VK_MAX_DESCRIPTION_SIZE 256
#[allow(dead_code)]
pub const VK_MAX_DESCRIPTION_SIZE: usize = 256;
