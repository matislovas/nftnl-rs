use crate::{MsgType, Table};
use nftnl_sys::{self as sys, libc};
use std::{
    ffi::{c_void, CStr},
    fmt,
    os::raw::c_char,
};

/// Base quota type.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum QuotaType {
    /// Verdict is passed UNTIL quota is exceeded
    Until,
    /// Verdict is passed WHEN quota is exceeded
    Over,
}

/// Abstraction of a `nftnl_obj` for quota. Quotas objs reside inside [`Table`]s.
///
/// There are two types of quotas, "until" and "over".
///
/// [`Table`]: struct.Table.html
pub struct Quota<'a> {
    quota: *mut sys::nftnl_obj,
    table: &'a Table,
}

// Safety: It should be safe to pass this around and *read* from it
// from multiple threads
unsafe impl<'a> Send for Quota<'a> {}
unsafe impl<'a> Sync for Quota<'a> {}

impl<'a> Quota<'a> {
    /// Creates a new quota instance inside the given [`Table`] and with the given name.
    ///
    /// [`Table`]: struct.Table.html
    pub fn new<T: AsRef<CStr>>(name: &T, table: &'a Table) -> Quota<'a> {
        unsafe {
            let quota = try_alloc!(sys::nftnl_obj_alloc());

            sys::nftnl_obj_set_u32(
                quota,
                sys::NFTNL_OBJ_FAMILY as u16,
                table.get_family() as u32,
            );

            sys::nftnl_obj_set_u32(
                quota,
                sys::NFTNL_OBJ_TYPE as u16,
                sys::NFT_OBJECT_QUOTA as u32,
            );

            sys::nftnl_obj_set_str(
                quota,
                sys::NFTNL_OBJ_TABLE as u16,
                table.get_name().as_ptr(),
            );

            sys::nftnl_obj_set_str(
                quota,
                sys::NFTNL_OBJ_NAME as u16,
                name.as_ref().as_ptr()
            );

            sys::nftnl_obj_set_u64(
                quota,
                sys::NFTNL_OBJ_QUOTA_CONSUMED as u16,
                0 as u64
            );

            Quota { quota, table }
        }
    }

    /// Set type of this quota ('over' or 'until')
    pub fn set_type(&mut self, quota_type: QuotaType) {
        let flag: u32 = match quota_type {
            QuotaType::Until => 0,
            QuotaType::Over => libc::NFT_QUOTA_F_INV as u32,
        };

        unsafe {
            sys::nftnl_obj_set_u32(
                self.quota,
                sys::NFTNL_OBJ_QUOTA_FLAGS as u16,
                flag
            );
        }
    }

    /// Set limit of bytes, when thi quota overflows
    pub fn set_limit(&mut self, bytes: u64) {
        unsafe {
            sys::nftnl_obj_set_u64(
                self.quota,
                sys::NFTNL_OBJ_QUOTA_BYTES as u16,
                bytes
            );
        }
    }
    
    /// Set starting value of already consumed bytes (default: 0)
    pub fn set_consumed(&mut self, bytes: u64) {
        unsafe {
            sys::nftnl_obj_set_u64(
                self.quota,
                sys::NFTNL_OBJ_QUOTA_CONSUMED as u16,
                bytes
            );
        }
    }

    pub fn get_name(&self) -> &CStr {
        unsafe {
            let ptr = sys::nftnl_obj_get_str(self.quota, sys::NFTNL_OBJ_NAME as u16);
            CStr::from_ptr(ptr)
        }
    }

    /// Returns a reference to the [`Table`] this quota belongs to
    ///
    /// [`Table`]: struct.Table.html
    pub fn get_table(&self) -> &Table {
        self.table
    }
}

impl<'a> fmt::Debug for Quota<'a> {
    /// Return a string representation of the quota.
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer: [u8; 4096] = [0; 4096];

        unsafe {
            sys::nftnl_obj_snprintf(
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
                self.quota,
                sys::NFT_OBJECT_QUOTA as u32,
                0,
            );
        }

        let s = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        
        write!(fmt, "{:?}", s)
    }
}

unsafe impl<'a> crate::NlMsg for Quota<'a> {
    unsafe fn write(&self, buf: *mut c_void, seq: u32, msg_type: MsgType) {
        let raw_msg_type = match msg_type {
            MsgType::Add => libc::NFT_MSG_NEWOBJ,
            MsgType::Del => libc::NFT_MSG_DELOBJ,
        };

        let flags: u16 = match msg_type {
            MsgType::Add => (libc::NLM_F_ACK | libc::NLM_F_CREATE) as u16,
            MsgType::Del => libc::NLM_F_ACK as u16,
        };

        let header = sys::nftnl_nlmsg_build_hdr(
            buf as *mut c_char,
            raw_msg_type as u16,
            self.table.get_family() as u16,
            flags,
            seq,
        );

        sys::nftnl_obj_nlmsg_build_payload(header, self.quota);
    }
}

impl<'a> Drop for Quota<'a> {
    fn drop(&mut self) {
        unsafe { sys::nftnl_obj_free(self.quota) };
    }
}
