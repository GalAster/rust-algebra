use std::{
    fmt::{Debug, Display, Formatter},
    ops::Range,
};

use valkyrie_errors::{FileID, FileSpan};

use crate::{ValkyrieASTKind, ValkyrieASTNode};

impl Debug for ValkyrieASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieASTKind::Statement(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Namespace(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Unary(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Binary(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Identifier(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Integer(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Decimal(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Boolean(v) => Debug::fmt(v, f),
            ValkyrieASTKind::Null => {
                write!(f, "null")
            }
            ValkyrieASTKind::HList(v) => {
                f.debug_struct("Tuple")
                    .field("nodes", v)
                    .finish()
            }
        }
    }
}

impl Display for ValkyrieASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ValkyrieASTKind::Statement(_) => {todo!()}
            ValkyrieASTKind::Namespace(_) => {todo!()}
            ValkyrieASTKind::Binary(_) => {todo!()}
            ValkyrieASTKind::Unary(_) => {todo!()}
            ValkyrieASTKind::HList(_) => {todo!()}
            ValkyrieASTKind::Identifier(_) => {todo!()}
            ValkyrieASTKind::Decimal(v) => {Display::fmt(v, f)}
            ValkyrieASTKind::Integer(v) => {Display::fmt(v, f)}
            ValkyrieASTKind::Boolean(_) => {todo!()}
            ValkyrieASTKind::Null => {
                f.write_str("null")
            }

        }
    }
}

impl ValkyrieASTKind {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode { kind: self, span: FileSpan { file, head: range.start, tail: range.end } }
    }
}
