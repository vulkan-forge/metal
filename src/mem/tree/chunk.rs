/// Memory chunk.
///
/// The interior of a memory chunk is adressed on 32-bits.
/// This means that the upper limit on a chunk size is 2^32 bytes (4GiB).
pub struct Chunk {
	memory: device::Memory
}

impl Chunk {
	pub fn new(device: &Arc<Device>, memory_type: MemoryType, size: u64) -> Result<Chunk, AllocationError> {
		let memory = device.allocate_memory(memory_type, size)?;
		Chunk {
			memory
		}
	}
}
