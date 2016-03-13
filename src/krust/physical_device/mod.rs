extern crate libc;

mod properties;
mod limits;
mod sparse_properties;
mod features;
mod queue_family_properties;

pub use self::properties::PhysicalDeviceProperties;
pub use self::limits::PhysicalDeviceLimits;
pub use self::sparse_properties::PhysicalDeviceSparseProperties;
pub use self::features::PhysicalDeviceFeatures;
pub use self::queue_family_properties::QueueFamilyProperties;

use std::ptr;
use std::vec::Vec;

use vk::fns::*;
use vk::structs::*;
use vk::types::*;


#[derive(Debug)]
pub struct PhysicalDevice {
	handle: VkPhysicalDevice,
	
	pub properties: PhysicalDeviceProperties,
	
	pub features: PhysicalDeviceFeatures,
	
	pub queue_family_properties: Vec<QueueFamilyProperties>,
}


impl PhysicalDevice {
	
	pub fn create(handle: VkPhysicalDevice) -> Self {
		
		let mut properties = VkPhysicalDeviceProperties::default();
		unsafe { vkGetPhysicalDeviceProperties(handle, &mut properties) };

		let mut features = VkPhysicalDeviceFeatures::default(); 
		unsafe { vkGetPhysicalDeviceFeatures(handle, &mut features) };
		
		PhysicalDevice {
			handle: handle,
			properties: PhysicalDeviceProperties::from(properties),
			features: PhysicalDeviceFeatures::from(features),
			queue_family_properties: Self::get_queue_family_properties(handle),
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

	}
}
