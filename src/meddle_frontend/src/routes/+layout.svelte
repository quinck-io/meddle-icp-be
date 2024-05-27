<script lang="ts">
	import { getIdentity, initAuth, isAuthenticated, login, logout } from "$lib/auth"
	import type { Identity } from "@dfinity/agent"
	import { Content, Header, HeaderGlobalAction, HeaderUtilities } from "carbon-components-svelte"
	import "carbon-components-svelte/css/g90.css"
	import UserIcon from "carbon-icons-svelte/lib/UserAvatarFilledAlt.svelte"
	import LogoutIcon from "carbon-icons-svelte/lib/Logout.svelte"
	import LoginIcon from "carbon-icons-svelte/lib/Login.svelte"
	import { onMount } from "svelte"

	let authenticated: boolean
	let identity: Identity | null

	onMount(async () => {
		await initAuth()
		authenticated = await isAuthenticated()
		identity = authenticated ? await getIdentity() : null
	})

	function handleLogin() {
		login()
	}

	function handleLogout() {
		logout()
	}
</script>

<main>
	<Header company="Quinck" platformName="Meddle ICP Demo">
		<HeaderUtilities>
			{#if authenticated}
				<HeaderGlobalAction
					iconDescription="Logged in as: {identity?.getPrincipal().toString()}"
					tooltipAlignment="end"
					icon={UserIcon}
				/>
				<HeaderGlobalAction
					iconDescription="Logout"
					tooltipAlignment="end"
					icon={LogoutIcon}
					on:click={handleLogout}
				/>
			{:else}
				<HeaderGlobalAction
					iconDescription="Login"
					tooltipAlignment="end"
					icon={LoginIcon}
					on:click={handleLogin}
				/>
			{/if}
		</HeaderUtilities>
	</Header>
	<Content>
		<slot />
	</Content>
</main>
