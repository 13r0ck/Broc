use broc_ast::lang::core::expr::expr2::{Expr2, ExprId};
use broc_ast::mem_pool::pool_str::PoolStr;
use broc_code_markup::slow_pool::MarkNodeId;

use crate::editor::ed_error::EdResult;
use crate::editor::mvc::app_update::InputOutcome;
use crate::editor::mvc::ed_model::EdModel;
use crate::ui::text::lines::SelectableLines;

pub fn update_invalid_lookup(
    input_str: &str,
    old_pool_str: &PoolStr,
    curr_mark_node_id: MarkNodeId,
    expr_id: ExprId,
    ed_model: &mut EdModel,
) -> EdResult<InputOutcome> {
    if input_str.chars().all(|ch| ch.is_ascii_alphanumeric()) {
        let mut new_lookup_str = String::new();

        new_lookup_str.push_str(old_pool_str.as_str(ed_model.module.env.pool));

        let caret_offset = ed_model
            .grid_node_map
            .get_offset_to_node_id(ed_model.get_caret(), curr_mark_node_id)?;

        new_lookup_str.insert_str(caret_offset, input_str);

        let new_pool_str = PoolStr::new(&new_lookup_str, ed_model.module.env.pool);

        // update AST
        ed_model
            .module
            .env
            .pool
            .set(expr_id, Expr2::InvalidLookup(new_pool_str));

        // update caret
        ed_model.simple_move_carets_right(input_str.len());

        Ok(InputOutcome::Accepted)
    } else {
        Ok(InputOutcome::Ignored)
    }
}
