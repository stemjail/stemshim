// Copyright (C) 2014-2016 Mickaël Salaün
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

#![feature(borrow_state)]
#![feature(thread_local_state)]

extern crate libc;
extern crate stemjail;

use libc::c_char;
use std::cell::{BorrowState, RefCell};
use std::ffi::CStr;
use std::str::from_utf8;
use std::thread::LocalKeyState;
use stemjail::cmd::shim::{AccessData, ShimKageCmd, AccessCache};
use stemjail::util::absolute_path;

// TODO: Handle openat-like functions (from the C side):
// 1. readlink /proc/self/<fd> + check for trailing " (deleted)"
// 2. walk throught .. until finding / (i.e. like getcwd, cf. http://lxr.linux.no/linux+v2.6.33/fs/dcache.c#L1905)

// One cache per thread to avoid synchronous operations
thread_local!(static ACCESS_CACHE: RefCell<AccessCache> = RefCell::new(AccessCache::new()));

/// Inform the monitor for a futur access.
/// Get back `true` if the request succeded or `false` otherwise (i.e. should perform the real access
/// in any case).
#[no_mangle]
pub extern "C" fn stemjail_request_access(path: *const c_char, write: bool) -> bool {
    if path == std::ptr::null() {
        return false;
    }
    let c_str = unsafe { CStr::from_ptr(path) };
    match from_utf8(c_str.to_bytes()) {
        Ok(val) => {
            match ACCESS_CACHE.state() {
                LocalKeyState::Destroyed => false,
                LocalKeyState::Uninitialized | LocalKeyState::Valid => {
                    ACCESS_CACHE.with(|cache| {
                        match cache.borrow_state() {
                            // Ignore the request when the cache_ask_access code is using the
                            // filesystem to avoid recursion.
                            BorrowState::Reading | BorrowState::Writing => false,
                            BorrowState::Unused => {
                                let access_data = AccessData {
                                    path: absolute_path(val),
                                    write: write,
                                };
                                ShimKageCmd::cache_ask_access(access_data, &mut *cache.borrow_mut()).is_ok()
                            }
                        }
                    })
                }
            }
        },
        Err(_) => false,
    }
}
