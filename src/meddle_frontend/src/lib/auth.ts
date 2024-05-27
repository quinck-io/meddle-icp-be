import { AuthClient } from "@dfinity/auth-client"

let authClient: AuthClient | null = null

export async function initAuth() {
	if (!authClient) {
		authClient = await AuthClient.create()
	}
}

export async function login() {
	if (!authClient) {
		await initAuth()
	}
	const url =
		process.env.DFX_NETWORK === "local"
			? `http://localhost:4943/?canisterId=${process.env.CANISTER_ID_INTERNET_IDENTITY}`
			: `https://${process.env.CANISTER_ID_INTERNET_IDENTITY}.ic0.app`

	await authClient!.login({
		identityProvider: url,
		onSuccess: () => {
			window.location.reload()
		}
	})
}

export function logout() {
	if (authClient) {
		authClient.logout()
		window.location.reload()
	}
}

export async function isAuthenticated() {
	if (!authClient) {
		await initAuth()
	}
	return await authClient!.isAuthenticated()
}

export async function getIdentity() {
	if (!authClient) {
		await initAuth()
	}
	return authClient!.getIdentity()
}
