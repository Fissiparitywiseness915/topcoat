import type { WriteSignal } from "@maverick-js/signals";
import type { SignalId, SignalRegistry } from "../signal";

export class Interpreter {
	public constructor(private readonly registry: SignalRegistry) {}

	public getSignal(id: SignalId): WriteSignal<unknown> {
		return this.registry.handle(id);
	}
}
