import { createActor, canisterId } from "$declarations/meddle_backend"
import { HttpAgent } from "@dfinity/agent"
import { getIdentity } from "./auth"

export function backendCanister() {
	const identity = getIdentity() ?? undefined
	const agent = new HttpAgent({ identity })

	return createActor(canisterId, {
		agent
	})
}
