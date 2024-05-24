<script lang="ts">
	import { backend } from "$lib/canisters"

	let greeting = ""
	let name = ""

	function onSubmit() {
		backend.greet(name).then((response) => {
			greeting = response
		})
		return false
	}
</script>

<main>
	<img src="/logo2.svg" alt="DFINITY logo" />

	<form action="#" on:submit|preventDefault={onSubmit}>
		<label for="name">Enter your name: &nbsp;</label>
		<input id="name" alt="Name" type="text" bind:value={name} />
		<button type="submit">Click Me!</button>
	</form>
	<section id="greeting">{greeting}</section>
</main>
