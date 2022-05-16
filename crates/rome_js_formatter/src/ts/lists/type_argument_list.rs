use crate::formatter::TrailingSeparator;
use crate::generated::FormatTsTypeArgumentList;
use crate::prelude::*;
use rome_js_syntax::TsTypeArgumentList;

impl FormatRule<TsTypeArgumentList> for FormatTsTypeArgumentList {
    type Options = JsFormatOptions;

    fn format(
        node: &TsTypeArgumentList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::Disallowed)?,
        ))
    }
}
