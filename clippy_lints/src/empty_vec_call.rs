use clippy_utils::diagnostics::span_lint_and_sugg;
use clippy_utils::{
    consts::{constant, Constant},
    higher,
    source::snippet,
};
use if_chain::if_chain;
use rustc_ast::ast::LitKind;
use rustc_errors::Applicability;
use rustc_hir::{ArrayLen, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_session::{declare_lint_pass, declare_tool_lint};

declare_clippy_lint! {
    /// ### What it does
    /// Warns the user that empty initializations of vectors and arrays can cause side effects.
    ///
    /// ### Why is this bad?
    /// The initialized object is immediately dropped and cannot be accessed.
    /// However, the initialization may introduce unexpected side effects.
    ///
    /// ### Example
    /// ```rust
    /// let x = vec![side_effect(); 0];
    /// let y = [side_effect(); 0];
    /// ```
    /// Use instead:
    /// ```rust
    /// let x = vec![];
    /// let y = [] as [usize; 0];
    /// ```
    #[clippy::version = "1.62.0"]
    pub EMPTY_VEC_CALL,
    correctness,
    "empty initializations of vectors and arrays can cause side effects"
}
declare_lint_pass!(EmptyVecCall => [EMPTY_VEC_CALL]);

impl<'tcx> LateLintPass<'tcx> for EmptyVecCall {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'_>) {
        if_chain! {
            if let Some(higher::VecArgs::Repeat(elem, len)) = higher::VecArgs::hir(cx, expr);
            if let Some((Constant::Int(len_constant), _)) = constant(cx, cx.typeck_results(), len);
            if len_constant == 0;
            if let ExprKind::Call(_, _) = elem.kind;
            then {
                let span = expr.span.ctxt().outer_expn_data().call_site;
                span_lint_and_sugg(
                    cx,
                    EMPTY_VEC_CALL,
                    span,
                    &format!("result of `{}` is unused in empty vec", snippet(cx, elem.span, "elem")),
                    "consider using",
                    "vec![]".to_string(),
                    Applicability::MaybeIncorrect,
                )
            }
        }

        if_chain! {
            if let ExprKind::Repeat(elem, ArrayLen::Body(anon_const)) = expr.kind;
            let len_expr = &cx.tcx.hir().body(anon_const.body).value;
            if let ExprKind::Lit(ref lit) = len_expr.kind;
            if let LitKind::Int(0, _) = lit.node;
            if let ExprKind::Call(_,_) = elem.kind;
            then {
                let ty = cx.typeck_results().expr_ty(elem);
                span_lint_and_sugg(
                    cx,
                    EMPTY_VEC_CALL,
                    expr.span,
                    &format!("result of `{}` is unused in empty array", snippet(cx, elem.span, "elem")),
                    "consider using",
                    format!("[] as [{}; 0]", ty),
                    Applicability::MaybeIncorrect,
                )
            }
        }
    }
}
