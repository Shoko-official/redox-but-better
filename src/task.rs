use alloc::vec::Vec;
use alloc::string::String;
use core::sync::atomic::{AtomicU64, Ordering};

/// A unique identifier for a Task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct TaskId(u64);

impl TaskId {
    pub fn new() -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(0);
        TaskId(NEXT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

/// Represents the execution intent of a process, used by the Infinity Governor.
#[derive(Debug, Clone)]
pub struct ProcessProfile {
    pub latency_sensitivity: u8, // 0-100
    pub throughput_need: u8,     // 0-100
    pub memory_pressure: u8,     // 0-100
    pub priority: u8,            // 0-100
}

/// Represents a fundamental right to access a system resource (seL4 style).
#[derive(Debug, Clone)]
pub enum Capability {
    /// Right to map/unmap a physical memory frame.
    MemoryFrame { phys_addr: u64, size: usize, writable: bool },
    /// Right to communicate over an IPC endpoint.
    IpcEndpoint { endpoint_id: u64, can_send: bool, can_recv: bool },
    /// Right to access a specific hardware device (MMIO, Ports).
    HardwareDevice { device_id: u64 },
    /// Right to schedule CPU time.
    ThreadControl { task_id: TaskId },
}

/// A Capability Space (CSpace) holds all the rights a task has.
#[derive(Debug)]
pub struct CSpace {
    capabilities: Vec<Capability>,
}

impl CSpace {
    pub fn new() -> Self {
        CSpace {
            capabilities: Vec::new(),
        }
    }

    pub fn grant(&mut self, cap: Capability) {
        self.capabilities.push(cap);
    }
}

/// The fundamental execution unit in Infinity NanoCore.
#[derive(Debug)]
pub struct Task {
    pub id: TaskId,
    pub name: String,
    pub profile: ProcessProfile,
    pub cspace: CSpace,
    // Context (Registers, stack pointer) will be added here
}

impl Task {
    pub fn new(name: &str, profile: ProcessProfile) -> Self {
        Task {
            id: TaskId::new(),
            name: String::from(name),
            profile,
            cspace: CSpace::new(),
        }
    }
}
