//! windows_x86_64 target stub — name, triple, and loud refusal.
//!
//! Every port method returns `TargetError::Unsupported`: this crate lets
//! target discovery enumerate Windows ahead of any backend work, but it
//! can never emit or execute code (there is deliberately no encoder,
//! image writer, or JIT here — selecting this target fails instead).

use rubyc::native::container;
use rubyc::native::ir::Program;
use rubyc::native::target::{
    BytecodeLoader, CodeGenBackend, ImageWriter, InputKind, MachineCode, MemoryManager, Target,
    TargetError,
};

pub struct Backend;
pub struct Writer;
pub struct Loader;
pub struct Memory;

pub const BACKEND: Backend = Backend;
pub const IMAGE_WRITER: Writer = Writer;
pub const LOADER: Loader = Loader;
pub const MEMORY: Memory = Memory;

impl CodeGenBackend for Backend {
    fn lower(&self, _program: &Program) -> Result<MachineCode, TargetError> {
        Err(TargetError::Unsupported(Target::WindowsX64))
    }
}

impl ImageWriter for Writer {
    fn write_image(&self, _mc: &MachineCode, _entry_preamble: bool) -> Result<Vec<u8>, TargetError> {
        Err(TargetError::Unsupported(Target::WindowsX64))
    }
}

impl BytecodeLoader for Loader {
    fn sniff(&self, bytes: &[u8]) -> InputKind {
        // Windows has no shebang contract; the container magic alone
        // decides, mirroring the Linux policy minus its first step.
        if bytes.starts_with(&container::MAGIC) {
            InputKind::Bytecode {
                payload_start: container::MAGIC.len(),
            }
        } else {
            InputKind::Source
        }
    }
}

impl MemoryManager for Memory {
    fn map_rw(&self, _len: usize) -> Result<u64, std::io::Error> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "windows_x86_64 has no implementation yet",
        ))
    }

    fn protect_exec(&self, _addr: u64, _len: usize) -> Result<(), std::io::Error> {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "windows_x86_64 has no implementation yet",
        ))
    }

    fn unmap(&self, _addr: u64, _len: usize) {}
}

pub struct WindowsX86_64Backend;

impl rubyc::extensions::TargetExtension for WindowsX86_64Backend {
    fn metadata(&self) -> &rubyc::extensions::ExtensionMetadata {
        static META: std::sync::LazyLock<rubyc::extensions::ExtensionMetadata> =
            std::sync::LazyLock::new(|| rubyc::extensions::ExtensionMetadata {
                name: "windows_x86_64".into(),
                display_name: "Windows x86-64".into(),
                author: "RubyC contributors".into(),
                version: env!("CARGO_PKG_VERSION").into(),
                description: "Not implemented".into(),
                license: "MIT".into(),
                homepage: String::new(),
                repository: String::new(),
            });
        &META
    }

    fn triple(&self) -> &str {
        "x86_64-pc-windows-msvc"
    }
}

impl rubyc_target::TargetBackend for WindowsX86_64Backend {
    fn name(&self) -> &'static str {
        "windows_x86_64"
    }

    fn triple(&self) -> &'static str {
        "x86_64-pc-windows-msvc"
    }

    fn lower(&self, _program: &[u8]) -> Result<Vec<u8>, String> {
        Err("windows_x86_64 has no implementation yet".into())
    }

    fn write_image(&self, _mc: &[u8], _entry_preamble: bool) -> Result<Vec<u8>, String> {
        Err("windows_x86_64 has no implementation yet".into())
    }
}

// ── Plugin ABI entry points ────────────────────────────────────────────────
//
// Same shape as the Linux plugin so the host loader accepts this crate,
// but every call errors: a stub must be selectable for discovery and
// refuse on use, never emit.

use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_name() -> *const c_char {
    static NAME: &[u8] = b"windows_x86_64\0";
    NAME.as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_triple() -> *const c_char {
    static TRIPLE: &[u8] = b"x86_64-pc-windows-msvc\0";
    TRIPLE.as_ptr() as *const c_char
}

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_base_addr() -> u64 {
    0
}

fn unsupported() -> *mut rubyc::plugin::PluginBytes {
    Box::into_raw(Box::new(rubyc::plugin::PluginBytes::error(
        "windows_x86_64 has no implementation yet".to_owned(),
    )))
}

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_lower(_ir: *const c_char, _len: u64) -> *mut rubyc::plugin::PluginBytes {
    unsupported()
}

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_write_image(
    _mc: *const c_char,
    _len: u64,
    _entry_preamble: u8,
) -> *mut rubyc::plugin::PluginBytes {
    unsupported()
}

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_write_shared(
    _mc: *const c_char,
    _len: u64,
) -> *mut rubyc::plugin::PluginBytes {
    unsupported()
}

#[unsafe(no_mangle)]
pub extern "C" fn rbxt_run(_mc: *const c_char, _len: u64) -> i32 {
    1
}

#[cfg(test)]
#[path = "../tests/mod.rs"]
mod tests;
