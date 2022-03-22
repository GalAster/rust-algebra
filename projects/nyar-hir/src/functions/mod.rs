use crate::{Identifier, IndexedIterator, NyarType, NyarValue, Symbol};
use indexmap::IndexMap;
use nyar_error::NyarError;
use std::slice::Iter;

pub mod externals;
pub mod keywords;
pub mod resolver;

#[derive(Default)]
pub struct FunctionRegister {
    native: IndexMap<String, FunctionItem>,
    external: IndexMap<String, FunctionExternalItem>,
}

/// `@ffi("module", "field")`
pub struct FunctionExternalItem {
    pub module: Symbol,
    pub field: Symbol,
    pub input: Vec<NyarType>,
    pub output: Vec<NyarType>,
}

/// `function`
pub struct FunctionItem {
    pub namepath: Identifier,
    pub export: bool,
    pub entry: bool,
    pub input: Vec<NyarType>,
    pub output: Vec<NyarType>,
    pub body: FunctionBody,
}

impl FunctionItem {
    pub fn new(path: Identifier) -> Self {
        Self { namepath: path, export: false, entry: false, input: vec![], output: vec![], body: FunctionBody::default() }
    }
    pub fn name(&self) -> String {
        self.namepath.to_string()
    }
    pub fn with_public(self) -> Self {
        Self { export: true, ..self }
    }
    pub fn with_inputs<I>(mut self, inputs: I) -> Self
    where
        I: IntoIterator<Item = NyarType>,
    {
        self.input = inputs.into_iter().collect();
        self
    }
    pub fn with_outputs<I>(mut self, outputs: I) -> Self
    where
        I: IntoIterator<Item = NyarType>,
    {
        self.output = outputs.into_iter().collect();
        self
    }
    pub fn with_operations<I>(mut self, operations: I) -> Self
    where
        I: IntoIterator<Item = Operation>,
    {
        self.body.codes = operations.into_iter().collect();
        self
    }
}

#[derive(Default)]
pub struct FunctionBody {
    codes: Vec<Operation>,
}

impl<'i> IntoIterator for &'i FunctionBody {
    type Item = &'i Operation;
    type IntoIter = Iter<'i, Operation>;

    fn into_iter(self) -> Self::IntoIter {
        self.codes.iter()
    }
}

#[derive(Debug)]
pub enum Operation {
    GlobalGet { index: u32 },
    LocalGet { index: u32 },
    LocalSet { index: u32 },
    Constant { value: NyarValue },
    NativeSum { native: NativeDataType, terms: Vec<Operation> },
    NativeEqual { native: NativeDataType, terms: Vec<Operation> },
    NativeEqualZero { native: NativeDataType, term: Box<Operation> },
}

#[derive(Debug)]
pub enum NativeDataType {
    I32,
    I64,
    F32,
    F64,
}

impl FunctionRegister {
    pub fn get_id(&self, name: &str) -> Result<usize, NyarError> {
        match self.native.get_full(name) {
            Some((index, _, _)) => return Ok(index),
            None => {}
        }
        match self.external.get_full(name) {
            Some((index, _, _)) => return Ok(self.native.len() + index),
            None => {}
        }
        Err(NyarError::custom(format!("missing function {name}")))
    }
    pub fn add_native(&mut self, item: FunctionItem) {
        self.native.insert(item.namepath.to_string(), item);
    }
    pub fn get_natives(&self) -> IndexedIterator<FunctionItem> {
        IndexedIterator::new(&self.native).with_index(self.external.len())
    }
    pub fn add_external(&mut self, item: FunctionExternalItem) {
        self.external.insert(item.name(), item);
    }
    pub fn get_externals(&self) -> IndexedIterator<FunctionExternalItem> {
        IndexedIterator::new(&self.external)
    }
}
