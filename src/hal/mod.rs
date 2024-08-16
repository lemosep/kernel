// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
// Modules
//==================================================================================================

pub mod arch;
pub mod cpu;
pub mod io;
pub mod mem;

//==================================================================================================
// Imports
//==================================================================================================

use crate::hal::{
    arch::x86::{
        cpu::{
            madt::MadtInfo,
            ExceptionController,
        },
        Arch,
    },
    cpu::InterruptManager,
    io::{
        IoMemoryAllocator,
        IoPortAllocator,
    },
    mem::{
        MemoryRegion,
        TruncatedMemoryRegion,
        VirtualAddress,
    },
};
use ::alloc::collections::linked_list::LinkedList;
use ::error::{
    Error,
    ErrorCode,
};

//==================================================================================================
// Structures
//==================================================================================================

///
/// # Description
///
/// A type that describes components of the hardware abstraction layer.
///
pub struct Hal {
    pub _arch: Arch,
    pub ioports: IoPortAllocator,
    pub ioaddresses: IoMemoryAllocator,
    pub intman: cpu::InterruptManager,
    pub excpman: ExceptionController,
}

//==================================================================================================
// Standalone Functions
//==================================================================================================

pub fn init(
    memory_regions: &mut LinkedList<MemoryRegion<VirtualAddress>>,
    mmio_regions: &mut LinkedList<TruncatedMemoryRegion<VirtualAddress>>,
    madt: &Option<MadtInfo>,
) -> Result<Hal, Error> {
    info!("initializing hardware abstraction layer...");

    let mut ioports: IoPortAllocator = IoPortAllocator::new();
    let mut ioaddresses: IoMemoryAllocator = IoMemoryAllocator::new();
    let mut arch: Arch =
        arch::init(&mut ioports, &mut ioaddresses, memory_regions, mmio_regions, madt)?;

    // Initialize the interrupt manager.
    let intman: InterruptManager = match arch.controller.take() {
        Some(controller) => InterruptManager::new(controller)?,
        None => {
            let reason: &str = "no interrupt controller found";
            error!("{}", reason);
            return Err(Error::new(ErrorCode::NoSuchDevice, reason));
        },
    };

    // Initialize exception manager.
    // TODO: add comments about safety.
    let excpman: ExceptionController = unsafe { ExceptionController::init()? };

    Ok(Hal {
        _arch: arch,
        ioports,
        ioaddresses,
        intman,
        excpman,
    })
}
