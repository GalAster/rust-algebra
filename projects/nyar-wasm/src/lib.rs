use crate::encoder::{CanonicalImport, WastEncoder};
pub use crate::{
    dag::DependentGraph,
    encoder::{encode_id, encode_kebab, CanonicalWasi},
    instances::WasiInstance,
    operations::{
        branch::{JumpBranch, JumpCondition, JumpTable},
        infix::{InfixCall, InfixOperator},
        looping::{LoopEach, LoopRepeat, LoopUntilBody, LoopWhileBody},
        WasiInstruction,
    },
    symbols::{
        exports::WasiExport,
        identifiers::Identifier,
        imports::WasiImport,
        wasi_publisher::{WasiModule, WasiPublisher},
    },
    wasi_types::{
        array::WasiArrayType,
        enumerations::{WasiEnumeration, WasiSemanticIndex},
        flags::WasiFlags,
        functions::{WasiFunction, WasiFunctionBody, WasiParameter},
        records::{WasiRecordField, WasiRecordType},
        reference::{WasiOwnership, WasiTypeReference},
        resources::WasiResource,
        variants::{WasiVariantItem, WasiVariantType},
        WasiType,
    },
    wasi_values::{array::ArrayValue, record::RecordValue, WasiValue},
};

mod dag;
mod encoder;
pub mod helpers;
mod instances;
mod operations;
mod symbols;
mod wasi_types;
mod wasi_values;
