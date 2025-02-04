// Copyright(c) The Maintainers of Nanvix.
// Licensed under the MIT License.

//==================================================================================================
// Imports
//==================================================================================================

use crate::{
    kcall::KcallArgs,
    pm::process::ProcessManager,
};
use ::sys::error::Error;
use ::sys::pm::{
    ProcessIdentifier,
    UserIdentifier,
};

//==================================================================================================
// Standalone Functions
//==================================================================================================

fn do_getuid(pm: &ProcessManager, pid: ProcessIdentifier) -> Result<UserIdentifier, Error> {
    pm.getuid(pid)
}

pub fn getuid(pm: &ProcessManager, args: &KcallArgs) -> i32 {
    match do_getuid(pm, args.pid) {
        Ok(uid) => uid.into(),
        Err(e) => e.code.into_errno(),
    }
}
