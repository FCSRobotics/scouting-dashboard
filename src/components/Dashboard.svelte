<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";
	import TeamView from "../lib/TeamView.svelte";
	import Modal from "./Modal.svelte";

	import type { TeamOverview, TeamGameData } from "../lib/types";
	let teamOverviews = [] as TeamOverview[];
	let teamsData = [] as TeamGameData[][];
	$: console.log("update"); /*(async () => {
		teamOverviews = await Promise.all<TeamOverview>(
			teamsData.map<Promise<TeamOverview>>((data_group) =>
				invoke("calculate_overview", { data: data_group })
			)
		);
	})();*/
	let index = 0;
	let showModal = false;
</script>

<Modal
	bind:showModal
	onClose={(list) => {
		console.log(list);
		list.forEach((json) => {
			let candidate = teamsData.findIndex(
				(collection) =>
					collection.findIndex(
						(game) => game.team.number == json.team.number
					) != -1
			);
			if (candidate == -1) teamsData.push([json]);
			else {
				teamsData[candidate].push(json);
			}
			console.log(teamsData);
			teamsData = [...teamsData];
		});
	}}
/>
<div id="save-load">
	<button on:click={() => invoke("save", { data: JSON.stringify(teamsData) })}
		>Save</button
	><button
		on:click={async () => {
			teamsData = JSON.parse(await invoke("load"));
		}}>Load</button
	>
</div>
<main class="container">
	<div id="team-list">
		{#each teamsData as _, i}
			<div class="team-display">
				<button
					class="team-button"
					on:click={() => {
						console.log("got here");
						index = i;
					}}>{teamsData[i][0].team.number}</button
				>
			</div>
		{/each}
		<div class="team-display">
			<button
				class="team-button"
				on:click={() => {
					showModal = true;
				}}>+</button
			>
		</div>
		{#if teamsData.length != 0}
			<div class="team-display" id="best-button">
				<button
					class="team-button"
					on:click={async () => {
						let overviews = await Promise.all(
							teamsData.map(
								async (it) => await invoke("calculate_overview", { data: it })
							)
						);
						overviews.sort((a, b) =>
							a.lifetime_overall_rank < b.lifetime_overall_rank ? 1 : -1
						);
						console.log(overviews);
						index = teamsData.findIndex(
							(it) => it[0].team.number == overviews[0].team.number
						);
						console.log(index);
					}}>Best team</button
				>
			</div>
		{/if}
	</div>
	<div id="team-view">
		{#if teamsData.length != 0}
			{#await invoke("calculate_overview", { data: teamsData[index] })}
				<p>loading overview</p>
			{:then overview}
				<TeamView raw_data={teamsData[index]} {overview} />
				<!-- <p>{JSON.stringify(overview)}</p> -->
			{:catch error}
				<p>{error}</p>
			{/await}
		{:else}
			<h1>Go ahead, scan some data, we know you want to!</h1>
		{/if}
	</div>
</main>

<style>
	main {
		height: 100%;
		display: grid;
		grid-template-columns: 1fr 8fr;
		grid-template-rows: 1f;
	}
	#team-list {
		background-color: rgb(216, 216, 216);
		text-align: center;
		height: 100%;
	}
	#team-view {
		background-color: hsl(100, 60%, 98%);
		padding: 2rem;
		display: flex;
		flex-direction: column;
		align-items: center;
	}
	.team-display {
		margin: 0px;
		padding: 0px;
		height: 2rem;
	}
	.team-button {
		width: 100%;
		height: 100%;
		margin: 0px;
		background-color: #3d6520;
		border: none;
		color: white;
		cursor: pointer;
	}

	#save-load {
		display: flex;
		width: 100%;
		height: 5vh;
		flex-direction: row;
	}
	#save-load > button {
		width: 100%;
		height: 100%;
	}
</style>
