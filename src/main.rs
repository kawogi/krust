#[macro_use]
extern crate bitflags;

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

	let instance_create_info = instance::CreateInfo::default();
	let allocator = Option::None;
	let instance: Instance = Instance::create(&instance_create_info, allocator).unwrap();
	println!("Instance handle: {}", instance.handle as u32);

	let physical_devices = instance.enumerate_physical_devices().unwrap();

	for physical_device in physical_devices.iter() {
		println!("Physical device: {:?}", physical_device);
	}

}
