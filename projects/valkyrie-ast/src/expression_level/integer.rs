use std::fmt::{Display, Formatter};

use super::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ValkyrieIntegerNode {
    pub hint: ValkyrieIdentifierNode,
    pub value: IBig,
}

impl Display for ValkyrieIntegerNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.hint.name)
    }
}

impl ValkyrieIntegerNode {
    pub fn to_node(self, file: FileID, range: &Range<usize>) -> ValkyrieASTNode {
        ValkyrieASTNode {
            kind: ValkyrieASTKind::Integer(box self),
            span: FileSpan { file, head: range.start, tail: range.end },
        }
    }
}
