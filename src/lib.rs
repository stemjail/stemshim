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

#![feature(collections)]
#![feature(libc)]

extern crate libc;
extern crate stemjail;

use libc::c_char;
use std::cell::RefCell;
use std::collections::{Bound, BTreeSet};
use std::ffi::CStr;
use std::str::from_utf8;
use stemjail::cmd::shim::{AccessRequest, ShimKageCmd};
use stemjail::util::absolute_path;

// TODO: Handle openat-like functions (from the C side):
// 1. readlink /proc/self/<fd> + check for trailing " (deleted)"
// 2. walk throught .. until finding / (i.e. like getcwd, cf. http://lxr.linux.no/linux+v2.6.33/fs/dcache.c#L1905)

// One cache per thread to avoid synchronous operations
thread_local!(static ACCESS_CACHE: RefCell<BTreeSet<AccessRequest>> = RefCell::new(BTreeSet::new()));

#[no_mangle]
/// Inform the monitor for a futur access.
/// Get back `true` if the request succeded or `false` otherwise (i.e. should perform the real access
/// in any case).
pub extern "C" fn stemjail_request_access(path: *const c_char, write: bool) -> bool {
    if path == std::ptr::null() {
        return false;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    match from_utf8(c_str.to_bytes()) {
        Ok(val) => {
            let req = AccessRequest {
                path: absolute_path(val),
                write: write,
            };

            ACCESS_CACHE.with(|cache| {
                // Same as stemflow::SetAccess::is_allowed()
                let is_cached = match cache.borrow()
                        .range(Bound::Included(&req), Bound::Unbounded).next() {
                    Some(ref x) => x.path.starts_with(&req.path),
                    None => false,
                };
                if is_cached {
                    true
                } else {
                    let ret = match ShimKageCmd::do_access(&req.path, req.write) {
                        Ok(_) => true,
                        Err(_) => false,
                    };
                    let _ = cache.borrow_mut().insert(req);
                    // TODO: Cleanup included requests if needed
                    ret
                }
            })
        },
        Err(_) => false,
    }
}
