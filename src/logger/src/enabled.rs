//! Contains implementation of default logger.

use enso_prelude::*;

use crate::AnyLogger;
use crate::Message;

use enso_shapely::CloneRef;
use std::fmt::Debug;

#[cfg(target_arch = "wasm32")]
use web_sys::console;
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::JsValue;



// ==============
// === Logger ===
// ==============

/// Default Logger implementation.
#[derive(Clone,CloneRef,Debug,Default)]
pub struct Logger {
    /// Path that is used as an unique identifier of this logger.
    path         : ImString,
    trace_copies : TraceCopies,
    #[cfg(not(target_arch="wasm32"))]
    indent       : Rc<Cell<usize>>,
}

#[cfg(not(target_arch="wasm32"))]
impl Logger {
    fn format(&self, msg:impl Message) -> String {
        let indent = " ".repeat(4*self.indent.get());
        msg.with(|s|iformat!("{indent}[{self.path}] {s}"))
    }

    fn inc_indent(&self) {
        self.indent.update(|t|t.saturating_add(1));
    }

    fn dec_indent(&self) {
        self.indent.update(|t|t.saturating_sub(1));
    }
}

#[cfg(target_arch="wasm32")]
impl Logger {
    fn format(&self, msg:impl Message) -> JsValue {
        msg.with(|s|iformat!("[{self.path}] {s}")).into()
    }
}

#[cfg(not(target_arch="wasm32"))]
impl AnyLogger for Logger {
    type Owned = Self;
    fn new(path:impl Into<ImString>) -> Self {
        let path         = path.into();
        let indent       = default();
        let trace_copies = default();
        Self {path,indent,trace_copies}
    }
    fn path        (&self) -> &str           { &self.path }
    fn trace       (&self, msg:impl Message) { println!("{}",self.format(msg)) }
    fn debug       (&self, msg:impl Message) { println!("{}",self.format(msg)) }
    fn info        (&self, msg:impl Message) { println!("{}",self.format(msg)) }
    fn warning     (&self, msg:impl Message) { println!("[WARNING] {}",self.format(msg)) }
    fn error       (&self, msg:impl Message) { println!("[ERROR] {}",self.format(msg)) }
    fn group_begin (&self, msg:impl Message) { println!("{}",self.format(msg)); self.inc_indent() }
    fn group_end   (&self)                   { self.dec_indent() }
    fn trace_copies(&self)                   { self.trace_copies.enable(&self.path) }
}


#[cfg(target_arch="wasm32")]
impl AnyLogger for Logger {
    type Owned = Self;
    fn new(path:impl Into<ImString>) -> Self {
        let path         = path.into();
        let trace_copies = default();
        Self {path,trace_copies}
    }
    fn path        (&self) -> &str           { &self.path }
    fn trace       (&self, msg:impl Message) { console::trace_1 (&self.format(msg)) }
    fn debug       (&self, msg:impl Message) { console::debug_1 (&self.format(msg)) }
    fn info        (&self, msg:impl Message) { console::info_1  (&self.format(msg)) }
    fn warning     (&self, msg:impl Message) { console::warn_1  (&self.format(msg)) }
    fn error       (&self, msg:impl Message) { console::error_1 (&self.format(msg)) }
    fn group_begin (&self, msg:impl Message) { console::group_1 (&self.format(msg)) }
    fn group_end   (&self)                   { console::group_end() }
    fn trace_copies(&self)                   { self.trace_copies.enable(&self.path) }
}



// ===================
// === Conversions ===
// ===================

impls!{ From + &From <crate::disabled::Logger> for Logger { |logger| Self::new(logger.path()) }}
