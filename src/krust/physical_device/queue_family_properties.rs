
use vk::flags::*;
use vk::structs::*;

#[derive(Debug)]
pub struct QueueFamilyProperties {
	
	/// if true, then the queues in this queue family support graphics operations.
	is_graphics: bool,
	/// if true, then the queues in this queue family support compute operations.
	is_compute: bool,
	/// if true, then the queues in this queue family support transfer operations.
	is_transfer: bool,
	/// if true, then the queues in this queue family support sparse memory management operations (see Sparse Resources). If any of the sparse resource features are enabled, then at least one queue family must support this bit.	
	is_sparse_binding: bool,
	
	/// the unsigned integer count of queues in this queue family.
	queue_count: u32,
	
	/// the unsigned integer count of meaningful bits in the timestamps written via vkCmdWriteTimestamp. The valid range for the count is 36..64 bits, or a value of 0, indicating no support for timestamps. Bits outside the valid range are guaranteed to be zeros.
	timestamp_valid_bits: u32,
	
	/// the minimum granularity supported for image transfer operations on the queues in this queue family.
	min_image_transfer_granularity: [u32; 3],
}

impl<'a> From<&'a VkQueueFamilyProperties> for QueueFamilyProperties {
	
	fn from(queue_family_properties: &VkQueueFamilyProperties) -> Self {
		QueueFamilyProperties {
			is_graphics: queue_family_properties.queueFlags.contains(VK_QUEUE_GRAPHICS_BIT),
			is_compute: queue_family_properties.queueFlags.contains(VK_QUEUE_COMPUTE_BIT),
			is_transfer: queue_family_properties.queueFlags.contains(VK_QUEUE_TRANSFER_BIT),
			is_sparse_binding: queue_family_properties.queueFlags.contains(VK_QUEUE_SPARSE_BINDING_BIT),
			queue_count: queue_family_properties.queueCount,
			timestamp_valid_bits: queue_family_properties.timestampValidBits,
			min_image_transfer_granularity: [queue_family_properties.minImageTransferGranularity.width, queue_family_properties.minImageTransferGranularity.height, queue_family_properties.minImageTransferGranularity.depth],
		}
	}
	
}

impl QueueFamilyProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}is_graphics: {:?}", prefix, self.is_graphics);
		println!("{}is_compute: {:?}", prefix, self.is_compute);
		println!("{}is_transfer: {:?}", prefix, self.is_transfer);
		println!("{}is_sparse_binding: {:?}", prefix, self.is_sparse_binding);
		println!("{}queue_count: {:?}", prefix, self.queue_count);
		println!("{}timestamp_valid_bits: {:?}", prefix, self.timestamp_valid_bits);
		println!("{}min_image_transfer_granularity: {:?}", prefix, self.min_image_transfer_granularity);
	}
}