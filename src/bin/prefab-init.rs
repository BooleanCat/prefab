extern crate nix;

use std::{mem, ptr};
use nix::{libc, errno, unistd};

fn main() {
    let mut action = unsafe{ mem::uninitialized::<libc::sigaction>() };
    sigaction(libc::SIGCHLD, ptr::null(), &mut action).unwrap();

    action.sa_flags |= libc::SA_NOCLDWAIT;
    sigaction(libc::SIGCHLD, &action, ptr::null_mut()).unwrap();

    loop { unistd::pause(); }
}

fn sigaction(signum: libc::c_int, act: *const libc::sigaction, oldact: *mut libc::sigaction) -> nix::Result<()> {
    errno::Errno::result(unsafe { libc::sigaction(signum, act, oldact) }).and(Ok(()))
}
