//! The IOCTL calls we need for the native linux backend

use nix::{ioctl_read, ioctl_readwrite_buf};

// From linux/hidraw.h
const HIDRAW_IOC_MAGIC: u8 = b'H';
const HIDRAW_IOC_GRDESCSIZE: u8 = 0x01;
const HIDRAW_SET_FEATURE: u8 = 0x06;
const HIDRAW_GET_FEATURE: u8 = 0x07;

ioctl_read!(
    hidraw_ioc_grdescsize,
    HIDRAW_IOC_MAGIC,
    HIDRAW_IOC_GRDESCSIZE,
    libc::c_int
);

ioctl_readwrite_buf!(
    hidraw_ioc_set_feature,
    HIDRAW_IOC_MAGIC,
    HIDRAW_SET_FEATURE,
    u8
);
ioctl_readwrite_buf!(
    hidraw_ioc_get_feature,
    HIDRAW_IOC_MAGIC,
    HIDRAW_GET_FEATURE,
    u8
);
