extern crate libc;

mod vk;

use vk::enums::*;
use vk::flags::*;
use vk::fns::*;
use vk::structs::*;
use vk::types::*;
use std::ptr;

fn main() {
	println!("Hello, Vulkan!");

	let instance_create_flags: VkInstanceCreateFlags = 0u32;
	
	//let applicationInfo: VkApplicationInfo = VkApplicationInfo { };
	
	let instance_create_info: VkInstanceCreateInfo = VkInstanceCreateInfo {
			// sType is the type of this structure.
			sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			// pNext is NULL or a pointer to an extension-specific structure.
			pNext: ptr::null(),
			// flags is reserved for future use.
			flags: instance_create_flags,
			// pApplicationInfo is NULL or a pointer to an instance of VkApplicationInfo.
			// If not NULL, this information helps implementations recognize behavior inherent to classes of applications.
			pApplicationInfo: ptr::null(),
			// enabledLayerCount is the number of global layers to enable.
			enabledLayerCount: 0,
			ppEnabledLayerNames: ptr::null(),
			enabledExtensionCount: 0,
			ppEnabledExtensionNames: ptr::null(),
			};
	
	let mut instance: VkInstance = 0;

	let allocation_callbacks_ptr: *const VkAllocationCallbacks = ptr::null();

	unsafe {
		{
			let result: VkResult = vkCreateInstance(&instance_create_info, allocation_callbacks_ptr, &mut instance);
			println!("Result: {}", result as u32);
		}
		println!("Instance handle: {}", instance as u32);
		
		vkDestroyInstance(instance, allocation_callbacks_ptr);
		
	}
	
}
