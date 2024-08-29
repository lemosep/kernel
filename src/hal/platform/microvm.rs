// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
// Imports
//==================================================================================================

use crate::{
    hal::{
        arch::x86::{
            self,
            Arch,
        },
        io::{
            IoMemoryAllocator,
            IoPortAllocator,
        },
        mem::{
            MemoryRegion,
            PhysicalAddress,
            TruncatedMemoryRegion,
        },
        platform::{
            bootinfo::BootInfo,
            madt::MadtInfo,
        },
    },
    kmod::KernelModule,
};
use ::alloc::{
    collections::linked_list::LinkedList,
    string::ToString,
};
use ::arch::mem;
use ::error::Error;
use ::sys::mm::{
    Address,
    VirtualAddress,
};

//==================================================================================================
// Structures
//==================================================================================================

pub struct Platform {
    pub arch: Arch,
}

//==================================================================================================
// Constants
//==================================================================================================

/// Bootloader magic number.
pub const MICROVM_BOOT_MAGIC: u32 = 0x0c00ffee;

//==================================================================================================
// Standalone Functions
//==================================================================================================

///
/// # Description
///
/// Writes the 8-bit value `b` to the platform's standard output device.
///
/// # Parameters
///
/// - `b`: Value to write.
///
/// # Safety
///
/// This function is unsafe for multiple reasons:
/// - It assumes that the standard output device is present.
/// - It assumes that the standard output device was properly initialized.
/// - It does not prevent concurrent access to the standard output device.
///
pub unsafe fn putb(b: u8) {
    ::arch::io::out8(0xe9, b);
}

///
/// # Description
///
/// Writes the 32 bit value `val` to the platform's standard output device.
///
/// # Parameters
///
/// - `val`: Value to write.
///
/// # Safety
///
/// This function is unsafe for multiple reasons:
/// - It assumes that the standard output device is present.
/// - It assumes that the standard output device was properly initialized.
/// - It does not prevent concurrent access to the standard output device.
///
#[cfg(feature = "stdio")]
pub unsafe fn out32(val: u32) {
    ::arch::io::out32(0xe9, val);
}

///
/// # Description
///
/// Reads a 32-bit value from the platform's standard input device.
///
/// # Return
///
/// The 32-bit value read from the standard input device.
///
/// # Safety
///
/// This function is unsafe for multiple reasons:
/// - It assumes that the standard input device is present.
/// - It assumes that the standard input device was properly initialized.
/// - It does not prevent concurrent access to the standard input device.
///
#[cfg(feature = "stdio")]
pub unsafe fn in32() -> u32 {
    ::arch::io::in32(0xe9)
}

///
/// # Description
///
/// Shutdowns the machine.
///
/// # Return
///
/// This function never returns.
///
pub fn shutdown() -> ! {
    unsafe { arch::io::out16(0x604, 0x2000) };
    loop {
        core::hint::spin_loop();
    }
}

///
/// # Description
///
/// Parses boot information.
///
/// # Parameters
///
/// - `magic`: Magic number.
/// - `info`:  Address of the boot information.
///
/// # Returns
///
/// A new boot information structure.
///
pub fn parse_bootinfo(magic: u32, info: usize) -> Result<BootInfo, Error> {
    // Check if magic number matches what we expect.
    if magic != MICROVM_BOOT_MAGIC {
        let reason: &str = "invalid boot magic number";
        error!("parse_bootinfo(): magic={:#010x}, info={:#010x} (error={})", magic, info, reason);
        return Err(Error::new(error::ErrorCode::InvalidArgument, reason));
    }

    trace!("parse_bootinfo(): magic={:#010x}, info={:#010x}", magic, info);

    // Retrieve initrd information.
    // - Lower 12 bits encode the size of the initrd.
    // - Higher bits encode the base address of the initrd.
    let initrd_size: usize = info & 0xfff;
    let initrd_base: usize = info & !0xfff;

    let mut kernel_modules: LinkedList<KernelModule> = LinkedList::new();

    // Register initrd as a kernel module.
    if initrd_size != 0 {
        info!(
            "parse_bootinfo(): initrd_base={:#010x}, initrd_size={:#010x}",
            initrd_base, initrd_size
        );

        // Add kernel module to the list of kernel modules.
        let module: KernelModule = KernelModule::new(
            PhysicalAddress::from_raw_value(initrd_base)?,
            initrd_size * mem::PAGE_SIZE,
            "initrd".to_string(),
        );
        kernel_modules.push_back(module);
    }

    Ok(BootInfo::new(None, LinkedList::new(), LinkedList::new(), kernel_modules))
}

pub fn init(
    ioports: &mut IoPortAllocator,
    ioaddresses: &mut IoMemoryAllocator,
    _memory_regions: &mut LinkedList<MemoryRegion<VirtualAddress>>,
    _mmio_regions: &mut LinkedList<TruncatedMemoryRegion<VirtualAddress>>,
    madt: &Option<MadtInfo>,
) -> Result<Platform, Error> {
    Ok(Platform {
        arch: x86::init(ioports, ioaddresses, madt)?,
    })
}
