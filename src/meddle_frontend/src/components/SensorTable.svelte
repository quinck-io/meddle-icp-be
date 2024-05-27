<script lang="ts">
	import { backendCanister } from "$lib/canisters"
	import { DUMMY_IN } from "$lib/consts"
	import { decodeSensorData, encodeRecords, type SensorRecord } from "$lib/types"
	import { DataTable, Toolbar, ToolbarContent, Button, Pagination } from "carbon-components-svelte"
	import { onMount } from "svelte"

	let records: SensorRecord[] = []
	let totalRecords = 0
	let currentPage = 1
	let pageSize = 10

	onMount(async () => {
		await fetchRecords(0, pageSize)
	})

	async function fetchRecords(offset: number, limit: number) {
		const { data, len } = await backendCanister().get_data(offset, limit, true)
		records = encodeRecords(data)
		totalRecords = len
	}

	async function handleAddRecord() {
		await backendCanister().post_data([decodeSensorData(DUMMY_IN)])
		await fetchRecords(currentPage * pageSize, pageSize)
	}

	async function handlePageChange(page: number) {
		currentPage = page
		await fetchRecords((currentPage - 1) * pageSize, pageSize)
	}

	async function handlePageSizeChange(size: number) {
		pageSize = size
		await fetchRecords((currentPage - 1) * pageSize, pageSize)
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
>
	<Toolbar>
		<ToolbarContent>
			<Button on:click={handleAddRecord}>Add record</Button>
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
