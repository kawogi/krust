extern crate libc;

mod features;
mod limits;
mod memory_properties;
mod properties;
mod sparse_properties;

pub use self::features::PhysicalDeviceFeatures;
pub use self::limits::PhysicalDeviceLimits;
pub use self::memory_properties::PhysicalDeviceMemoryProperties;
pub use self::properties::PhysicalDeviceProperties;
pub use self::sparse_properties::PhysicalDeviceSparseProperties;

use krust::extension_properties::ExtensionProperties;
use krust::sparse_image_format_properties::SparseImageFormatProperties;
use krust::queue_family_properties::QueueFamilyProperties;
use krust::layer_properties::LayerProperties;

use std::ptr;
use std::vec::Vec;

use vk::fns::*;
use vk::structs::*;
use vk::types::*;
use vk::enums::*;
use vk::flags::*;


#[derive(Debug)]
pub struct PhysicalDevice {
	handle: VkPhysicalDevice,
	
	pub properties: PhysicalDeviceProperties,
	
	pub features: PhysicalDeviceFeatures,
	
	pub queue_family_properties: Vec<QueueFamilyProperties>,
	
	pub memory_properties: PhysicalDeviceMemoryProperties,
	
	pub layer_properties: Vec<LayerProperties>,
}


impl PhysicalDevice {
	
	pub fn create(handle: VkPhysicalDevice) -> Self {
		
		let mut properties = VkPhysicalDeviceProperties::default();
		unsafe { vkGetPhysicalDeviceProperties(handle, &mut properties) };

		let mut features = VkPhysicalDeviceFeatures::default(); 
		unsafe { vkGetPhysicalDeviceFeatures(handle, &mut features) };
		
		let mut memory_properties = VkPhysicalDeviceMemoryProperties::default(); 
		unsafe { vkGetPhysicalDeviceMemoryProperties(handle, &mut memory_properties) };
		
		PhysicalDevice {
			handle: handle,
			properties: PhysicalDeviceProperties::from(properties),
			features: PhysicalDeviceFeatures::from(features),
			queue_family_properties: Self::get_queue_family_properties(handle),
			memory_properties: PhysicalDeviceMemoryProperties::from(&memory_properties),
			layer_properties: Self::get_layer_properties(handle),
		}
	}

	// pub fn vkCreateDevice(physicalDevice: VkPhysicalDevice, pCreateInfo: *const VkDeviceCreateInfo, pAllocator: *const VkAllocationCallbacks, pDevice: *mut VkDevice) -> VkResult;

	// pub fn vkGetPhysicalDeviceFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, pFormatProperties: *mut VkFormatProperties);
	#[allow(dead_code)]
	// TODO change to result type FormatProperties
	pub fn get_format_properties(&self, format: VkFormat) -> VkFormatProperties {
		let mut format_properties = VkFormatProperties::default(); 
		unsafe { vkGetPhysicalDeviceFormatProperties(self.handle, format, &mut format_properties) };
		
		format_properties
	}
	
	// pub fn vkGetPhysicalDeviceImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags, pImageFormatProperties: *mut VkImageFormatProperties) -> VkResult;
	#[allow(dead_code)]
	// TODO change to result type ImageFormatProperties
	pub fn get_image_format_properties(&self, format: VkFormat, type_: VkImageType, tiling: VkImageTiling, usage: VkImageUsageFlags, flags: VkImageCreateFlags) -> Result<VkImageFormatProperties, VkResult> {
		let mut image_format_properties = VkImageFormatProperties::default();
		let result = unsafe { vkGetPhysicalDeviceImageFormatProperties(self.handle, format, type_, tiling, usage, flags, &mut image_format_properties) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
		
		Ok(image_format_properties)
	}
	
	// pub fn vkGetPhysicalDeviceSparseImageFormatProperties(physicalDevice: VkPhysicalDevice, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlagBits, usage: VkImageUsageFlags, tiling: VkImageTiling, pPropertyCount: *mut u32, pProperties: *mut VkSparseImageFormatProperties);
	#[allow(dead_code)]
	pub fn get_sparse_image_format_properties(&self, format: VkFormat, type_: VkImageType, samples: VkSampleCountFlags, usage: VkImageUsageFlags, tiling: VkImageTiling) -> Vec<SparseImageFormatProperties> {
   		let mut sparse_image_format_properties_count: u32 = 0;
    	
    	// query number of physical devices available to this instance
	    unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(self.handle, format, type_, samples, usage, tiling, &mut sparse_image_format_properties_count, ptr::null_mut()) };
		
		// create result buffer
		let mut raw_sparse_image_format_properties = Vec::with_capacity(sparse_image_format_properties_count as usize);
		raw_sparse_image_format_properties.resize(sparse_image_format_properties_count as usize, VkSparseImageFormatProperties::default());
		
		// fill the result buffer
	    unsafe { vkGetPhysicalDeviceSparseImageFormatProperties(self.handle, format, type_, samples, usage, tiling, &mut sparse_image_format_properties_count, raw_sparse_image_format_properties.as_mut_ptr()) };
	    
		raw_sparse_image_format_properties.truncate(sparse_image_format_properties_count as usize);
		
		raw_sparse_image_format_properties.iter().map(|item| SparseImageFormatProperties::from(item)).collect::<Vec<SparseImageFormatProperties>>()
	}
	
	// pub fn vkEnumerateDeviceExtensionProperties(physicalDevice: VkPhysicalDevice, pLayerName: *const u8, pPropertyCount: *mut u32, pProperties: *mut VkExtensionProperties) -> VkResult;
	#[allow(dead_code)]
	// TODO accept layer_name as String
	pub fn get_extension_properties(&self, layer_name: *const u8) -> Result<Vec<ExtensionProperties>, VkResult> {
   		let mut extension_properties_count: u32 = 0;
    	
    	// query number of physical devices available to this instance
	    let result = unsafe { vkEnumerateDeviceExtensionProperties(self.handle, layer_name, &mut extension_properties_count, ptr::null_mut()) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
		
		// create result buffer
		let mut raw_extension_properties = Vec::with_capacity(extension_properties_count as usize);
		raw_extension_properties.resize(extension_properties_count as usize, VkExtensionProperties::default());
		
		// fill the result buffer
	    let result = unsafe { vkEnumerateDeviceExtensionProperties(self.handle, layer_name, &mut extension_properties_count, raw_extension_properties.as_mut_ptr()) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
	    
		raw_extension_properties.truncate(extension_properties_count as usize);
		
		Ok(raw_extension_properties.iter().map(|item| ExtensionProperties::from(item)).collect::<Vec<ExtensionProperties>>())
	}
	


    fn get_layer_properties(handle: VkPhysicalDevice) -> Vec<LayerProperties> {
    	
   		let mut device_layer_property_count: u32 = 0;
    	
    	// query number of properties available to this device
		unsafe { vkEnumerateDeviceLayerProperties(handle, &mut device_layer_property_count, ptr::null_mut()) };
		
		// create result buffer
		let mut layer_properties = Vec::with_capacity(device_layer_property_count as usize);
		layer_properties.resize(device_layer_property_count as usize, VkLayerProperties::default());
		
		// fill the result buffer
		unsafe { vkEnumerateDeviceLayerProperties(handle, &mut device_layer_property_count, layer_properties.as_mut_ptr()) };
	    
		layer_properties.truncate(device_layer_property_count as usize);
		
		layer_properties.iter().map(|item| LayerProperties::from(item)).collect::<Vec<LayerProperties>>()
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
		
		queue_family_properties.iter().map(|item| QueueFamilyProperties::from(item)).collect::<Vec<QueueFamilyProperties>>()
    }

	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		
		let mut indent: String = String::from(prefix);
		indent.push('\t');
		
		println!("{}handle: {:?}", prefix, self.handle);

		println!("{}properties: {{", prefix);
		self.properties.dump(&indent);
		println!("{}}}", prefix);
		
		println!("{}features: {{", prefix);
		self.features.dump(&indent);
		println!("{}}}", prefix);
		
		for properties in &self.queue_family_properties {
			println!("{}queue_family_properties[]: {{", prefix);
			properties.dump(&indent);
			println!("{}}}", prefix);
		}

		println!("{}memory_properties: {{", prefix);
		self.memory_properties.dump(&indent);
		println!("{}}}", prefix);

		for properties in &self.layer_properties {
			println!("{}layer_properties[]: {{", prefix);
			properties.dump(&indent);
			println!("{}}}", prefix);
		}

	}
}
