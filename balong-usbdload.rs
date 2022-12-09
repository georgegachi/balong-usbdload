#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case, non_upper_case_globals, unused_assignments, unused_mut)]
#![register_tool(c2rust)]
#![feature(extern_types, register_tool)]
extern "C" {
    pub type ptable_t;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(__filename: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn putchar(__c: libc::c_int) -> libc::c_int;
    fn fseek(
        __stream: *mut FILE,
        __off: libc::c_long,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn fread(
        __ptr: *mut libc::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> size_t;
    fn atoi(__nptr: *const libc::c_char) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(__ptr: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn bzero(_: *mut libc::c_void, _: libc::c_ulong);
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn tcsetattr(
        __fd: libc::c_int,
        __optional_actions: libc::c_int,
        __termios_p: *const termios,
    ) -> libc::c_int;
    fn tcdrain(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn getopt(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
    ) -> libc::c_int;
    static mut optind: libc::c_int;
    static mut optarg: *mut libc::c_char;
    fn htonl(__hostlong: uint32_t) -> uint32_t;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __ssize_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub __pad1: *mut libc::c_void,
    pub __pad2: *mut libc::c_void,
    pub __pad3: *mut libc::c_void,
    pub __pad4: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_marker {
    pub _next: *mut _IO_marker,
    pub _sbuf: *mut _IO_FILE,
    pub _pos: libc::c_int,
}
pub type FILE = _IO_FILE;
pub type ssize_t = __ssize_t;
pub type uint8_t = libc::c_uchar;
pub type uint32_t = libc::c_uint;
pub type cc_t = libc::c_uchar;
pub type speed_t = libc::c_uint;
pub type tcflag_t = libc::c_uint;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub lmode: libc::c_int,
    pub size: libc::c_int,
    pub adr: libc::c_int,
    pub offset: libc::c_int,
    pub pbuf: *mut libc::c_char,
}
#[no_mangle]
pub static mut siofd: libc::c_int = 0;
#[no_mangle]
pub static mut sioparm: termios = termios {
    c_iflag: 0,
    c_oflag: 0,
    c_cflag: 0,
    c_lflag: 0,
    c_line: 0,
    c_cc: [0; 32],
    c_ispeed: 0,
    c_ospeed: 0,
};
#[no_mangle]
pub static mut ldr: *mut FILE = 0 as *const FILE as *mut FILE;
#[no_mangle]
pub unsafe extern "C" fn dump(mut buffer: *mut libc::c_uchar, mut len: libc::c_int) {
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ch: libc::c_uchar = 0;
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    i = 0 as libc::c_int;
    while i < len {
        printf(b"%04x: \0" as *const u8 as *const libc::c_char, i);
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            if i + j < len {
                printf(
                    b"%02x \0" as *const u8 as *const libc::c_char,
                    *buffer.offset((i + j) as isize) as libc::c_int & 0xff as libc::c_int,
                );
            } else {
                printf(b"   \0" as *const u8 as *const libc::c_char);
            }
            j += 1;
        }
        printf(b" *\0" as *const u8 as *const libc::c_char);
        j = 0 as libc::c_int;
        while j < 16 as libc::c_int {
            if i + j < len {
                ch = *buffer.offset((i + j) as isize);
                if (ch as libc::c_int) < 0x20 as libc::c_int
                    || ch as libc::c_int > 0x7e as libc::c_int
                        && (ch as libc::c_int) < 0xc0 as libc::c_int
                {
                    putchar('.' as i32);
                } else {
                    putchar(ch as libc::c_int);
                }
            } else {
                printf(b" \0" as *const u8 as *const libc::c_char);
            }
            j += 1;
        }
        printf(b"*\n\0" as *const u8 as *const libc::c_char);
        i += 16 as libc::c_int;
    }
}
#[no_mangle]
pub unsafe extern "C" fn csum(mut buf: *mut libc::c_uchar, mut len: libc::c_int) {
    let mut i: libc::c_uint = 0;
    let mut c: libc::c_uint = 0;
    let mut csum_0: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut cconst: [libc::c_uint; 16] = [
        0 as libc::c_int as libc::c_uint,
        0x1021 as libc::c_int as libc::c_uint,
        0x2042 as libc::c_int as libc::c_uint,
        0x3063 as libc::c_int as libc::c_uint,
        0x4084 as libc::c_int as libc::c_uint,
        0x50a5 as libc::c_int as libc::c_uint,
        0x60c6 as libc::c_int as libc::c_uint,
        0x70e7 as libc::c_int as libc::c_uint,
        0x8108 as libc::c_int as libc::c_uint,
        0x9129 as libc::c_int as libc::c_uint,
        0xa14a as libc::c_int as libc::c_uint,
        0xb16b as libc::c_int as libc::c_uint,
        0xc18c as libc::c_int as libc::c_uint,
        0xd1ad as libc::c_int as libc::c_uint,
        0xe1ce as libc::c_int as libc::c_uint,
        0xf1ef as libc::c_int as libc::c_uint,
    ];
    i = 0 as libc::c_int as libc::c_uint;
    while i < (len - 2 as libc::c_int) as libc::c_uint {
        c = (*buf.offset(i as isize) as libc::c_int & 0xff as libc::c_int)
            as libc::c_uint;
        csum_0 = csum_0 << 4 as libc::c_int & 0xffff as libc::c_int as libc::c_uint
            ^ cconst[(c >> 4 as libc::c_int ^ csum_0 >> 12 as libc::c_int) as usize];
        csum_0 = csum_0 << 4 as libc::c_int & 0xffff as libc::c_int as libc::c_uint
            ^ cconst[(c & 0xf as libc::c_int as libc::c_uint
                ^ csum_0 >> 12 as libc::c_int) as usize];
        i = i.wrapping_add(1);
    }
    *buf
        .offset(
            (len - 2 as libc::c_int) as isize,
        ) = (csum_0 >> 8 as libc::c_int & 0xff as libc::c_int as libc::c_uint)
        as libc::c_uchar;
    *buf
        .offset(
            (len - 1 as libc::c_int) as isize,
        ) = (csum_0 & 0xff as libc::c_int as libc::c_uint) as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn sendcmd(
    mut cmdbuf: *mut libc::c_uchar,
    mut len: libc::c_int,
) -> libc::c_int {
    let mut replybuf: [libc::c_uchar; 1024] = [0; 1024];
    let mut replylen: libc::c_uint = 0;
    csum(cmdbuf, len);
    write(siofd, cmdbuf as *const libc::c_void, len as size_t);
    tcdrain(siofd);
    replylen = read(
        siofd,
        replybuf.as_mut_ptr() as *mut libc::c_void,
        1024 as libc::c_int as size_t,
    ) as libc::c_uint;
    if replylen == 0 as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    if replybuf[0 as libc::c_int as usize] as libc::c_int == 0xaa as libc::c_int {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn open_port(mut devname: *mut libc::c_char) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut dflag: libc::c_int = 1 as libc::c_int;
    let mut devstr: [libc::c_char; 200] = [
        0 as libc::c_int as libc::c_char,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    i = 0 as libc::c_int;
    while (i as libc::c_ulong) < strlen(devname) {
        if (*devname.offset(i as isize) as libc::c_int) < '0' as i32
            || *devname.offset(i as isize) as libc::c_int > '9' as i32
        {
            dflag = 0 as libc::c_int;
        }
        i += 1;
    }
    if dflag != 0 {
        strcpy(
            devstr.as_mut_ptr(),
            b"/dev/ttyUSB\0" as *const u8 as *const libc::c_char,
        );
    }
    strcat(devstr.as_mut_ptr(), devname);
    siofd = open(
        devstr.as_mut_ptr(),
        0o2 as libc::c_int | 0o400 as libc::c_int | 0o4010000 as libc::c_int,
    );
    if siofd == -(1 as libc::c_int) {
        return 0 as libc::c_int;
    }
    bzero(
        &mut sioparm as *mut termios as *mut libc::c_void,
        ::std::mem::size_of::<termios>() as libc::c_ulong,
    );
    sioparm
        .c_cflag = (0o10002 as libc::c_int | 0o60 as libc::c_int | 0o4000 as libc::c_int
        | 0o200 as libc::c_int) as tcflag_t;
    sioparm.c_iflag = 0 as libc::c_int as tcflag_t;
    sioparm.c_oflag = 0 as libc::c_int as tcflag_t;
    sioparm.c_lflag = 0 as libc::c_int as tcflag_t;
    sioparm.c_cc[5 as libc::c_int as usize] = 30 as libc::c_int as cc_t;
    sioparm.c_cc[6 as libc::c_int as usize] = 0 as libc::c_int as cc_t;
    tcsetattr(siofd, 0 as libc::c_int, &mut sioparm);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn locate_kernel(
    mut pbuf: *mut libc::c_char,
    mut size: uint32_t,
) -> libc::c_int {
    let mut off: libc::c_int = 0;
    off = size.wrapping_sub(8 as libc::c_int as libc::c_uint) as libc::c_int;
    while off > 0 as libc::c_int {
        if strncmp(
            pbuf.offset(off as isize),
            b"ANDROID!\0" as *const u8 as *const libc::c_char,
            8 as libc::c_int as libc::c_ulong,
        ) == 0 as libc::c_int
        {
            return off;
        }
        off -= 1;
    }
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::std::ptr::null_mut());
    unsafe {
        main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        )
    }
    ::std::process::exit(0i32);
}
