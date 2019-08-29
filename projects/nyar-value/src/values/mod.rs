use std::{
    collections::{HashMap, LinkedList},
    fmt::{Debug, Display, Formatter},
    ops::Deref,
    sync::{Arc, Mutex},
    time::Instant,
};

use indexmap::IndexMap;
use num::BigInt;
use shredder::{atomic::AtomicGc, Gc, Scan};
use smartstring::{LazyCompact, SmartString};

use crate::{
    values::{integer::NyarInteger, listing::AnyList},
    NyarClass,
};

mod integer;
mod listing;
mod string;

/// Internal [`NyarValue`] representation.
///
/// Box variants to reduce the size.
#[repr(u8)]
#[derive(Scan)]
pub enum NyarValue {
    /// Something wrong happened
    Never,
    /// The unit return
    Unit,
    /// A boolean value
    Bool(bool),
    /// An integer value.
    Unsigned8(u8),
    /// An integer value.
    Unsigned16(u16),
    /// An integer value.
    Unsigned32(u32),
    /// An integer value.
    Unsigned64(u64),
    /// An integer value.
    Unsigned128(u128),
    /// An integer value.
    Integer8(i8),
    /// An integer value.
    Integer16(i16),
    /// An integer value.
    Integer32(i32),
    /// An integer value.
    Integer64(i64),
    /// An integer value.
    Integer128(i128),
    /// An integer value.
    Integer(Gc<NyarInteger>),
    /// A UTF8 character value
    Character(char),
    /// An [`StringView`] value
    String(Gc<String>),
    /// An array value.
    AnyList(Gc<AnyList>),
    /// An blob (byte array).
    AnyDict(Gc<AnyDict>),
    /// A function pointer.
    FunctionPointer,
    /// Any type as a trait object.
    AnyObject,
}

#[derive(Debug, Scan)]
pub struct Float32(f32);

#[derive(Debug, Scan)]
pub struct Float64(f64);

#[derive(Debug, Scan)]
pub struct ByteArray {
    inner: Vec<u8>,
}

#[derive(Debug, Scan)]
pub struct AnyDict {
    // TODO: using some faster non-safe hasher
    inner: HashMap<String, NyarValue>,
}

impl Debug for NyarValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            NyarValue::Never => f.write_str("!"),
            NyarValue::Unit => f.write_str("()"),
            NyarValue::Bool(v) => Display::fmt(v, f),
            NyarValue::Unsigned8(v) => Display::fmt(v, f),
            NyarValue::Unsigned16(v) => Display::fmt(v, f),
            NyarValue::Unsigned32(v) => Display::fmt(v, f),
            NyarValue::Unsigned64(v) => Display::fmt(v, f),
            NyarValue::Unsigned128(v) => Display::fmt(v, f),
            NyarValue::Integer8(v) => Display::fmt(v, f),
            NyarValue::Integer16(v) => Display::fmt(v, f),
            NyarValue::Integer32(v) => Display::fmt(v, f),
            NyarValue::Integer64(v) => Display::fmt(v, f),
            NyarValue::Integer128(v) => Display::fmt(v, f),
            NyarValue::Integer(v) => Display::fmt(v, f),
            NyarValue::Character(v) => Display::fmt(v, f),
            NyarValue::String(_) => unimplemented!(),
            NyarValue::AnyList(_) => unimplemented!(),
            NyarValue::AnyDict(_) => unimplemented!(),
            NyarValue::FunctionPointer => unimplemented!(),
            NyarValue::AnyObject => unimplemented!(),
        }
    }
}
