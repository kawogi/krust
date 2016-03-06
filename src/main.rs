extern crate libc;

mod vk;
mod krust;

//use vk::enums::*;
//use vk::flags::*;
//use vk::fns::*;
//use vk::structs::*;
//use vk::types::*;
//use std::ptr;
use std::option::Option;

use krust::instance::Instance;
use krust::instance;

fn main() {
	println!("Hello, Vulkan!");

	let instance_create_info = instance::CreateInfo {
			next: Option::None,
			flags: 0,
			application_info: Option::None,
			enabled_layer_names: &vec![],
			enabled_extension_names: &vec![],
			};

	let allocator = Option::None;
	
	let mut instance: Instance = Instance::create(&instance_create_info, allocator).unwrap();
	println!("Instance handle: {}", instance.handle as u32);

	for physical_device in &instance.enumerate_physical_devices().unwrap() {
		println!("Physical device handle: {}", *physical_device as u32);
	}

}
