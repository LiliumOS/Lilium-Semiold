#![no_std]

pub trait Device {
    fn seek() -> bool;
}
