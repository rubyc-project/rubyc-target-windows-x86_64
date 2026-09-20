//! Windows stub pins: identity plus loud refusal on every port.

use super::{BACKEND, IMAGE_WRITER, LOADER, MEMORY};
use rubyc::native::container;
use rubyc::native::ir::Program;
use rubyc::native::target::{
    BytecodeLoader, CodeGenBackend, ImageWriter, InputKind, MachineCode, MemoryManager, Target,
    TargetError,
};
use std::ffi::CStr;
use std::os::raw::c_char;

fn empty_program() -> Program {
    Program {
        functions: vec![],
        statics: vec![],
        vtables: vec![],
        imports: vec![],
        exports: vec![],
        strings: vec![],
        heap_size: 0,
        names: vec![],
        entry: None,
        init: None,
    }
}

fn empty_mc() -> MachineCode {
    MachineCode {
        code: vec![],
        data: vec![],
        fn_bases: vec![],
        vtable_offsets: vec![],
        vtable_names: vec![],
        static_offsets: vec![],
        static_names: vec![],
        helper_itoa: None,
        helper_itoa_err: None,
        helper_scan_int: None,
        helper_retain: None,
        helper_release: None,
        helper_alloc: None,
        helper_exc_enter: None,
        helper_exc_pop: None,
        helper_exc_throw: None,
        helper_exc_rethrow: None,
        helper_exc_load: None,
        helper_exc_clear: None,
        exc_state_offset: None,
        helper_argv: None,
        import_names: vec![],
        import_bases: vec![],
        import_abs: vec![],
        exports: vec![],
        export_takes_receiver: vec![],
        init_fn: None,
        relocs: vec![],
        heap_size: 0,
    }
}

#[test]
fn triple_is_msvc() {
    let triple =
        unsafe { CStr::from_ptr(super::rbxt_triple() as *const c_char) }.to_string_lossy();
    assert_eq!(triple, "x86_64-pc-windows-msvc");
}

#[test]
fn backend_name_matches() {
    let name = unsafe { CStr::from_ptr(super::rbxt_name() as *const c_char) }.to_string_lossy();
    assert_eq!(name, "windows_x86_64");
}

#[test]
fn lower_refuses_loudly() {
    let err = BACKEND.lower(&empty_program()).unwrap_err();
    assert_eq!(err, TargetError::Unsupported(Target::WindowsX64));
}

#[test]
fn image_writer_refuses_loudly() {
    let err = IMAGE_WRITER.write_image(&empty_mc(), true).unwrap_err();
    assert_eq!(err, TargetError::Unsupported(Target::WindowsX64));
}

#[test]
fn loader_sniffs_magic_without_shebang() {
    let mut bytes = container::MAGIC.to_vec();
    bytes.extend_from_slice(&[0x01, 0x02]);
    assert!(matches!(
        LOADER.sniff(&bytes),
        InputKind::Bytecode { payload_start: 8 }
    ));
    assert!(matches!(LOADER.sniff(b"class c {}"), InputKind::Source));
}

#[test]
fn memory_refuses_loudly() {
    assert!(MEMORY.map_rw(4096).is_err());
}
