<script lang="ts">
	import { Loading, Modal, TextArea } from "carbon-components-svelte"
	import SensorTable from "../components/SensorTable.svelte"
	import { backendCanister } from "$lib/canisters"
	import { decodeSensorData, sensorDataSchema } from "$lib/types"
	import { fromError } from "zod-validation-error"
	import { jsonAreaPlaceholder } from "$lib/consts"

	let modalOpen = false

	let jsonValue = ""
	let modalError = ""
	let modalLoading = false

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

<SensorTable on:addrecord={() => (modalOpen = true)} />

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
