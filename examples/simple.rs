
use macvtap::{Iface, Mode};
use std::{io::{Read}, vec};



fn main() {
    let mut iface = Iface::new("macvtap-01", Mode::MacvTap, 1500).unwrap();
    let mut buf =  vec![0;1504];
    iface.read(&mut buf).unwrap();
}