use super::{Expression, Rule};
use nftnl_sys::{self as sys};
use std::{
    ffi::CString,
    os::raw::c_char
};


bitflags::bitflags! {
    pub struct LogFlags: u8 {
        const TCPSEQ =  0x01;
        const TCPOPT =  0x02;
        const IPOPT =  0x04;
        const UID =  0x08;
        const MACDECODE =  0x20;
        const MASK =  0x2f;
    }
}

pub struct Log {
    group: Option<u16>,
    snaplen: Option<u32>,
    qthreshold: Option<u16>,
    level: Option<i32>,
    flags: Option<LogFlags>,
    prefix: Option<CString>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            group: None,
            snaplen: None,
            qthreshold: None,
            level: None,
            flags: None,
            prefix: None,
        }
    }

    pub fn group(self, group: u16) -> Self {
        Log {
            group: Some(group),
            ..self
        }
    }

    pub fn snaplen(self, snaplen: u32) -> Self {
        Log {
            snaplen: Some(snaplen),
            ..self
        }
    }

    pub fn qthreshold(self, qthreshold: u16) -> Self {
        Log {
            qthreshold: Some(qthreshold),
            ..self
        }
    }

    pub fn level(self, level: i32) -> Self {
        Log {
            level: Some(level),
            ..self
        }
    }

    pub fn flags(self, flags: LogFlags) -> Self {
        Log {
            flags: Some(flags),
            ..self
        }
    }

    pub fn prefix(self, prefix: &CString) -> Self {
        Log {
            prefix: Some(prefix.clone()),
            ..self
        }
    }
}


impl Expression for Log {
    fn to_expr(&self, _rule: &Rule) -> *mut sys::nftnl_expr {
        unsafe {
            let expr = try_alloc!(sys::nftnl_expr_alloc(b"log\0" as *const _ as *const c_char));
            match self.prefix.as_deref() {
                Some(prefix) => {
                    sys::nftnl_expr_set_str(
                        expr, 
                        sys::NFTNL_EXPR_LOG_PREFIX as u16,
                        prefix.as_ref().as_ptr(),
                    );
                },
                _ => {},
            };
            match self.group {
                Some(group) => {
                    sys::nftnl_expr_set_u16(
                        expr,
                        sys::NFTNL_EXPR_LOG_GROUP as u16,
                        group as u16,
                    );

                    match self.snaplen {
                        Some(snaplen) => {
                            sys::nftnl_expr_set_u32(
                                expr,
                                sys::NFTNL_EXPR_LOG_SNAPLEN as u16,
                                snaplen as u32,
                            );
                        },
                        _ => {},
                    };

                    match self.qthreshold {
                        Some(qthreshold) => {
                            sys::nftnl_expr_set_u16(
                                expr,
                                sys::NFTNL_EXPR_LOG_QTHRESHOLD as u16,
                                qthreshold as u16,
                            );
                        },
                        _ => {},
                    };
                },
                _ => {
                    match self.level {
                        Some(level) => {
                            sys::nftnl_expr_set_u32(
                                expr,
                                sys::NFTNL_EXPR_LOG_LEVEL as u16,
                                level as u32,
                            );
                        },
                        _ => {},
                    };

                    match self.flags {
                        Some(flags) => {
                            sys::nftnl_expr_set_u32(
                                expr,
                                sys::NFTNL_EXPR_LOG_FLAGS as u16,
                                flags.bits as u32,
                            );
                        },
                        _ => {},
                    };
                },
            };

            expr
        }
    }
}
