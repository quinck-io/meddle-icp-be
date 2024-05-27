<script lang="ts">
	import { backendCanister } from "$lib/canisters"
	import { DataTable, Toolbar, ToolbarContent, Button } from "carbon-components-svelte"

	function handleAddRecord() {
		backendCanister().greet("Hello, world!").then(console.log)
	}
</script>

<DataTable
	title="Load balancers"
	headers={[
		{ key: "name", value: "Name" },
		{ key: "protocol", value: "Protocol" },
		{ key: "port", value: "Port" },
		{ key: "rule", value: "Rule" }
	]}
	rows={[
		{
			id: "a",
			name: "Load Balancer 3",
			protocol: "HTTP",
			port: 3000,
			rule: "Round robin"
		},
		{
			id: "b",
			name: "Load Balancer 1",
			protocol: "HTTP",
			port: 443,
			rule: "Round robin"
		},
		{
			id: "c",
			name: "Load Balancer 2",
			protocol: "HTTP",
			port: 80,
			rule: "DNS delegation"
		},
		{
			id: "d",
			name: "Load Balancer 6",
			protocol: "HTTP",
			port: 3000,
			rule: "Round robin"
		},
		{
			id: "e",
			name: "Load Balancer 4",
			protocol: "HTTP",
			port: 443,
			rule: "Round robin"
		},
		{
			id: "f",
			name: "Load Balancer 5",
			protocol: "HTTP",
			port: 80,
			rule: "DNS delegation"
		}
	]}
>
	<Toolbar>
		<ToolbarContent>
			<Button on:click={handleAddRecord}>Add record</Button>
		</ToolbarContent>
	</Toolbar>
</DataTable>
