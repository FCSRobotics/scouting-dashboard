<script lang="ts">
	import type { Team, TeamOverview } from "./types";
	let options = {};
	let radar_averages = {};
	export let overview: TeamOverview;
	console.log(overview);
	$: (() => {
		options = {
			chart: {
				type: "bar",
			},
			fill: {
				colors: ["#3d6520"],
			},
			series: [
				{
					name: "Scored",
					data: [overview.average_teleop_peices, overview.average_auto_peices],
				},
			],
			xaxis: {
				categories: ["Teleop pieces", "Auto pieces"],
			},
		};
		// radar_averages = {
		// 	series: [
		// 		{
		// 			name: "Averages",
		// 			data: [
		// 				overview.average_auto,
		// 				overview.average_auto_rank,
		// 				overview.average_teleop,
		// 				overview.average_rank,
		// 			],
		// 		},
		// 	],
		// 	chart: {
		// 		height: "500",
		// 		width: "500",
		// 		type: "radar",
		// 	},
		// 	title: {
		// 		text: "Averages",
		// 	},
		// 	xaxis: {
		// 		categories: [
		// 			"Average auto",
		// 			"Average auto rating",
		// 			"Average teleop",
		// 			"Average teleop rating",
		// 		],
		// 	},
		// };
	})();
	import { chart } from "svelte-apexcharts";
</script>

<h1>{overview.team.number}</h1>
<label for="wl">Recorded: Wins / Losses</label>
<h2 id="wl">
	{overview.wins} / {overview.losses} ({overview.wins / overview.losses})
</h2>
<p>Overall rank: {overview.lifetime_overall_rank}</p>
<div use:chart={options} />
<!-- <div use:chart={radar_averages} /> -->
