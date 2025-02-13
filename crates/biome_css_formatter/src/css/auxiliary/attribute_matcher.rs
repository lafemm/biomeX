use crate::prelude::*;
use biome_css_syntax::CssAttributeMatcher;
use biome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatCssAttributeMatcher;
impl FormatNodeRule<CssAttributeMatcher> for FormatCssAttributeMatcher {
    fn fmt_fields(&self, node: &CssAttributeMatcher, f: &mut CssFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
