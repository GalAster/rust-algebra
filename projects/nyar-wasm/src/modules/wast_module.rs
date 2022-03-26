use crate::{
    helpers::{Id, WasmOutput},
    modules::ModuleBuilder,
};
use nyar_error::NyarError;
use wast::{
    core::{
        Custom, Expression, Global, GlobalKind, GlobalType, Module, ModuleField, ModuleKind, Producers, Type, TypeDef, ValType,
    },
    token::{NameAnnotation, Span},
    Wat,
};

impl ModuleBuilder {
    fn wast_producer(&self) -> ModuleField {
        let item = Custom::Producers(Producers {
            fields: vec![
                ("language", vec![("valkyrie", "2024"), ("player", "berserker")]),
                ("processed-by", vec![("nyar-wasm", env!("CARGO_PKG_VERSION"))]),
            ],
        });
        ModuleField::Custom(item)
    }
    pub fn build_module(&self) -> Result<Wat, NyarError> {
        let mut terms = Vec::with_capacity(1024);
        for (_, _, k) in self.functions.get_externals() {
            terms.push(ModuleField::Import(k.as_wast()))
        }
        for (_, _, k) in self.types.into_iter() {
            terms.push(k.as_wast())
        }
        for (_, _, k) in self.globals.into_iter() {
            terms.push(ModuleField::Global(k.as_wast()))
        }
        terms.push(self.wast_producer());
        Ok(Wat::Module(Module {
            span: Span::from_offset(0),
            id: None,
            name: Some(NameAnnotation { name: "runtime" }),
            kind: ModuleKind::Text(terms),
        }))
    }
}
