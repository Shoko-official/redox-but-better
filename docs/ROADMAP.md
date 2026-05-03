# Infinity Core - Development Roadmap

## Phase 1: Foundation (Bootloader + NanoCore minimal)

**Objective**: Establish a bootable, `no_std` Rust environment and set up basic core functionalities required for the Infinity NanoCore.

### Step 1: Project Skeleton & Build System
- Initialize bare-metal Rust workspace (`no_std`, `no_main`).
- Configure custom target spec (e.g., `x86_64-infinity_core.json`).
- Integrate a bootloader (e.g., `bootloader` crate) to boot the kernel in QEMU.
- Implement a basic panic handler.

### Step 2: Early Observability (VGA / Serial)
- Implement a VGA text buffer driver for basic on-screen logging.
- Implement a Serial port driver (COM1) for headless debugging and CI test output.
- Create logging macros (`println!`, `serial_println!`).

### Step 3: CPU Configuration & Interrupts
- Set up the Global Descriptor Table (GDT) and Task State Segment (TSS).
- Configure the Interrupt Descriptor Table (IDT).
- Handle Double Faults safely.
- Initialize Programmable Interrupt Controller (PIC) / APIC.
- Handle hardware timer interrupts for future scheduling.

### Step 4: Memory Management (Unified Memory Plane - Foundation)
- Read memory map from bootloader.
- Implement a simple physical frame allocator.
- Set up page tables and virtual memory mapping.
- Implement a basic heap allocator to allow dynamic data structures (`alloc` crate support).

### Step 5: Process / Thread Skeleton (Preparation for Phase 2)
- Define initial Context structures for context switching.
- Draft the fundamental `Capability` structure to represent resources.

---

## Phase 2: Infinity Abstractions (IPC, Capabilities, Scheduler)

**Objective**: Implement the unique capabilities that make Infinity Core a true "nano-kernel" based on seL4 concepts.

### Step 1: Capability Security Model
- Implement Capability nodes and CSpace.
- Define capability operations (Mint, Copy, Revoke, Move).

### Step 2: Fast IPC (Inter-Process Communication)
- Design zero-copy IPC mechanisms via shared memory capabilities.
- Implement synchronous IPC endpoints.

### Step 3: Infinity Governor (Hyper-Scheduler Skeleton)
- Implement a priority-based round-robin scheduler.
- Introduce application intent metadata (`ProcessProfile`).

---

## Phase 3+ (Future Work)
- **Phase 3**: Essential Drivers (Keyboard, Mouse, NVMe skeleton).
- **Phase 4**: NFSx Core (Storage DAG).
- **Phase 5**: Aether Engine (UI Prototype).

## Development Rules
- Everything is written in Rust (`no_std` for the kernel).
- Any `unsafe` block must be strictly isolated, documented, and tested.
- Telemetry/Observability must be built-in from the start via serial output.
- Tests will run via custom test runners in QEMU.
