use super::*;

impl<'a, 'i> IntoWasm<'a, wast::component::InlineExport<'i>> for Option<WasmExportName>
where
    'a: 'i,
{
    fn as_wast(&'a self) -> wast::component::InlineExport<'i> {
        let names = match &self {
            Some(s) => vec![ComponentExternName(s.inner.as_ref())],
            None => vec![],
        };
        wast::component::InlineExport { names }
    }
}
impl<'a, 'i> IntoWasm<'a, wast::core::InlineExport<'i>> for Option<WasmExportName>
where
    'a: 'i,
{
    fn as_wast(&'a self) -> wast::core::InlineExport<'i> {
        let names = match &self {
            Some(s) => vec![s.inner.as_ref()],
            None => vec![],
        };
        wast::core::InlineExport { names }
    }
}

impl WasmSymbol {
    pub(crate) fn as_index<'a, 'i>(&'a self) -> Index<'i>
    where
        'a: 'i,
    {
        Index::Id(WasmName::new(self.inner.as_ref()))
    }
    pub(crate) fn as_id<'a, 'i>(&'a self) -> Option<Id<'i>>
    where
        'a: 'i,
    {
        Some(WasmName::new(self.inner.as_ref()))
    }
}
