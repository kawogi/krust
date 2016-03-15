
use vk::structs::*;

/// The VkPhysicalDeviceMemoryProperties structure describes a number of memory heaps as well as a number
/// of memory types that can be used to access memory allocated in those heaps. Each heap describes a memory resource of
/// a particular size, and each memory type describes a set of memory properties (e.g. host cached vs uncached) that can be
/// used with a given memory heap. Allocations using a particular memory type will consume resources from the heap
/// indicated by that memory type’s heap index. More than one memory type may share each heap, and the heaps and
/// memory types provide a mechanism to advertise an accurate size of the physical memory resources while allowing the
/// memory to be used with a variety of different properties.
#[derive(Debug)]
pub struct PhysicalDeviceMemoryProperties {
	
	/// Each memory type is described by an element of the memoryTypes array, as a VkMemoryType structure.
	pub memory_heaps: Vec<VkMemoryHeap>, // TODO replace by krust structure
	
	/// Each heap is described by an element of the memoryHeaps array, as a VkMemoryHeap structure.
	pub memory_types: Vec<VkMemoryType>, // TODO replace by krust structure
	
}

impl<'a> From<&'a VkPhysicalDeviceMemoryProperties> for PhysicalDeviceMemoryProperties {
	
	fn from(memory_properties: &VkPhysicalDeviceMemoryProperties) -> Self {
		
		let memory_type_count = memory_properties.memoryTypeCount as usize;
		let mut memory_types = Vec::with_capacity(memory_type_count);
		memory_types.extend_from_slice(&memory_properties.memoryTypes.0[0..memory_type_count]);
		
		let memory_heap_count = memory_properties.memoryHeapCount as usize;
		let mut memory_heaps = Vec::with_capacity(memory_heap_count);
		memory_heaps.extend_from_slice(&memory_properties.memoryHeaps.0[0..memory_heap_count]);
		
		PhysicalDeviceMemoryProperties {
			memory_types: memory_types,
			memory_heaps: memory_heaps,
		}
	}
	
}

impl PhysicalDeviceMemoryProperties {
	
	// TODO use some predefined Display, Debug, ToString, whatever trait or at least replace by a macro
	pub fn dump(&self, prefix: &str) {
		
		let mut indent: String = String::from(prefix);
		indent.push('\t');

		for memory_type in &self.memory_types {
			println!("{}memory_types[]: {{", prefix);
			println!("{}\t{:?}", prefix, memory_type);
			println!("{}}}", prefix);
		}
		
		for memory_heap in &self.memory_heaps {
			println!("{}memory_heaps[]: {{", prefix);
			println!("{}\t{:?}", prefix, memory_heap);
			println!("{}}}", prefix);
		}
		
//		println!("{}memory_types: {:?}", prefix, self.memory_types);
//		println!("{}memory_heaps: {:?}", prefix, self.memory_heaps);
	}
}