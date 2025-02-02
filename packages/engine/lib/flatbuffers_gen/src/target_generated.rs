#![allow(
    clippy::module_name_repetitions,
    clippy::must_use_candidate,
    clippy::cast_sign_loss,
    clippy::empty_enum,
    clippy::used_underscore_binding,
    clippy::redundant_static_lifetimes,
    clippy::redundant_field_names,
    unused_imports
)]
// automatically generated by the FlatBuffers compiler, do not modify

use std::{cmp::Ordering, mem};

extern crate flatbuffers;

use self::flatbuffers::{EndianScalar, Follow};

#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MIN_TARGET: i8 = 0;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
pub const ENUM_MAX_TARGET: i8 = 4;
#[deprecated(
    since = "2.0.0",
    note = "Use associated constants instead. This will no longer be generated in 2021."
)]
#[allow(non_camel_case_types)]
pub const ENUM_VALUES_TARGET: [Target; 5] = [
    Target::Python,
    Target::JavaScript,
    Target::Rust,
    Target::Main,
    Target::Dynamic,
];

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[repr(transparent)]
pub struct Target(pub i8);

#[allow(non_upper_case_globals)]
impl Target {
    pub const Dynamic: Self = Self(4);
    pub const ENUM_MAX: i8 = 4;
    pub const ENUM_MIN: i8 = 0;
    pub const ENUM_VALUES: &'static [Self] = &[
        Self::Python,
        Self::JavaScript,
        Self::Rust,
        Self::Main,
        Self::Dynamic,
    ];
    pub const JavaScript: Self = Self(1);
    pub const Main: Self = Self(3);
    pub const Python: Self = Self(0);
    pub const Rust: Self = Self(2);

    /// Returns the variant's name or "" if unknown.
    pub fn variant_name(self) -> Option<&'static str> {
        match self {
            Self::Python => Some("Python"),
            Self::JavaScript => Some("JavaScript"),
            Self::Rust => Some("Rust"),
            Self::Main => Some("Main"),
            Self::Dynamic => Some("Dynamic"),
            _ => None,
        }
    }
}

impl std::fmt::Debug for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        if let Some(name) = self.variant_name() {
            f.write_str(name)
        } else {
            f.write_fmt(format_args!("<UNKNOWN {:?}>", self.0))
        }
    }
}

impl<'a> flatbuffers::Follow<'a> for Target {
    type Inner = Self;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        let b = unsafe { flatbuffers::read_scalar_at::<i8>(buf, loc) };
        Self(b)
    }
}

impl flatbuffers::Push for Target {
    type Output = Target;

    #[inline]
    fn push(&self, dst: &mut [u8], _rest: &[u8]) {
        unsafe {
            flatbuffers::emplace_scalar::<i8>(dst, self.0);
        }
    }
}

impl flatbuffers::EndianScalar for Target {
    #[inline]
    fn to_little_endian(self) -> Self {
        let b = i8::to_le(self.0);
        Self(b)
    }

    #[inline]
    #[allow(clippy::wrong_self_convention)]
    fn from_little_endian(self) -> Self {
        let b = i8::from_le(self.0);
        Self(b)
    }
}

impl<'a> flatbuffers::Verifiable for Target {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        i8::run_verifier(v, pos)
    }
}

impl flatbuffers::SimpleToVerifyInSlice for Target {}
