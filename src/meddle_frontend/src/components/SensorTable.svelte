<script lang="ts">
	import { backendCanister } from "$lib/canisters"
	import { encodeRecords, type SensorData, type SensorRecord } from "$lib/types"
	import { DataTable, Toolbar, ToolbarContent, Button, Pagination } from "carbon-components-svelte"
	import { createEventDispatcher, onMount } from "svelte"
	import refresh from "carbon-icons-svelte/lib/renew.svelte"

	let records: SensorRecord[] = []
	let totalRecords = 0
	let currentPage = 1
	let pageSize = 10
	let lastRefresh = new Date().toLocaleTimeString()
	$: currentOffset = (currentPage - 1) * pageSize

	const dispatch = createEventDispatcher<{ addrecord: void }>()

	onMount(async () => {
		await fetchRecords(0, pageSize)
	})

	async function fetchRecords(offset: number, limit: number) {
		const { data, len } = await backendCanister().get_data(offset, limit, true)
		records = encodeRecords(data)
		totalRecords = len
		lastRefresh = new Date().toLocaleTimeString()
	}

	async function handlePageChange(page: number) {
		currentPage = page
		await fetchRecords(currentOffset, pageSize)
	}

	async function handlePageSizeChange(size: number) {
		pageSize = size
		await fetchRecords(currentOffset, pageSize)
	}
</script>

<DataTable
	title="Sensor data"
	headers={[
		{ key: "sensorId", value: "Sensor ID" },
		{ key: "value", value: "Value" },
		{ key: "unitId", value: "Unit ID" },
		{ key: "timestamp", value: "Timestamp" }
	]}
	rows={records}
	description="Last refresh: {lastRefresh}"
>
	<Toolbar>
		<ToolbarContent>
			<Button
				kind="ghost"
				icon={refresh}
				iconDescription="Refresh data"
				on:click={async () => {
					await fetchRecords(currentOffset, pageSize)
				}}
			></Button>
			<Button
				on:click={() => {
					dispatch("addrecord")
				}}>Add record</Button
			>
		</ToolbarContent>
	</Toolbar>
</DataTable>

<Pagination
	totalItems={totalRecords}
	pageSizes={[10, 50, 100]}
	{pageSize}
	page={currentPage}
	on:click:button--next={async () => await handlePageChange(currentPage + 1)}
	on:click:button--previous={async () => await handlePageChange(currentPage - 1)}
	on:update={async (e) => {
		await handlePageSizeChange(e.detail.pageSize)
		await handlePageChange(e.detail.page)
	}}
/>
