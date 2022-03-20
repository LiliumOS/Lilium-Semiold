use core::fmt::{Error, Write};
use stivale_boot::v2::StivaleTerminalTag;

pub struct TerminalWriter<'a> {
    internal: &'a StivaleTerminalTag,
    _not_thread_safe: core::marker::PhantomData<&'a mut ()>,
}

impl<'a> TerminalWriter<'a> {
    pub fn new(internal: &'a StivaleTerminalTag) -> Self {
        Self { internal, _not_thread_safe: core::marker::PhantomData }
    }
}

impl Write for TerminalWriter<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), Error> {
        Ok(self.internal.term_write()(s))
    }
}
