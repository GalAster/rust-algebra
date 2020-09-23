use super::*;

const UNARY: &str = r#"
+1;
2!;
+3!;
-+4!!;
"#;

#[test]
fn debug_unary() -> Result<()> {
    let ast: ASTKind = ASTDump::parse(UNARY);
    ast.save("tests/test_expr/debug_unary.yaml")
}
