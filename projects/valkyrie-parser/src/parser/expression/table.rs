use valkyrie_ast::HeterogeneousList;

use crate::parser::valkyrie::{KeyNode, KeyValueNode, TableStatement, ValueNode};

use super::*;

impl TableStatement {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        let mut pairs = vec![];
        for _row in &self.args {
            // pairs.push(row.visit(parser)?)
        }
        Ok(ValkyrieASTNode::dict(pairs, parser.file, &self.position))
    }
}

impl KeyValueNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<HeterogeneousList> {
        let key = self.key.visit(parser)?;
        let value = self.value.visit(parser)?;
        Ok(HeterogeneousList::pair(key, value))
    }
}

impl KeyNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        match self {
            KeyNode::IdentifierNode(v) => {
                v.as_identifier(parser);
                unimplemented!()
            }
            KeyNode::StringNode(v) => {
                v.visit(parser)?;
                unimplemented!()
            }
        }
        todo!()
    }
}

impl ValueNode {
    pub fn visit(&self, parser: &mut ValkyrieParser) -> ValkyrieResult<ValkyrieASTNode> {
        match self {
            ValueNode::IntegerNode(v) => {
                v.visit(parser)?;
                unimplemented!()
            }
            ValueNode::StringNode(v) => {
                v.visit(parser)?;
                unimplemented!()
            }
        }
        todo!()
    }
}
