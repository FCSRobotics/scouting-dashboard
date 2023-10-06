<script lang="ts">
	import Dashboard from "./components/Dashboard.svelte";
	import Scheduling from "./components/Scheduling.svelte";

	let pageindex = -1;
</script>

<nav>
	{#if pageindex != -1}<button on:click={() => (pageindex = -1)}>Home</button
		>{/if}
</nav>
{#if pageindex == -1}
	<button on:click={() => (pageindex = 0)}>Dashboard</button><button
		on:click={() => (pageindex = 1)}>Scheduling</button
	>
{:else if pageindex == 0}
	<Dashboard />
{:else if pageindex == 1}
	<Scheduling />
{/if}

<style>
	nav {
		padding: 0px;
		margin: 0px;
		background-color: #cae8b5;
		display: flex;
	}
	nav > button {
		height: 100%;
		background-color: #5f9f32;
		border: none;
		cursor: pointer;
	}
</style>
