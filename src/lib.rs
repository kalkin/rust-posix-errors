//! Posix error codes and handy functions for using them.
//!
//! Error codes taken from [http://fxr.watson.org/fxr/source/sys/errno.h](http://fxr.watson.org/fxr/source/sys/errno.h)

/// This module makes it easy.
use std::fmt;
use std::io::ErrorKind;

/// Struct containing a posix error code and an error message
#[derive(Debug, Clone, PartialEq)]
pub struct PosixError {
    code: i32,
    message: String,
}

/// Operation not permitted
pub const EPERM: i32 = 1;
/// No such file or directory
pub const ENOENT: i32 = 2;
/// No such process
pub const ESRCH: i32 = 3;
/// Interrupted system call
pub const EINTR: i32 = 4;
///  Input/output error
pub const EIO: i32 = 5;
/// Device not configured
pub const ENXIO: i32 = 6;
/// Argument list too long
pub const E2BIG: i32 = 7;
/// Exec format error
pub const ENOEXEC: i32 = 8;
/// Bad file descriptor
pub const EBADF: i32 = 9;
/// No child processes
pub const ECHILD: i32 = 10;
/// Resource deadlock avoided
pub const EDEADLK: i32 = 11;
/// Cannot allocate memory
pub const ENOMEM: i32 = 12;
/// Permission denied
pub const EACCES: i32 = 13;
/// Bad address
pub const EFAULT: i32 = 14;
/// Block device required
pub const ENOTBLK: i32 = 15;
/// Device busy
pub const EBUSY: i32 = 16;
/// File exists
pub const EEXIST: i32 = 17;
/// Cross-device link
pub const EXDEV: i32 = 18;
/// Operation not supported by device
pub const ENODEV: i32 = 19;
/// Not a directory
pub const ENOTDIR: i32 = 20;
/// Is a directory
pub const EISDIR: i32 = 21;
/// Invalid argument
pub const EINVAL: i32 = 22;
/// Too many open files in system
pub const ENFILE: i32 = 23;
/// Too many open files
pub const EMFILE: i32 = 24;
/// Inappropriate ioctl for device
pub const ENOTTY: i32 = 25;
/// Text file busy
pub const ETXTBSY: i32 = 26;
/// File too large
pub const EFBIG: i32 = 27;
/// No space left on device
pub const ENOSPC: i32 = 28;
/// Illegal seek
pub const ESPIPE: i32 = 29;
/// Read-only filesystem
pub const EROFS: i32 = 30;
/// Too many links
pub const EMLINK: i32 = 31;
/// Broken pipe
pub const EPIPE: i32 = 32;
/// Numerical argument out of domain
pub const EDOM: i32 = 33;
/// Result too large
pub const ERANGE: i32 = 34;
/// Resource temporarily unavailable
pub const EAGAIN: i32 = 35;
/// Resource temporarily unavailable
pub const EWOULDBLOCK: i32 = 35;
/// Operation now in progress
pub const EINPROGRESS: i32 = 36;
/// Operation already in progress
pub const EALREADY: i32 = 37;
/// Socket operation on non-socket
pub const ENOTSOCK: i32 = 38;
/// Destination address required
pub const EDESTADDRREQ: i32 = 39;
/// Message too long
pub const EMSGSIZE: i32 = 40;
/// Protocol wrong type for socket
pub const EPROTOTYPE: i32 = 41;
/// Protocol not available
pub const ENOPROTOOPT: i32 = 42;
/// Protocol not supported
pub const EPROTONOSUPPORT: i32 = 43;
/// Socket type not supported
pub const ESOCKTNOSUPPORT: i32 = 44;
/// Operation not supported
pub const EOPNOTSUPP: i32 = 45;
/// Operation not supported
pub const ENOTSUP: i32 = 45;
/// Protocol family not supported
pub const EPFNOSUPPORT: i32 = 46;
/// Address family not supported by protocol family
pub const EAFNOSUPPORT: i32 = 47;
/// Address already in use
pub const EADDRINUSE: i32 = 48;
/// Can't assign requested address
pub const EADDRNOTAVAIL: i32 = 49;
/// Network is down
pub const ENETDOWN: i32 = 50;
/// Network is unreachable
pub const ENETUNREACH: i32 = 51;
/// Network dropped connection on reset
pub const ENETRESET: i32 = 52;
/// Software caused connection abort
pub const ECONNABORTED: i32 = 53;
/// Connection reset by peer
pub const ECONNRESET: i32 = 54;
/// No buffer space available
pub const ENOBUFS: i32 = 55;
/// Socket is already connected
pub const EISCONN: i32 = 56;
/// Socket is not connected
pub const ENOTCONN: i32 = 57;
/// Can't send after socket shutdown
pub const ESHUTDOWN: i32 = 58;
/// Too many references: can't splice
pub const ETOOMANYREFS: i32 = 59;
/// Operation timed out
pub const ETIMEDOUT: i32 = 60;
/// Connection refused
pub const ECONNREFUSED: i32 = 61;
/// Too many levels of symbolic links
pub const ELOOP: i32 = 62;
/// File name too long
pub const ENAMETOOLONG: i32 = 63;
/// Host is down
pub const EHOSTDOWN: i32 = 64;
/// No route to host
pub const EHOSTUNREACH: i32 = 65;
/// Directory not empty
pub const ENOTEMPTY: i32 = 66;
/// Too many processes
pub const EPROCLIM: i32 = 67;
/// Too many users
pub const EUSERS: i32 = 68;
/// Disc quota exceeded
pub const EDQUOT: i32 = 69;
/// Stale NFS file handle
pub const ESTALE: i32 = 70;
/// Too many levels of remote in path
pub const EREMOTE: i32 = 71;
/// RPC struct is bad
pub const EBADRPC: i32 = 72;
/// RPC version wrong
pub const ERPCMISMATCH: i32 = 73;
/// RPC prog. not avail
pub const EPROGUNAVAIL: i32 = 74;
/// Program version wrong
pub const EPROGMISMATCH: i32 = 75;
/// Bad procedure for program
pub const EPROCUNAVAIL: i32 = 76;
/// No locks available
pub const ENOLCK: i32 = 77;
/// Function not implemented
pub const ENOSYS: i32 = 78;
/// Inappropriate file type or format
pub const EFTYPE: i32 = 79;
/// Authentication error
pub const EAUTH: i32 = 80;
/// Need authenticator
pub const ENEEDAUTH: i32 = 81;
/// Identifier removed
pub const EIDRM: i32 = 82;
/// No message of desired type
pub const ENOMSG: i32 = 83;
/// Value too large to be stored in data type
pub const EOVERFLOW: i32 = 84;
/// Operation canceled
pub const ECANCELED: i32 = 85;
/// Illegal byte sequence
pub const EILSEQ: i32 = 86;
/// Attribute not found
pub const ENOATTR: i32 = 87;
/// Programming error
pub const EDOOFUS: i32 = 88;
/// Bad message
pub const EBADMSG: i32 = 89;
/// Multihop attempted
pub const EMULTIHOP: i32 = 90;
/// Link has been severed
pub const ENOLINK: i32 = 91;
/// Protocol error
pub const EPROTO: i32 = 92;
/// Capabilities insufficient
pub const ENOTCAPABLE: i32 = 93;
/// Not permitted in capability mode
pub const ECAPMODE: i32 = 94;
/// State not recoverable
pub const ENOTRECOVERABLE: i32 = 95;
/// Previous owner died
pub const EOWNERDEAD: i32 = 96;
/// Must be equal largest errno
pub const ELAST: i32 = 96;

/// # Bash error codes
/// Uses the range 126-165

/// Command invoked cannot execute
pub const ENOTEXEC: i32 = 126;

/// Command not found
pub const ENOCMD: i32 = 127;

/// Invalid argument to exit
pub const EINVALEXIT: i32 = 128;

/// Terminated by CTRL-C
pub const ECTRLC: i32 = 130;

/// # My custom errors

/// UTF8 decode/encode error
pub const EUTF8: i32 = 166;

impl fmt::Display for PosixError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return write!(f, "{}", self.message);
    }
}

impl From<std::io::Error> for PosixError {
    #[inline]
    fn from(error: std::io::Error) -> Self {
        match error.kind() {
            ErrorKind::NotFound => Self {
                code: ENOENT,
                message: error.to_string(),
            },
            ErrorKind::PermissionDenied => Self {
                code: EACCES,
                message: error.to_string(),
            },
            ErrorKind::ConnectionRefused => Self {
                code: ECONNREFUSED,
                message: error.to_string(),
            },

            ErrorKind::ConnectionReset => Self {
                code: ECONNRESET,
                message: error.to_string(),
            },
            ErrorKind::ConnectionAborted => Self {
                code: ECONNABORTED,
                message: error.to_string(),
            },
            ErrorKind::NotConnected => Self {
                code: ENOTCONN,
                message: error.to_string(),
            },
            ErrorKind::AddrInUse => Self {
                code: EADDRINUSE,
                message: error.to_string(),
            },
            ErrorKind::AddrNotAvailable => Self {
                code: EADDRNOTAVAIL,
                message: error.to_string(),
            },
            ErrorKind::BrokenPipe => Self {
                code: EPIPE,
                message: error.to_string(),
            },
            ErrorKind::AlreadyExists => Self {
                code: EEXIST,
                message: error.to_string(),
            },
            ErrorKind::WouldBlock => Self {
                code: EWOULDBLOCK,
                message: error.to_string(),
            },
            ErrorKind::InvalidInput => Self {
                code: EINVAL,
                message: error.to_string(),
            },
            ErrorKind::InvalidData => Self {
                code: EFTYPE,
                message: error.to_string(),
            },
            ErrorKind::TimedOut => Self {
                code: ETIMEDOUT,
                message: error.to_string(),
            },
            ErrorKind::WriteZero => Self {
                code: ENOLINK,
                message: error.to_string(),
            },
            ErrorKind::Interrupted => Self {
                code: EINTR,
                message: error.to_string(),
            },
            ErrorKind::UnexpectedEof => Self {
                code: ESHUTDOWN,
                message: error.to_string(),
            },
            _ => Self {
                code: EPERM,
                message: error.to_string(),
            },
        }
    }
}

impl From<std::process::Output> for PosixError {
    #[inline]
    fn from(output: std::process::Output) -> Self {
        let tmp = String::from_utf8_lossy(&output.stderr).to_string();
        let mut code = output.status.code().unwrap_or(1);
        if code == 0 {
            // This should not happen, but who knows.
            code = 1;
        }
        Self::new(code, tmp)
    }
}

impl PosixError {
    /// Create a new [`PosixError`]
    #[must_use]
    #[inline]
    pub fn new(code: i32, message: String) -> Self {
        Self { code, message }
    }

    /// Return the posix error code
    #[must_use]
    #[inline]
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Return the error message
    #[must_use]
    #[inline]
    pub fn message(&self) -> String {
        self.message.clone()
    }
}

#[must_use]
#[inline]
pub fn posix_error(code: i32, msg: &str) -> PosixError {
    PosixError::new(code, msg.to_owned())
}

/// Convert [`std::io::Error`] to a [`PosixError`]
#[allow(clippy::needless_pass_by_value)]
#[deprecated(since = "1.1.0", note = "Please use PosixError::from")]
#[must_use]
#[inline]
pub fn to_posix_error(err: std::io::Error) -> PosixError {
    PosixError::from(err)
}

/// Return a [`PosixError`] from a failed [`std::process::Output`]
#[must_use]
#[inline]
#[deprecated(since = "1.1.0", note = "Please use PosixError::from")]
pub fn error_from_output(output: std::process::Output) -> PosixError {
    PosixError::from(output)
}
