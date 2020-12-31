
use std::os::unix::io::{AsRawFd, RawFd, IntoRawFd};
use libc::{c_int, c_uint, c_char};
use std::ffi::{CString};
use std::io::{Read, Write, Result};



extern {
    fn create_macvtap(ifname: *const c_char, device: *mut c_char, mtu: c_uint) -> c_int;
    fn create_tap(ifname: *const c_char, device: *mut c_char, mtu: c_uint) -> c_int;
}


#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Mode {
    Tun,
    Tap,
    MacvTap
}


#[derive(Debug)]
pub struct Iface {
    fd: i32
}


impl Iface {
    /// Creates a new Iface.
    ///
    /// # Examples
    /// ```
    /// modprobe macvtap
    /// 
    /// ip link add link ens192 name tap0 type macvtap mode bridge
    /// 
    /// ```
    ///
    /// ```
    /// use macvtap::{Iface, Mode};
    /// use std::{io::{Read, Write, Result}, vec};
    /// 
    /// #sync read/write
    /// let mut iface = Iface::new("tap0", Mode::MacvTap, 1500)?;
    /// let mut buf = vec![0;1504];
    /// buf.read(&mut buf);
    ///
    /// # async read/write
    ///  use smol::{Async};
    ///  
    ///  let async_iface = Async::new(iface)?;
    ///  async_iface.read(&mut buf).await;
    /// ```
    pub fn new(ifname: &str, mode: Mode, mtu: u32) -> Result<Self> {
        let fd = unsafe {
            let t = CString::new(ifname).unwrap();
            let iface_name = t.as_ptr();
            let mut device = vec![0i8; 256];
            match mode {
                Mode::Tap => { create_tap(iface_name, device.as_mut_ptr(), mtu) },
                Mode::MacvTap => { create_macvtap(iface_name, device.as_mut_ptr(), mtu) },
                _ => { -1 }
            }
        };

        if fd < 0 {
            use std::io::{Error, ErrorKind};
            return Err(Error::new(ErrorKind::Other, "unable to create tap"));
        }

        Ok(Self {
            fd
        })
    }

    pub fn close(&mut self) -> i32 {
        unsafe {
            libc::close(self.fd)
        }
    }
}


impl AsRawFd for Iface {
    fn as_raw_fd(&self) -> RawFd {
        self.fd
    }
}


impl IntoRawFd for Iface {
    fn into_raw_fd(self) -> RawFd {
        self.fd
    }
}


impl Read for &Iface {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        assert!(buf.len() <= isize::max_value() as usize);
        match unsafe { libc::read(self.fd, buf.as_mut_ptr() as _, buf.len()) } {
            x if x < 0 => Err(std::io::Error::last_os_error()),
            x => Ok(x as usize),
        }
    }
}


impl Read for Iface {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        assert!(buf.len() <= isize::max_value() as usize);
        match unsafe { libc::read(self.fd, buf.as_mut_ptr() as _, buf.len()) } {
            x if x < 0 => Err(std::io::Error::last_os_error()),
            x => Ok(x as usize),
        }
    }
}


impl Write for &Iface {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        assert!(buf.len() <= isize::max_value() as usize);
        match unsafe { libc::write(self.fd, buf.as_ptr() as _, buf.len()) } {
            x if x < 0 => Err(std::io::Error::last_os_error()),
            x => Ok(x as usize),
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}


impl Write for Iface {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        assert!(buf.len() <= isize::max_value() as usize);
        match unsafe { libc::write(self.fd, buf.as_ptr() as _, buf.len()) } {
            x if x < 0 => Err(std::io::Error::last_os_error()),
            x => Ok(x as usize),
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}


impl Drop for Iface {
    fn drop(&mut self) {
        let _ = self.close();
    }
}





























#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}



