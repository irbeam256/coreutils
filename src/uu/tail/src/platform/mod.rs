// This file is part of the uutils coreutils package.
//
// For the full copyright and license information, please view the LICENSE
// file that was distributed with this source code.

#[cfg(unix)]
pub use self::unix::{
    Pid,
    ProcessChecker,
    //stdin_is_bad_fd, stdin_is_pipe_or_fifo, supports_pid_checks, Pid, ProcessChecker,
    supports_pid_checks,
};

#[cfg(windows)]
pub use self::windows::{Pid, ProcessChecker, supports_pid_checks};

#[cfg(unix)]
mod unix;

#[cfg(windows)]
mod windows;
