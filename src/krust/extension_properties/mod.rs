
use vk::structs::*;

#[derive(Debug)]
pub struct ExtensionProperties {

	/// extensionName is a null-terminated string specifying the name of the extension.
	pub extension_name: String,
	
	/// specVersion is the version of this extension. It is an integer, incremented with backward compatible changes.
	pub spec_version: u32,
	
}

impl<'a> From<&'a VkExtensionProperties> for ExtensionProperties {
	
	fn from(extension_properties: &VkExtensionProperties) -> Self {
		
		// TODO there has to be a std-solution somewhere
		let extension_name = {
			let &ExtensionNameSlice(extension_name_bytes) = &extension_properties.extensionName;
			let extension_name_len = extension_name_bytes.iter().position(|&c| c == 0).unwrap_or(extension_name_bytes.len());
			String::from_utf8_lossy(&extension_name_bytes[0..extension_name_len]).into_owned()
		};
		
		ExtensionProperties {
			extension_name: extension_name,
			spec_version: extension_properties.specVersion,
		}
	}
	
}

impl ExtensionProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		println!("{}extension_name: {:?}", prefix, self.extension_name);
		println!("{}extension_name: {:?}", prefix, self.extension_name);
	}
}
