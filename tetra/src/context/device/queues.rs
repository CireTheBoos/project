use ash::vk;
use bitflags::bitflags;

/////////////////////////////////////////////////////////////////////////
// Structures
/////////////////////////////////////////////////////////////////////////

#[derive(Clone, Copy)]
pub struct Queue {
    pub family_index: u32,
    pub vk_queue: vk::Queue,
    pub roles: QueueRoleFlags,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct QueueRoleFlags: u32 {
        // single
        const PRESENT = 0b_0000_0001;
        const RENDER = 0b_0000_0010;
        const COMPUTE = 0b_0000_0100;
        const TRANSFER = 0b_0000_1000;

        // composed
        const GRAPHICS = Self::PRESENT.bits() | Self::RENDER.bits();
        const ALL = Self::PRESENT.bits() | Self::RENDER.bits() | Self::COMPUTE.bits() | Self::TRANSFER.bits();
    }
}

/////////////////////////////////////////////////////////////////////////
// Functions
/////////////////////////////////////////////////////////////////////////

pub fn queue(queues: &[Queue], role: QueueRoleFlags) -> Queue {
    queues
        .iter()
        .find(|queue| queue.roles.contains(role))
        .copied()
        .unwrap()
}

pub fn queue_family_indices(queues: &[Queue], mut roles: QueueRoleFlags) -> Vec<u32> {
    let mut queue_family_indices = Vec::with_capacity(3);
    for queue in queues {
        if queue.roles.intersects(roles) {
            queue_family_indices.push(queue.family_index);
            roles -= queue.roles;
        }
        if roles.is_empty() {
            break;
        }
    }
    queue_family_indices.sort();
    queue_family_indices.dedup();
    queue_family_indices
}
