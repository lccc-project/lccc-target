use target_tuples::pieces::{Architecture, Environment, OS, ObjectFormat};

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
            Architecture::X86_64 { .. },
            OS::Linux | OS::Lilium | OS::FreeBSD | OS::NetBSD | OS::OpenBSD | OS::MacOSX,
            _,
        )
        | (Architecture::X86_64 { .. }, _, Some(ObjectFormat::Elf)) => Some("sysv64"),
        (Architecture::X86_64 { .. }, OS::Win32, _) => Some("win64"),
        (
            Architecture::X86_32(_),
            OS::Linux | OS::Lilium | OS::FreeBSD | OS::NetBSD | OS::OpenBSD | OS::MacOSX,
            _,
        )
        | (Architecture::X86_32(_), _, Some(ObjectFormat::Elf)) => Some("cdecl-unix"),
        (Architecture::X86_32(_), OS::Win32, _) => Some("cdecl-ms"),
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
        (Architecture::X86_32(_), OS::Lilium, _) => Some("fastcall-unix"),
        (Architecture::X86_32(_), OS::Win32, _) => Some("stdcall-ms"),
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

/// Computers the properties of a specfied [`TargetRef`][target_tuples::TargetRef].
pub fn from_target(targ: &target_tuples::TargetRef) -> Option<Target> {
    let sysname = targ.sys;
    let os_name = match sysname.os() {
        Some(name) => name,
        None => OS::None,
    };
    let arch = const_try_option!(archs::from_target(targ.arch));
    let os = const_try_option!(os::from_target(os_name));
    let link = const_try_option!(link::from_target(targ.arch, sysname,));

    let default_tag =
        const_try_option!(default_tag_for(targ.arch, os_name, sysname.object_format()));
    let system_tag = match system_tag_for(targ.arch, os_name, sysname.object_format()) {
        Some(tag) => tag,
        None => default_tag,
    };

    let primitive_layout = const_try_option!(abi::primitives_from_target(
        targ.arch,
        os_name,
        sysname.env()
    ));
    let abi = const_try_option!(abi::abi_from_target(targ.arch, os_name, sysname.env()));

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

    match (targ.arch, os_name, sysname.env(), sysname.object_format()) {
        (Architecture::X86_16(_) | Architecture::X86_32(_), OS::None, _, _) => {
            target.override_features = slice![
                (cowstr!("x87"), false),
                (cowstr!("fxsr"), false),
                (cowstr!("xsave"), false),
            ]
        }
        (Architecture::X86_64 { .. }, OS::None, _, _) => {
            target.override_features = slice![
                (cowstr!("x87"), false),
                (cowstr!("fxsr"), false),
                (cowstr!("xsave"), false),
            ]
        }
        (
            Architecture::X86_16(_) | Architecture::X86_32(_),
            OS::Lilium,
            Some(Environment::Kernel),
            _,
        ) => {
            target.override_features = slice![(cowstr!("xsave"), false)];
            target.extended_properties = slice![(
                cowstr!("rust.abi.rustcall-tag"),
                ExtPropertyValue::String(cowstr!("fastcall-unix"))
            )];
        }
        (Architecture::X86_16(_) | Architecture::X86_32(_), OS::Lilium | OS::Win32, _, _) => {
            target.extended_properties = slice![(
                cowstr!("rust.abi.rustcall-tag"),
                ExtPropertyValue::String(cowstr!("fastcall-unix"))
            )];
        }

        (Architecture::X86_64 { .. }, OS::Lilium, Some(Environment::Kernel), _) => {
            target.override_features = slice![(cowstr!("xsave"), false)];
        }
        _ => {}
    }

    Some(target)
}
