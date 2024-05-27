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
	await authClient!.login({
		identityProvider: "http://localhost:4943/?canisterId=bkyz2-fmaaa-aaaaa-qaaaq-cai",
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
