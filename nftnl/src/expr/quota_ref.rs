use super::{Expression, Rule};
use crate::quota::Quota;
use nftnl_sys::{self as sys};
use std::os::raw::c_char;
use std::ffi::CString;

/// A reference to quota obj expression adds a quota to the rule that is incremented to count number of bytes
/// for all packets that has matched the rule.
pub struct QuotaRef {
    quota_name: CString,
}

impl QuotaRef {
    pub fn new(quota: &Quota) -> Self {
        QuotaRef {
            quota_name: quota.get_name().to_owned(),
        }
    }
}

// pub fn new<T: AsRef<CStr>>(name: &T) -> Self {
//     QuotaRef {
//         quota_name: name.as_ref().to_owned(),
//     }
// }

// trait IntoQuotaRef {
//     fn into(self) -> QuotaRef;
// }

// impl<'a> IntoQuotaRef for &'a Quota {
//     fn into(self) -> QuotaRef {
//         QuotaRef::new(self.get_name().to_owned())
//     }
// }

// impl IntoQuotaRef for CString {
//     fn into(self) -> QuotaRef {
//         QuotaRef::new(self.to_owned())
//     }
// }

impl Expression for QuotaRef {
    fn to_expr(&self, _rule: &Rule) -> *mut sys::nftnl_expr {
        unsafe {
            let expr = try_alloc!(sys::nftnl_expr_alloc(b"objref\0" as *const _ as *const c_char));

            sys::nftnl_expr_set_str(
                expr, 
                sys::NFTNL_EXPR_OBJREF_IMM_NAME as u16, 
                self.quota_name.as_ptr() as *const _ as *const c_char
            );

	        sys::nftnl_expr_set_u32(
                expr, 
                sys::NFTNL_EXPR_OBJREF_IMM_TYPE as u16,
                sys::NFT_OBJECT_QUOTA as u32,
            );

            expr
        }
    }
}


#[macro_export]
macro_rules! nft_expr_quota {
    ($quota:expr) => {
        $crate::expr::QuotaRef::new($quota)
    };
}
