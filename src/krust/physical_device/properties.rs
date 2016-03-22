use super::limits::PhysicalDeviceLimits;
use super::sparse_properties::PhysicalDeviceSparseProperties;

use vk::consts::*;
use vk::enums::*;
use vk::structs::*;

use krust::ApiVersion;

#[derive(Debug)]
pub struct PhysicalDeviceProperties {
	///  the version of Vulkan supported by the device, encoded as described in the API Version Numbers and Semantics section.
	pub api_version: ApiVersion, // apiVersion: 0u32,
	
	/// the vendor-specified version of the driver.
	pub driver_version: u32, // driverVersion: 0u32,
	
	/// a unique identifier for the vendor (see below) of the physical device.
	pub vendor_id: u32, // vendorID: 0u32,
	
	/// a unique identifier for the physical device among devices available from the vendor.
	pub device_id: u32, // deviceID: 0u32,
	
	/// a VkPhysicalDeviceType specifying the type of device.
	pub device_type: VkPhysicalDeviceType, // deviceType: VkPhysicalDeviceType::VK_PHYSICAL_DEVICE_TYPE_OTHER,
	
	/// a string containing the name of the device.
	pub device_name: String, // deviceName: [b'\0' as libc::c_uchar; VK_MAX_PHYSICAL_DEVICE_NAME_SIZE],
	
	/// an array of size VK_UUID_SIZE, containing 8-bit values that represent a universally unique identifier for the device.
	pub pipeline_cache_uuid: [u8; VK_UUID_SIZE], // pipelineCacheUUID: [0u8; VK_UUID_SIZE],
	
	pub limits: PhysicalDeviceLimits,

	pub sparse_properties: PhysicalDeviceSparseProperties,
}

impl From<VkPhysicalDeviceProperties> for PhysicalDeviceProperties {
	
	fn from(properties: VkPhysicalDeviceProperties) -> Self {

		// TODO there has to be a std-solution somewhere
		let device_name = {
			let &VkPhysicalDeviceProperties_DeviceNameSlice(device_name_bytes) = &properties.deviceName;
			let device_name_len = device_name_bytes.iter().position(|&c| c == 0).unwrap_or(device_name_bytes.len());
			String::from_utf8_lossy(&device_name_bytes[0..device_name_len]).into_owned()
		};

		PhysicalDeviceProperties {
			api_version: ApiVersion::from_raw(properties.apiVersion),
			driver_version: properties.driverVersion,
			vendor_id: properties.vendorID,
			device_id: properties.deviceID,
			device_type: properties.deviceType,
			device_name: device_name,
			pipeline_cache_uuid: properties.pipelineCacheUUID,
			limits: PhysicalDeviceLimits::from(properties.limits),
			sparse_properties: PhysicalDeviceSparseProperties::from(properties.sparseProperties),
		}
	}
	
}

impl PhysicalDeviceProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		
		let mut indent: String = String::from(prefix);
		indent.push('\t');
		
		println!("{}api_version: {:?}", prefix, self.api_version);
		println!("{}driver_version: {:?}", prefix, self.driver_version);
		println!("{}vendor_id: {:?}", prefix, self.vendor_id);
		println!("{}device_id: {:?}", prefix, self.device_id);
		println!("{}device_type: {:?}", prefix, self.device_type);
		println!("{}device_name: {:?}", prefix, self.device_name);
		println!("{}pipeline_cache_uuid: {:?}", prefix, self.pipeline_cache_uuid);

		println!("{}limits: {{", prefix);
		self.limits.dump(&indent);
		println!("{}}}", prefix);

		println!("{}sparse_properties: {{", prefix);
		self.sparse_properties.dump(&indent);
		println!("{}}}", prefix);
	}
}
