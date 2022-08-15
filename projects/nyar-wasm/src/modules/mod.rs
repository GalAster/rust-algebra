use crate::{
    functions::FunctionType, DataItem, DataSection, ExternalSection, ExternalType, FunctionSection, GlobalSection, TypeItem,
    TypeSection, WasmVariable,
};
use nyar_error::NyarError;

mod wast_component;
mod wast_module;

#[derive(Default)]
pub struct ModuleBuilder {
    name: String,
    entry: String,
    memory_pages: u64,
    globals: GlobalSection,
    types: TypeSection,
    data: DataSection,
    functions: FunctionSection,
    externals: ExternalSection,
}

impl ModuleBuilder {
    pub fn new<S: ToString>(name: S) -> Self {
        Self { name: name.to_string(), ..Default::default() }
    }

    pub fn get_module_name(&self) -> &str {
        &self.name
    }
    pub fn set_module_name<S: ToString>(&mut self, name: S) {
        self.name = name.to_string();
    }

    pub fn insert_type<T: Into<TypeItem>>(&mut self, t: T) -> Option<TypeItem> {
        self.types.insert(t.into())
    }
    pub fn insert_function(&mut self, f: FunctionType) {
        if f.entry {
            self.entry = f.name()
        }
        self.functions.insert(f)
    }
    pub fn insert_external(&mut self, f: ExternalType) -> Option<ExternalType> {
        self.externals.insert(f)
    }
    pub fn insert_data(&mut self, item: DataItem) -> Option<DataItem> {
        self.data.insert(item)
    }
    pub fn insert_global(&mut self, global: WasmVariable) -> Option<WasmVariable> {
        self.globals.insert(global)
    }
}
