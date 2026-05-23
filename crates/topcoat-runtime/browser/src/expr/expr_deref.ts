import { type Expr, interpret } from "./index";
import type { Interpreter } from "./interpreter";

export type ExprDeref = {
	type: "Deref";
	inner: Expr;
};

export function interpret_expr_deref(
	expr: ExprDeref,
	interpreter: Interpreter,
): unknown {
	const target = interpret(expr.inner, interpreter);
	return (target as () => unknown)();
}
