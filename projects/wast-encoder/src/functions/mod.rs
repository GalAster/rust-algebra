use std::{
    fmt::{Display, Formatter, Write},
    ops::AddAssign,
    sync::Arc,
};

use crate::{dag::DependentGraph, DependenciesTrace, Identifier, wasi_types::AliasExport, WasiModule, WasiType, WastEncoder};

mod arithmetic;
mod display;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ExternalFunction {
    pub symbol: Identifier,
    pub wasi_module: WasiModule,
    pub wasi_name: String,
    pub inputs: Vec<WasiParameter>,
    pub output: Option<WasiType>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WasiParameter {
    pub name: Arc<str>,
    pub wasi_name: Arc<str>,

    pub r#type: WasiType,
}

impl ExternalFunction {
    pub fn new<S, M>(wasi_module: M, wasi_name: &str, name: S) -> Self
    where
        S: Into<Identifier>,
        M: Into<WasiModule>,
    {
        Self {
            symbol: name.into(),
            wasi_module: wasi_module.into(),
            wasi_name: wasi_name.to_string(),
            inputs: vec![],
            output: None,
        }
    }
}

impl WasiParameter {
    pub fn new<S>(name: S, r#type: WasiType) -> Self
    where
        S: Into<Arc<str>>,
    {
        let wasi_name = name.into();
        Self { name: wasi_name.clone(), wasi_name, r#type }
    }
}
impl AliasExport for ExternalFunction {
    fn alias_export<W: Write>(&self, w: &mut WastEncoder<W>, module: &WasiModule) -> std::fmt::Result {
        let id = self.symbol.wasi_id();
        let name = self.wasi_name.as_str();
        write!(w, "(alias export ${module} \"{name}\" (func {id}))")
    }
}
impl DependenciesTrace for ExternalFunction {
    fn define_language_types(&self, dict: &mut DependentGraph) {
        dict.types.insert(self.symbol.clone(), WasiType::External(Box::new(self.clone())));
    }

    fn collect_wasi_types<'a, 'i>(&'a self, dict: &'i DependentGraph, collected: &mut Vec<&'i WasiType>)
    where
        'a: 'i,
    {
        self.inputs.iter().for_each(|input| input.r#type.collect_wasi_types(dict, collected));
        self.output.iter().for_each(|output| output.collect_wasi_types(dict, collected));
    }
}
