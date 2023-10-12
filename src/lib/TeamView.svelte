<script lang="ts">
	import type { Team, TeamGameData, TeamOverview } from "./types";
	let options = {};
	let rating_over_time = {};
	let score_over_time = {};
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
		rating_over_time = {
			series: [
				{
					name: "Auto",
					data: overview.processed_matches.map((game) => game.auto_rank),
				},
				{
					name: "Overall",
					data: overview.processed_matches.map((game) => game.overall_rank),
				},
			],

			chart: {
				height: 350,
				type: "line",
				zoom: {
					enabled: false,
				},
			},
			dataLabels: {
				enabled: false,
			},
			stroke: {
				curve: "straight",
			},
			title: {
				text: "Rating over time",
				align: "left",
			},
			grid: {
				row: {
					colors: ["#f3f3f3", "transparent"], // takes an array which will be repeated on columns
					opacity: 0.5,
				},
			},
			xaxis: {
				categories: overview.processed_matches.map((game) => game.match_number),
			},
		};
		score_over_time = {
			series: [
				{
					name: "Auto",
					data: overview.processed_matches.map((game) => game.auto_score),
				},
				{
					name: "Teleop",
					data: overview.processed_matches.map((game) => game.teleop_score),
				},
			],

			chart: {
				height: 350,
				type: "line",
				zoom: {
					enabled: false,
				},
			},
			dataLabels: {
				enabled: false,
			},
			stroke: {
				curve: "straight",
			},
			title: {
				text: "Score over time",
				align: "left",
			},
			grid: {
				row: {
					colors: ["#f3f3f3", "transparent"], // takes an array which will be repeated on columns
					opacity: 0.5,
				},
			},
			xaxis: {
				categories: overview.processed_matches.map((game) => game.match_number),
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
<div id="charts">
	<div use:chart={options} />
	<div use:chart={rating_over_time} />
	<div use:chart={score_over_time} />
	{#each Object.keys(overview).filter((key) => key != "processed_matches" && key != "team").map((key) => `${key}-${overview[key]}`) as text}
		<p>{text}</p>
	{/each}

</div>

<style>
	#charts {
		width: 100%;
		height: 100%;
		display: grid;
		grid-template-columns: 1fr 1fr 1fr;
	}
</style>
