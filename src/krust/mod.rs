pub mod instance;
pub mod physical_device;


// TODO make proper comment
//The Vulkan version number is used in several places in the API. In each such use, the API major version number, minor
//version number, and patch version number are packed into a 32-bit integer as follows:
//• The major version number is a 10-bit integer packed into bits 31-22.
//• The minor version number is a 10-bit integer packed into bits 21-12.
//• The patch version number is a 12-bit integer packed into bits 11-0.
//Differences in any of the Vulkan version numbers indicates a change to the API in some way, with each part of the
//version number indicating a different scope of changes.
//A difference in patch version numbers indicates that some usually small aspect of the specification or header has been
//modified, typically to fix a bug, and may have an impact on the behavior of existing functionality. Differences in this
//version number should not affect either full compatibility or backwards compatibility between two versions, or add
//additional interfaces to the API.
//A difference in minor version numbers indicates that some amount of new functionality has been added. This will
//usually include new interfaces in the header, and may also include behavior changes and bug fixes. Functionality may be
//deprecated in a minor revision, but will not be removed. When a new minor version is introduced, the patch version is
//reset to 0, and each minor revision maintains its own set of patch versions. Differences in this version should not affect
//backwards compatibility, but will affect full compatibility.
//A difference in major version numbers indicates a large set of changes to the API, potentially including new functionality
//and header interfaces, behavioral changes, removal of deprecated features, modification or outright replacement of any
//feature, and is thus very likely to break any and all compatibility. Differences in this version will typically require
//significant modification to an application in order for it to function.

#[derive(Debug)]
pub struct ApiVersion {
	pub raw: u32,
	pub major: u32,
	pub minor: u32,
	pub patch: u32,
}

impl ApiVersion {
	
	pub fn from_raw(raw: u32) -> Self {
		ApiVersion {
			raw: raw,
			major: (raw >> 22) & 0b0000_0011_1111_1111,
			minor: (raw >> 12) & 0b0000_0011_1111_1111,
			patch: (raw >>  0) & 0b0000_1111_1111_1111,
		}
	}

}