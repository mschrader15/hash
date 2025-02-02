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

use super::{batch_generated::*, metaversion_generated::*};

extern crate flatbuffers;

use self::flatbuffers::{EndianScalar, Follow};

pub enum StateSyncOffset {}

#[derive(Copy, Clone, PartialEq)]
pub struct StateSync<'a> {
    pub _tab: flatbuffers::Table<'a>,
}

impl<'a> flatbuffers::Follow<'a> for StateSync<'a> {
    type Inner = StateSync<'a>;

    #[inline]
    fn follow(buf: &'a [u8], loc: usize) -> Self::Inner {
        Self {
            _tab: flatbuffers::Table { buf, loc },
        }
    }
}

impl<'a> StateSync<'a> {
    pub const VT_AGENT_POOL: flatbuffers::VOffsetT = 4;
    pub const VT_CURRENT_STEP: flatbuffers::VOffsetT = 8;
    pub const VT_MESSAGE_POOL: flatbuffers::VOffsetT = 6;

    #[inline]
    pub fn init_from_table(table: flatbuffers::Table<'a>) -> Self {
        StateSync { _tab: table }
    }

    #[allow(unused_mut)]
    pub fn create<'bldr: 'args, 'args: 'mut_bldr, 'mut_bldr>(
        _fbb: &'mut_bldr mut flatbuffers::FlatBufferBuilder<'bldr>,
        args: &'args StateSyncArgs<'args>,
    ) -> flatbuffers::WIPOffset<StateSync<'bldr>> {
        let mut builder = StateSyncBuilder::new(_fbb);
        builder.add_current_step(args.current_step);
        if let Some(x) = args.message_pool {
            builder.add_message_pool(x);
        }
        if let Some(x) = args.agent_pool {
            builder.add_agent_pool(x);
        }
        builder.finish()
    }

    #[inline]
    pub fn agent_pool(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch>>,
            >>(StateSync::VT_AGENT_POOL, None)
            .unwrap()
    }

    #[inline]
    pub fn message_pool(&self) -> flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>> {
        self._tab
            .get::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch>>,
            >>(StateSync::VT_MESSAGE_POOL, None)
            .unwrap()
    }

    #[inline]
    pub fn current_step(&self) -> i64 {
        self._tab
            .get::<i64>(StateSync::VT_CURRENT_STEP, Some(0))
            .unwrap()
    }
}

impl flatbuffers::Verifiable for StateSync<'_> {
    #[inline]
    fn run_verifier(
        v: &mut flatbuffers::Verifier,
        pos: usize,
    ) -> Result<(), flatbuffers::InvalidFlatbuffer> {
        use self::flatbuffers::Verifiable;
        v.visit_table(pos)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Batch>>,
            >>(&"agent_pool", Self::VT_AGENT_POOL, true)?
            .visit_field::<flatbuffers::ForwardsUOffset<
                flatbuffers::Vector<'_, flatbuffers::ForwardsUOffset<Batch>>,
            >>(&"message_pool", Self::VT_MESSAGE_POOL, true)?
            .visit_field::<i64>(&"current_step", Self::VT_CURRENT_STEP, false)?
            .finish();
        Ok(())
    }
}

pub struct StateSyncArgs<'a> {
    pub agent_pool: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>>>,
    >,
    pub message_pool: Option<
        flatbuffers::WIPOffset<flatbuffers::Vector<'a, flatbuffers::ForwardsUOffset<Batch<'a>>>>,
    >,
    pub current_step: i64,
}

impl<'a> Default for StateSyncArgs<'a> {
    #[inline]
    fn default() -> Self {
        StateSyncArgs {
            agent_pool: None,   // required field
            message_pool: None, // required field
            current_step: 0,
        }
    }
}

pub struct StateSyncBuilder<'a: 'b, 'b> {
    fbb_: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    start_: flatbuffers::WIPOffset<flatbuffers::TableUnfinishedWIPOffset>,
}

impl<'a: 'b, 'b> StateSyncBuilder<'a, 'b> {
    #[inline]
    pub fn add_agent_pool(
        &mut self,
        agent_pool: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Batch<'b>>>,
        >,
    ) {
        self.fbb_
            .push_slot_always::<flatbuffers::WIPOffset<_>>(StateSync::VT_AGENT_POOL, agent_pool);
    }

    #[inline]
    pub fn add_message_pool(
        &mut self,
        message_pool: flatbuffers::WIPOffset<
            flatbuffers::Vector<'b, flatbuffers::ForwardsUOffset<Batch<'b>>>,
        >,
    ) {
        self.fbb_.push_slot_always::<flatbuffers::WIPOffset<_>>(
            StateSync::VT_MESSAGE_POOL,
            message_pool,
        );
    }

    #[inline]
    pub fn add_current_step(&mut self, current_step: i64) {
        self.fbb_
            .push_slot::<i64>(StateSync::VT_CURRENT_STEP, current_step, 0);
    }

    #[inline]
    pub fn new(_fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>) -> StateSyncBuilder<'a, 'b> {
        let start = _fbb.start_table();
        StateSyncBuilder {
            fbb_: _fbb,
            start_: start,
        }
    }

    #[inline]
    pub fn finish(self) -> flatbuffers::WIPOffset<StateSync<'a>> {
        let o = self.fbb_.end_table(self.start_);
        self.fbb_
            .required(o, StateSync::VT_AGENT_POOL, "agent_pool");
        self.fbb_
            .required(o, StateSync::VT_MESSAGE_POOL, "message_pool");
        flatbuffers::WIPOffset::new(o.value())
    }
}

impl std::fmt::Debug for StateSync<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut ds = f.debug_struct("StateSync");
        ds.field("agent_pool", &self.agent_pool());
        ds.field("message_pool", &self.message_pool());
        ds.field("current_step", &self.current_step());
        ds.finish()
    }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_root_as_state_sync<'a>(buf: &'a [u8]) -> StateSync<'a> {
    unsafe { flatbuffers::root_unchecked::<StateSync<'a>>(buf) }
}

#[inline]
#[deprecated(since = "2.0.0", note = "Deprecated in favor of `root_as...` methods.")]
pub fn get_size_prefixed_root_as_state_sync<'a>(buf: &'a [u8]) -> StateSync<'a> {
    unsafe { flatbuffers::size_prefixed_root_unchecked::<StateSync<'a>>(buf) }
}

#[inline]
/// Verifies that a buffer of bytes contains a `StateSync`
/// and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_sync_unchecked`.
pub fn root_as_state_sync(buf: &[u8]) -> Result<StateSync, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root::<StateSync>(buf)
}

#[inline]
/// Verifies that a buffer of bytes contains a size prefixed
/// `StateSync` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `size_prefixed_root_as_state_sync_unchecked`.
pub fn size_prefixed_root_as_state_sync(
    buf: &[u8],
) -> Result<StateSync, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root::<StateSync>(buf)
}

#[inline]
/// Verifies, with the given options, that a buffer of bytes
/// contains a `StateSync` and returns it.
/// Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_sync_unchecked`.
pub fn root_as_state_sync_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<StateSync<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::root_with_opts::<StateSync<'b>>(opts, buf)
}

#[inline]
/// Verifies, with the given verifier options, that a buffer of
/// bytes contains a size prefixed `StateSync` and returns
/// it. Note that verification is still experimental and may not
/// catch every error, or be maximally performant. For the
/// previous, unchecked, behavior use
/// `root_as_state_sync_unchecked`.
pub fn size_prefixed_root_as_state_sync_with_opts<'b, 'o>(
    opts: &'o flatbuffers::VerifierOptions,
    buf: &'b [u8],
) -> Result<StateSync<'b>, flatbuffers::InvalidFlatbuffer> {
    flatbuffers::size_prefixed_root_with_opts::<StateSync<'b>>(opts, buf)
}

#[inline]
/// Assumes, without verification, that a buffer of bytes contains a StateSync and returns it.
///
/// # Safety
///
/// Callers must trust the given bytes do indeed contain a valid `StateSync`.
pub unsafe fn root_as_state_sync_unchecked(buf: &[u8]) -> StateSync {
    flatbuffers::root_unchecked::<StateSync>(buf)
}

#[inline]
/// Assumes, without verification, that a buffer of bytes contains a size prefixed StateSync and
/// returns it.
///
/// # Safety
///
/// Callers must trust the given bytes do indeed contain a valid size prefixed `StateSync`.
pub unsafe fn size_prefixed_root_as_state_sync_unchecked(buf: &[u8]) -> StateSync {
    flatbuffers::size_prefixed_root_unchecked::<StateSync>(buf)
}

#[inline]
pub fn finish_state_sync_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<StateSync<'a>>,
) {
    fbb.finish(root, None);
}

#[inline]
pub fn finish_size_prefixed_state_sync_buffer<'a, 'b>(
    fbb: &'b mut flatbuffers::FlatBufferBuilder<'a>,
    root: flatbuffers::WIPOffset<StateSync<'a>>,
) {
    fbb.finish_size_prefixed(root, None);
}
