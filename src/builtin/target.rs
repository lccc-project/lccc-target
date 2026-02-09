use target_tuples::{Architecture, Environment, OS, ObjectFormat};

use crate::{
    helpers::CowPtr,
    properties::{ExtPropertyValue, target::Target},
};

use super::*;

/// Determines the default (`extern "C"`) tag for the target
pub const fn default_tag_for(
    arch: Architecture,
    os: OS,
    objfmt: Option<ObjectFormat>,
) -> Option<&'static str> {
    match (arch, os, objfmt) {
        (
            Architecture::X86_64,
            OS::Linux | OS::Lilium | OS::FreeBSD | OS::NetBSD | OS::OpenBSD | OS::MacOSX,
            _,
        )
        | (Architecture::X86_64, _, Some(ObjectFormat::Elf)) => Some("sysv64"),
        (Architecture::X86_64, OS::Win32, _) => Some("win64"),
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Linux | OS::Lilium | OS::FreeBSD | OS::NetBSD | OS::OpenBSD | OS::MacOSX,
            _,
        )
        | (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            _,
            Some(ObjectFormat::Elf),
        ) => Some("cdecl-unix"),
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Win32,
            _,
        ) => Some("cdecl-ms"),
        _ => None,
    }
}

/// Determines the system tag (`extern "system"`) for the target
pub const fn system_tag_for(
    arch: Architecture,
    os: OS,
    objfmt: Option<ObjectFormat>,
) -> Option<&'static str> {
    match (arch, os, objfmt) {
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Lilium,
            _,
        ) => Some("fastcall-unix"),
        (
            Architecture::I386
            | Architecture::I486
            | Architecture::I586
            | Architecture::I686
            | Architecture::I786,
            OS::Win32,
            _,
        ) => Some("stdcall-ms"),
        _ => None,
    }
}

macro_rules! const_try_option {
    ($expr:expr) => {
        match $expr {
            Some(val) => val,
            None => return None,
        }
    };
}

/// Computers the properties of a specfied [`Target`][target_tuples::Target].
pub fn from_target(targ: &target_tuples::Target) -> Option<Target> {
    let os_name = match targ.operating_system() {
        Some(name) => name,
        None => OS::None,
    };
    let arch = const_try_option!(archs::from_target(targ.arch()));
    let os = const_try_option!(os::from_target(os_name));
    let link = const_try_option!(link::from_target(
        targ.arch(),
        os_name,
        targ.environment(),
        targ.object_format(),
    ));

    let default_tag =
        const_try_option!(default_tag_for(targ.arch(), os_name, targ.object_format()));
    let system_tag = match system_tag_for(targ.arch(), os_name, targ.object_format()) {
        Some(tag) => tag,
        None => default_tag,
    };

    let primitive_layout = const_try_option!(abi::primitives_from_target(
        targ.arch(),
        os_name,
        targ.environment()
    ));
    let abi = const_try_option!(abi::abi_from_target(
        targ.arch(),
        os_name,
        targ.environment()
    ));

    let mut target = Target {
        arch: CowPtr::Borrowed(arch),
        os: CowPtr::Borrowed(os),
        default_tag: CowPtr::Borrowed(default_tag),
        system_tag: CowPtr::Borrowed(system_tag),
        primitive_layout: CowPtr::Borrowed(primitive_layout),
        abi: CowPtr::Borrowed(abi),
        link: CowPtr::Borrowed(link),
        override_features: slice![],
        extended_properties: slice![],
    };

    match (
        targ.arch(),
        os_name,
        targ.environment(),
        targ.object_format(),
    ) {
        (arch, OS::None, _, _) if arch.is_x86() => {
            target.override_features = slice![
                (cowstr!("x87"), false),
                (cowstr!("fxsr"), false),
                (cowstr!("xsave"), false),
            ]
        }
        (Architecture::X86_64, OS::None, _, _) => {
            target.override_features = slice![
                (cowstr!("x87"), false),
                (cowstr!("fxsr"), false),
                (cowstr!("xsave"), false),
            ]
        }
        (arch, OS::Lilium, Some(Environment::Kernel), _) if arch.is_x86() => {
            target.override_features = slice![(cowstr!("xsave"), false)];
            target.extended_properties = slice![(
                cowstr!("rust.abi.rustcall-tag"),
                ExtPropertyValue::String(cowstr!("fastcall-unix"))
            )];
        }
        (arch, OS::Lilium | OS::Win32, _, _) if arch.is_x86() => {
            target.extended_properties = slice![(
                cowstr!("rust.abi.rustcall-tag"),
                ExtPropertyValue::String(cowstr!("fastcall-unix"))
            )];
        }

        (Architecture::X86_64, OS::Lilium, Some(Environment::Kernel), _) => {
            target.override_features = slice![(cowstr!("xsave"), false)];
        }
        _ => {}
    }

    Some(target)
}
