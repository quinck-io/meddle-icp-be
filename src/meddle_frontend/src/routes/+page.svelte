<script lang="ts">
	import { Loading, Modal, TextArea } from "carbon-components-svelte"
	import SensorTable from "../components/SensorTable.svelte"
	import { backendCanister } from "$lib/canisters"
	import { decodeSensorData, sensorDataSchema } from "$lib/types"
	import { fromError } from "zod-validation-error"
	import { jsonAreaPlaceholder } from "$lib/consts"
	import { isAuthenticated } from "$lib/auth"
	import { onMount } from "svelte"

	let modalOpen = false
	let authenticated = false

	let jsonValue = ""
	let modalError = ""
	let modalLoading = false

	onMount(async () => {
		authenticated = await isAuthenticated()
	})

	async function handleAddRecord() {
		let parsed
		try {
			parsed = JSON.parse(jsonValue)
		} catch {
			modalError = "Invalid JSON"
			return
		}

		const result = sensorDataSchema.safeParse(parsed)
		if (!result.success || !result.data) {
			modalError = fromError(result.error).message
			return
		}

		modalError = ""
		modalLoading = true
		await backendCanister().post_data([decodeSensorData(result.data)])

		modalLoading = false
		modalOpen = false
		window.location.reload()
	}
</script>

{#if authenticated}
	<SensorTable on:addrecord={() => (modalOpen = true)} />
{:else}
	<p>To view data, please login with Internet Identity.</p>
{/if}

<Modal
	bind:open={modalOpen}
	modalHeading="Add record manually"
	primaryButtonText="Confirm"
	secondaryButtonText="Cancel"
	on:click:button--secondary={() => (modalOpen = false)}
	on:submit={handleAddRecord}
>
	<TextArea
		rows={20}
		labelText="JSON data"
		placeholder={jsonAreaPlaceholder}
		bind:value={jsonValue}
	/>
	{#if modalLoading}
		<Loading small withOverlay={false} />
	{/if}
	<p style="color: orangered;">{modalError}</p>
</Modal>
