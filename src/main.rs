#[macro_use]
extern crate bitflags;
extern crate libc;

pub mod vk;
pub mod krust;

use std::option::Option;

use krust::instance::Instance;
use krust::instance;

fn main() {
	println!("Hello, Vulkan!");

	let instance_create_info = instance::CreateInfo::default();
	let allocator = Option::None;
	let instance: Instance = Instance::create(&instance_create_info, allocator).unwrap();
	println!("Instance handle: {:?}", instance.handle);

	let physical_devices = instance.enumerate_physical_devices().unwrap();

	for physical_device in &physical_devices {
		println!("PhysicalDevice {{");
		physical_device.dump("\t");
		println!("}}");
	}

}
