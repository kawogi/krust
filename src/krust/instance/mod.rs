//! There is no global state in Vulkan and all per-application state is stored in a Instance object.
//! Creating a Instance object initializes the Vulkan library and allows the application to pass information about
//! itself to the implementation.

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

use krust::physical_device::PhysicalDevice;

pub struct Instance<'a> {
	pub handle: VkInstance,
	pub allocator: Option<&'a VkAllocationCallbacks>,
}


/// Controls the creation of an Instance
pub struct CreateInfo<'a> {
	/// None() or a pointer to an extension-specific structure.
	pub next: Option<&'a Any>,
	/// reserved for future use.
	pub flags: VkInstanceCreateFlags,
	/// None() or a pointer to an instance of VkApplicationInfo.
	/// If Some(), this information helps implementations recognize behavior inherent to classes of applications.
	pub application_info: Option<&'a VkApplicationInfo>,
	/// a (possibly empty) array of strings containing the names of layers to enable for the created instance.
	/// See the Layers section for further details
	pub enabled_layer_names: &'a [&'a str],
	/// a (possibly empty) array of strings containing the names of extensions to enable.
	pub enabled_extension_names: &'a [&'a str],
}

impl<'a> CreateInfo<'a> {
	
	pub fn default() -> CreateInfo<'a> {
		CreateInfo {
			next: Option::None,
			flags: 0,
			application_info: Option::None,
			enabled_layer_names: &[],
			enabled_extension_names: &[],
		}
	}
	
	#[allow(dead_code)]
	pub fn for_application_info(application_info: &'a VkApplicationInfo) -> CreateInfo<'a> {
		CreateInfo {
			application_info: Option::Some(application_info),
			.. CreateInfo::default()
		}
	}
	
}

impl<'a> Instance<'a> {
	
	/// Creates an Instance
	///
	/// create_info points to an instance of instance::CreateInfo controlling creation of the instance.
	///
	/// allocator (optional) controls host memory allocation as described in the Memory Allocation chapter.
	///
	/// returns a new Instance holding the per-application state
	///
	/// # Errors
	///
	/// - VK_ERROR_OUT_OF_HOST_MEMORY
	/// - VK_ERROR_OUT_OF_DEVICE_MEMORY
	/// - VK_ERROR_INITIALIZATION_FAILED
	/// - VK_ERROR_LAYER_NOT_PRESENT
	/// - VK_ERROR_EXTENSION_NOT_PRESENT
	/// - VK_ERROR_INCOMPATIBLE_DRIVER
	///
	pub fn create(create_info: &CreateInfo, allocator: Option<&'a VkAllocationCallbacks>) -> Result<Self, VkResult> {
		
		// converts str into null terminated utf-8-string
	    let to_cstring = |s: &str| -> CString { CString::new(s).unwrap() };
	    // retrieves the raw pointer of a CString
	    let to_raw_ptr = |s: &CString| -> *const u8 { s.as_ptr() as *const u8 };
		
		// convert enabled layer names into null terminated utf-8-strings for ffi
	    let enabled_layer_names_cstrings = create_info.enabled_layer_names.iter().map(|s| to_cstring(s)).collect::<Vec<CString>>();
	    let enabled_layer_names_raw_ptrs = enabled_layer_names_cstrings.iter().map(|s| to_raw_ptr(s)).collect::<Vec<*const u8>>();
		
		// convert enabled extension names into null terminated utf-8-strings for ffi
	    let enabled_extension_names_cstrings = create_info.enabled_extension_names.iter().map(|s| to_cstring(s)).collect::<Vec<CString>>();
	    let enabled_extension_names_raw_ptrs = enabled_extension_names_cstrings.iter().map(|s| to_raw_ptr(s)).collect::<Vec<*const u8>>();

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
			enabledLayerCount: enabled_layer_names_raw_ptrs.len() as u32,
			ppEnabledLayerNames: enabled_layer_names_raw_ptrs.as_ptr(),
			enabledExtensionCount: enabled_extension_names_raw_ptrs.len() as u32,
			ppEnabledExtensionNames: enabled_extension_names_raw_ptrs.as_ptr(),
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
	
    pub fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, VkResult> {
    	
   		let mut device_count: u32 = 0;
    	
    	// query number of physical devices available to this instance
	    let result = unsafe { vkEnumeratePhysicalDevices(self.handle, &mut device_count, ptr::null_mut()) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
		
		// create result buffer
		let mut device_handles = Vec::with_capacity(device_count as usize);
		device_handles.resize(device_count as usize, 0);
		
		// fill the result buffer
	    let result = unsafe { vkEnumeratePhysicalDevices(self.handle, &mut device_count, device_handles.as_mut_ptr()) };
	    if result != VkResult::VK_SUCCESS { return Err(result); }
	    
		device_handles.truncate(device_count as usize);
		
		let devices = device_handles.iter().map(|&handle| PhysicalDevice::create(handle)).collect::<Vec<PhysicalDevice>>();
		
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
