<script lang="ts">
	import type { Team, TeamOverview } from "./types";
	let options = {};
	export let overview: TeamOverview;
	$: options = {
		chart: {
			type: "bar",
		},
		fill: {
			colors: ["#3d6520"],
		},
		series: [
			{
				name: "Scored",
				data: [overview.lifetime_cones, overview.lifetime_cubes],
			},
		],
		xaxis: {
			categories: ["Cones", "Cubes"],
		},
	};
	import { chart } from "svelte-apexcharts";
</script>

<h1>{overview.team.number}</h1>
<label for="wl">Recorded: Wins / Losses</label>
<h2 id="wl">
	{overview.wins} / {overview.losses} ({overview.wins / overview.losses})
</h2>
<p>{overview.average_pieces}</p>
<div use:chart={options} />
