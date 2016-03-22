
use krust::ApiVersion;

use vk::structs::*;

#[derive(Debug)]
pub struct LayerProperties {

	/// layerName is a null-terminated UTF-8 string specifying the name of the layer. Use this name in the
	/// ppEnabledLayerNames array passed in the VkInstanceCreateInfo and VkDeviceCreateInfo structures
	/// passed to vkCreateInstance and vkCreateDevice, respectively, to enable this layer for an instance or device.
	pub layer_name: String,
	
	/// apiVersion is the Vulkan version the layer was written to, encoded as described in the API Version Numbers and
	/// Semantics section.
	pub spec_version: ApiVersion,
	
	/// implementationVersion is the version of this layer. It is an integer, increasing with backward compatible changes.
	pub implementation_version: u32,
	
	/// description is a null-terminated UTF-8 string providing additional details that can be used by the application to
	/// identify the layer.
	pub description: String,
	
}

impl<'a> From<&'a VkLayerProperties> for LayerProperties {
	
	fn from(queue_family_properties: &VkLayerProperties) -> Self {
		
		// TODO there has to be a std-solution somewhere
		let layer_name = {
			let &VkLayerProperties_LayerNameSlice(layer_name_bytes) = &queue_family_properties.layerName;
			let layer_name_len = layer_name_bytes.iter().position(|&c| c == 0).unwrap_or(layer_name_bytes.len());
			String::from_utf8_lossy(&layer_name_bytes[0..layer_name_len]).into_owned()
		};

		// TODO there has to be a std-solution somewhere
		let description = {
			let &VkLayerProperties_DescriptionSlice(description_bytes) = &queue_family_properties.description;
			let description_len = description_bytes.iter().position(|&c| c == 0).unwrap_or(description_bytes.len());
			String::from_utf8_lossy(&description_bytes[0..description_len]).into_owned()
		};
		
		LayerProperties {
			layer_name: layer_name,
			spec_version: ApiVersion::from_raw(queue_family_properties.specVersion),
			implementation_version: queue_family_properties.implementationVersion,
			description: description,
		}
	}
	
}

impl LayerProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}layer_name: {:?}", prefix, self.layer_name);
		println!("{}spec_version: {:?}", prefix, self.spec_version);
		println!("{}implementation_version: {:?}", prefix, self.implementation_version);
		println!("{}description: {:?}", prefix, self.description);
	}
}