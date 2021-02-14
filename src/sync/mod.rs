use ash::vk;
use std::sync::Arc;
use crate::device::Queue;

pub(crate) fn vulkan_sharing_mode<'a, S>(sharing_queues: S) -> (vk::SharingMode, u32, *const u32)
where
	S: IntoIterator<Item=&'a Arc<Queue>>
{
	let mut ids: Vec<u32> = sharing_queues.into_iter().map(|q| q.family_index()).collect();
	ids.sort();
	ids.dedup();

	if ids.len() > 1 {
		(vk::SharingMode::EXCLUSIVE, 0, std::ptr::null())
	} else {
		(vk::SharingMode::CONCURRENT, ids.len() as u32, ids.as_ptr())
	}
}