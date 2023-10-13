<script lang="ts">
    import { defaultTeamGameData, type TeamGameData } from "../lib/types";

	export let onClose;
	export let showModal; // boolean

	let dialog; // HTMLDialogElement

	$: if (dialog && showModal) dialog.showModal();

	let json = "";


	function parse(text: string) {
		console.log(JSON.parse(text).map((game: string[]) => {
			let values = game.map((value) => JSON.parse(value))
			let temp: TeamGameData = {
                high_cones_auto: values[0],
                mid_cones_auto: values[1],
                low_cones_auto: values[2],
                high_cubes_auto: values[3],
                mid_cubes_auto: values[4],
                low_cubes_auto: values[5],
                mobility_auto: values[6],
                balanced_auto: values[7],
                high_cones: values[8],
                mid_cones: values[9],
                low_cones: values[10],
                high_cubes: values[11],
                mid_cubes: values[12],
                low_cubes: values[13],
                defense: values[14],
                balanced: values[15],
                parked: values[16],
                catastrophic_failure: values[17],
                sabotage: values[18],
                drive_grade: values[19],
                overall_grade: values[20],
                notes: values[21],
                team: values[22],
                match_number: values[23],
                done: values[24],
                won: values[25]
            }
			
			return temp;
		}))
		return JSON.parse(text).map((game: string[]) => {
			let values = game.map((value) => JSON.parse(value))
			let temp: TeamGameData = {
                high_cones_auto: values[0],
                mid_cones_auto: values[1],
                low_cones_auto: values[2],
                high_cubes_auto: values[3],
                mid_cubes_auto: values[4],
                low_cubes_auto: values[5],
                mobility_auto: values[6],
                balanced_auto: values[7],
                high_cones: values[8],
                mid_cones: values[9],
                low_cones: values[10],
                high_cubes: values[11],
                mid_cubes: values[12],
                low_cubes: values[13],
                defense: values[14],
                balanced: values[15],
                parked: values[16],
                catastrophic_failure: values[17],
                sabotage: values[18],
                drive_grade: values[19],
                overall_grade: values[20],
                notes: values[21],
                team: values[22],
                match_number: values[23],
                done: values[24],
                won: values[25]
            }
			return temp;
		})
			
		

	}
</script>

<!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions -->
<dialog bind:this={dialog} on:close={() => (showModal = false)}>
	<!-- svelte-ignore a11y-no-static-element-interactions -->
	<div on:click|stopPropagation>
		<label for="json">Paste json:</label>
		<input autofocus type="text" bind:value={json} name="json" id="json" />
		<button
			
			on:click={() => {
				dialog.close();
				onClose(parse(json));
			}}>Submit</button
		>
	</div>
</dialog>

<style>
	dialog {
		max-width: 32em;
		border-radius: 0.2em;
		border: none;
		padding: 0;
	}
	dialog::backdrop {
		background: rgba(0, 0, 0, 0.3);
	}
	dialog > div {
		padding: 1em;
	}
	dialog[open] {
		animation: zoom 0.3s cubic-bezier(0.34, 1.56, 0.64, 1);
	}
	@keyframes zoom {
		from {
			transform: scale(0.95);
		}
		to {
			transform: scale(1);
		}
	}
	dialog[open]::backdrop {
		animation: fade 0.2s ease-out;
	}
	@keyframes fade {
		from {
			opacity: 0;
		}
		to {
			opacity: 1;
		}
	}
	button {
		display: block;
	}
</style>
