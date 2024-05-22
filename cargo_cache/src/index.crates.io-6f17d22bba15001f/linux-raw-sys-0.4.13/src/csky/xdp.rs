/* automatically generated by rust-bindgen 0.66.1 */

pub type __s8 = crate::ctypes::c_schar;
pub type __u8 = crate::ctypes::c_uchar;
pub type __s16 = crate::ctypes::c_short;
pub type __u16 = crate::ctypes::c_ushort;
pub type __s32 = crate::ctypes::c_int;
pub type __u32 = crate::ctypes::c_uint;
pub type __s64 = crate::ctypes::c_longlong;
pub type __u64 = crate::ctypes::c_ulonglong;
pub type __kernel_key_t = crate::ctypes::c_int;
pub type __kernel_mqd_t = crate::ctypes::c_int;
pub type __kernel_long_t = crate::ctypes::c_long;
pub type __kernel_ulong_t = crate::ctypes::c_ulong;
pub type __kernel_ino_t = __kernel_ulong_t;
pub type __kernel_mode_t = crate::ctypes::c_uint;
pub type __kernel_pid_t = crate::ctypes::c_int;
pub type __kernel_ipc_pid_t = crate::ctypes::c_int;
pub type __kernel_uid_t = crate::ctypes::c_uint;
pub type __kernel_gid_t = crate::ctypes::c_uint;
pub type __kernel_suseconds_t = __kernel_long_t;
pub type __kernel_daddr_t = crate::ctypes::c_int;
pub type __kernel_uid32_t = crate::ctypes::c_uint;
pub type __kernel_gid32_t = crate::ctypes::c_uint;
pub type __kernel_old_uid_t = __kernel_uid_t;
pub type __kernel_old_gid_t = __kernel_gid_t;
pub type __kernel_old_dev_t = crate::ctypes::c_uint;
pub type __kernel_size_t = crate::ctypes::c_uint;
pub type __kernel_ssize_t = crate::ctypes::c_int;
pub type __kernel_ptrdiff_t = crate::ctypes::c_int;
pub type __kernel_off_t = __kernel_long_t;
pub type __kernel_loff_t = crate::ctypes::c_longlong;
pub type __kernel_old_time_t = __kernel_long_t;
pub type __kernel_time_t = __kernel_long_t;
pub type __kernel_time64_t = crate::ctypes::c_longlong;
pub type __kernel_clock_t = __kernel_long_t;
pub type __kernel_timer_t = crate::ctypes::c_int;
pub type __kernel_clockid_t = crate::ctypes::c_int;
pub type __kernel_caddr_t = *mut crate::ctypes::c_char;
pub type __kernel_uid16_t = crate::ctypes::c_ushort;
pub type __kernel_gid16_t = crate::ctypes::c_ushort;
pub type __le16 = __u16;
pub type __be16 = __u16;
pub type __le32 = __u32;
pub type __be32 = __u32;
pub type __le64 = __u64;
pub type __be64 = __u64;
pub type __sum16 = __u16;
pub type __wsum = __u32;
pub type __poll_t = crate::ctypes::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sockaddr_xdp {
pub sxdp_family: __u16,
pub sxdp_flags: __u16,
pub sxdp_ifindex: __u32,
pub sxdp_queue_id: __u32,
pub sxdp_shared_umem_fd: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_ring_offset {
pub producer: __u64,
pub consumer: __u64,
pub desc: __u64,
pub flags: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_mmap_offsets {
pub rx: xdp_ring_offset,
pub tx: xdp_ring_offset,
pub fr: xdp_ring_offset,
pub cr: xdp_ring_offset,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_umem_reg {
pub addr: __u64,
pub len: __u64,
pub chunk_size: __u32,
pub headroom: __u32,
pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_statistics {
pub rx_dropped: __u64,
pub rx_invalid_descs: __u64,
pub tx_invalid_descs: __u64,
pub rx_ring_full: __u64,
pub rx_fill_ring_empty_descs: __u64,
pub tx_ring_empty_descs: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_options {
pub flags: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_desc {
pub addr: __u64,
pub len: __u32,
pub options: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_ring_offset_v1 {
pub producer: __u64,
pub consumer: __u64,
pub desc: __u64,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_mmap_offsets_v1 {
pub rx: xdp_ring_offset_v1,
pub tx: xdp_ring_offset_v1,
pub fr: xdp_ring_offset_v1,
pub cr: xdp_ring_offset_v1,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_umem_reg_v1 {
pub addr: __u64,
pub len: __u64,
pub chunk_size: __u32,
pub headroom: __u32,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct xdp_statistics_v1 {
pub rx_dropped: __u64,
pub rx_invalid_descs: __u64,
pub tx_invalid_descs: __u64,
}
pub const XDP_SHARED_UMEM: u32 = 1;
pub const XDP_COPY: u32 = 2;
pub const XDP_ZEROCOPY: u32 = 4;
pub const XDP_USE_NEED_WAKEUP: u32 = 8;
pub const XDP_UMEM_UNALIGNED_CHUNK_FLAG: u32 = 1;
pub const XDP_RING_NEED_WAKEUP: u32 = 1;
pub const XDP_MMAP_OFFSETS: u32 = 1;
pub const XDP_RX_RING: u32 = 2;
pub const XDP_TX_RING: u32 = 3;
pub const XDP_UMEM_REG: u32 = 4;
pub const XDP_UMEM_FILL_RING: u32 = 5;
pub const XDP_UMEM_COMPLETION_RING: u32 = 6;
pub const XDP_STATISTICS: u32 = 7;
pub const XDP_OPTIONS: u32 = 8;
pub const XDP_OPTIONS_ZEROCOPY: u32 = 1;
pub const XDP_PGOFF_RX_RING: u32 = 0;
pub const XDP_PGOFF_TX_RING: u32 = 2147483648;
pub const XDP_UMEM_PGOFF_FILL_RING: u64 = 4294967296;
pub const XDP_UMEM_PGOFF_COMPLETION_RING: u64 = 6442450944;
pub const XSK_UNALIGNED_BUF_OFFSET_SHIFT: u32 = 48;
pub const XSK_UNALIGNED_BUF_ADDR_MASK: u64 = 281474976710655;
pub const XDP_USE_SG: u32 = 16;
pub const XDP_PKT_CONTD: u32 = 1;
