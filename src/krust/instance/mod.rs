extern crate libc;

use std::ptr;
//use vk::consts::*;
//use vk::enums::VkResult;
//use vk::enums::VkResult::VK_SUCCESS;
use vk::enums::*;
use vk::flags::*;
use vk::fns::*;
use vk::structs::*;
use vk::types::*;
use std::any::Any;
use std::vec::Vec;
use std::ffi::CString;
use libc::c_void;

pub struct Instance<'a> {
	pub handle: VkInstance,
	pub allocator: Option<&'a VkAllocationCallbacks>,
}


pub struct CreateInfo<'a> {
	pub next: Option<&'a Any>,
	pub flags: VkInstanceCreateFlags,
	pub application_info: Option<&'a VkApplicationInfo>,
	pub enabled_layer_names: &'a Vec<&'a str>,
	pub enabled_extension_names: &'a Vec<&'a str>,
}

impl<'a> Instance<'a> {
	
	pub fn create(create_info: &CreateInfo, allocator: Option<&'a VkAllocationCallbacks>) -> Result<Self, VkResult> {
		
		// convert enabled layer names into null terminated utf-8-strings for ffi
		let mut enabled_layer_names: Vec<*const u8> = Vec::with_capacity(create_info.enabled_layer_names.len());
		for enabled_layer_name in create_info.enabled_layer_names {
			match CString::new(*enabled_layer_name) {
				Ok(name_as_cstring) => enabled_layer_names.push(name_as_cstring.as_ptr() as *const u8),
				Err(nul_error) => panic!(nul_error), // be more descriptive, even though this error cannot occur
			}
		}
		
		// convert enabled extension names into null terminated utf-8-strings for ffi
		let mut enabled_extension_names: Vec<*const u8> = Vec::with_capacity(create_info.enabled_extension_names.len());
		for enabled_extension_name in create_info.enabled_extension_names {
			match CString::new(*enabled_extension_name) {
				Ok(name_as_cstring) => enabled_extension_names.push(name_as_cstring.as_ptr() as *const u8),
				Err(nul_error) => panic!(nul_error), // be more descriptive, even though this error cannot occur
			}
		}
		
		let vk_create_info = VkInstanceCreateInfo {
			// sType is the type of this structure.
			sType: VkStructureType::VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
			// pNext is NULL or a pointer to an extension-specific structure.
			pNext: match create_info.next { Some(some) => some as *const Any as *const c_void, None => ptr::null::<c_void>()},
			// flags is reserved for future use.
			flags: create_info.flags,
			// pApplicationInfo is NULL or a pointer to an instance of VkApplicationInfo.
			// If not NULL, this information helps implementations recognize behavior inherent to classes of applications.
			pApplicationInfo: match create_info.application_info { Some(some) => some, None => ptr::null() },
			// enabledLayerCount is the number of global layers to enable.
			enabledLayerCount: enabled_layer_names.len() as u32,
			ppEnabledLayerNames: enabled_layer_names.as_ptr(),
			enabledExtensionCount: enabled_extension_names.len() as u32,
			ppEnabledExtensionNames: enabled_extension_names.as_ptr(),
			};
		
		let optional_allocator: *const VkAllocationCallbacks = match allocator { Some(some) => some, None => ptr::null() };
		
		let mut instance: VkInstance = 0;
		
		let result = unsafe { vkCreateInstance(&vk_create_info, optional_allocator, &mut instance) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }

		Ok(
			Instance {
				handle: instance,
				allocator: allocator,
			}
		)
	}
	
    pub fn enumerate_physical_devices(&mut self) -> Result<Vec<VkPhysicalDevice>, VkResult> {
    	
   		let mut device_count: u32 = 0;
    	
    	// query number of physical devices available to this instance
	    let result = unsafe { vkEnumeratePhysicalDevices(self.handle, &mut device_count, ptr::null_mut()) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
		
		// create result buffer
		let mut devices = Vec::with_capacity(device_count as usize);
		devices.resize(device_count as usize, 0);
		
		// fill the result buffer
	    let result = unsafe { vkEnumeratePhysicalDevices(self.handle, &mut device_count, devices.as_mut_ptr()) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
	    
		devices.truncate(device_count as usize);
		Ok(devices)
    }

	#[allow(dead_code, unused_variables)]
	pub fn get_proc_addr(name: &str) -> ! {
		//pub fn vkGetInstanceProcAddr(instance: VkInstance, pName: *const u8) -> PFN_vkVoidFunction;
		unimplemented!()
	}

}

impl<'a> Drop for Instance<'a> {
	
	fn drop(&mut self) {
		unsafe {
			let optional_allocator: *const VkAllocationCallbacks = match self.allocator { Some(some) => some, None => ptr::null() };
			vkDestroyInstance(self.handle, optional_allocator);
		}
	}
	
}
