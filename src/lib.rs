// Copyright (C) 2014-2015 Mickaël Salaün
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, version 3 of the License.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with this program. If not, see <http://www.gnu.org/licenses/>.

#![feature(libc)]

extern crate libc;
extern crate stemjail;

use libc::c_char;
use std::ffi::CStr;
use std::str::from_utf8;
use stemjail::cmd::shim::ShimKageCmd;
use stemjail::util::absolute_path;

// TODO: Handle openat-like functions (from the C side):
// 1. readlink /proc/self/<fd> + check for trailing " (deleted)"
// 2. walk throught .. until finding / (i.e. like getcwd, cf. http://lxr.linux.no/linux+v2.6.33/fs/dcache.c#L1905)

#[no_mangle]
/// Inform the monitor for a futur access.
/// Get back `true` if the request succeded or `false` otherwise (i.e. should perform the real access
/// in any case).
pub extern "C" fn stemjail_request_access(path: *const c_char, write: bool) -> bool {
    // TODO: Add a cache
    if path == std::ptr::null() {
        return false;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    match from_utf8(c_str.to_bytes()) {
        Ok(val) => {
            match ShimKageCmd::do_access(absolute_path(val), write) {
                Ok(_) => true,
                Err(_) => false,
            }
        },
        Err(_) => false,
    }
}
