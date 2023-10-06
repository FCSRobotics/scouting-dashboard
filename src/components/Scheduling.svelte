<script lang="ts">
	import { invoke } from "@tauri-apps/api/tauri";

	let requested = false;

	const tournamentLevels = ["Practice", "Qualification", "Playoff"];
	const finals_levels = ["ef", "qf", "sf", "f"];
	let scouters = [];
	let scouters_amount = "6";
	let num_scouters = 6;
	$: scouters = [...Array(num_scouters).keys()];
	let tournamentLevel;
	let code;

	/* ex:
        [
            {
                "match_number": 0,
                "team_assignments": {
                    "9094": 0,
                    "1218": 1,
                    "314": 2,
                    "103": 3,
                    "5310": 4,
                    "4575": 5
                }
            }
        ]
    */
	let schedule = [];

	function set_scouter(match, team, value) {
		schedule.filter(
			(it) => it.match_number == match.match_number
		)[0].team_assignments[team] = value;
	}
	function get_scouter(match, team) {
		return schedule.filter((it) => it.match_number == match.match_number)[0]
			.team_assignments[team];
	}

	async function request_schedule(level, eventCode) {
		///2xQWaDZKpkWUUO0vD4WQOUv6vUfSF2Thv5HJHVziMArCgeJLlbxs1G70Ou3mJ97J
		let response = JSON.parse(
			await (
				await fetch(
					`https://www.thebluealliance.com/api/v3/event/${eventCode}/matches/simple`,
					{
						headers: {
							"X-TBA-Auth-Key":
								"2xQWaDZKpkWUUO0vD4WQOUv6vUfSF2Thv5HJHVziMArCgeJLlbxs1G70Ou3mJ97J",
						},
					}
				)
			).text()
		);

		set_schedule(response);

		return response;
	}

	function set_schedule(matches) {
		for (let i = 0; i < matches.length; i++) {
			let match = matches[i];
			if (!finals_levels.includes(match.comp_level)) {
				schedule.push(convert_match(match));
			}
		}
		schedule["scouters"] = num_scouters;
		console.log(schedule);
	}

	function convert_match(match) {
		let obj = {};
		let team_assignments = {};
		console.log(JSON.stringify(match.match_number));
		obj["match_number"] = match.match_number;
		let teams = [
			...match.alliances.blue.team_keys,
			...match.alliances.red.team_keys,
		];
		for (let i = 0; i < teams.length; i++) {
			let team = teams[i];
			team_assignments[extract_number(team)] = scouters.at(i % num_scouters);
		}
		obj["team_assignments"] = team_assignments;
		return obj;
	}

	function extract_number(team: string) {
		return team.slice(3, team.length);
	}
</script>

{#if requested}
	{#await request_schedule(tournamentLevel, code)}
		<p>Loading..</p>
	{:then response}
		{#each response as match}
			<!-- [Octo finals, Quarter finals, Semi finals, finals] -->
			{#if !finals_levels.includes(match.comp_level)}
				<div class="game-teams">
					{#each match.alliances.blue.team_keys as team}
						<p>{extract_number(team)} -&gt;</p>
						<select
							on:change={(e) => {
								set_scouter(match, extract_number(team), e.target.value);
							}}
							value={get_scouter(match, extract_number(team))}
						>
							{#each scouters as scouter}
								<option value={scouter}>
									{scouter + 1}
								</option>
							{/each}
						</select>
					{/each}
					{#each match.alliances.red.team_keys as team}
						<p>{extract_number(team)}-&gt;</p>
						<select
							on:change={(e) => {
								set_scouter(match, extract_number(team), e.target.value);
							}}
							value={get_scouter(match, extract_number(team))}
						>
							{#each scouters as scouter}
								<option value={scouter}>
									{scouter + 1}
								</option>
							{/each}
						</select>
					{/each}
				</div>
			{/if}
		{/each}
		<button
			on:click={async () => {
				schedule["scouters"] = num_scouters;
				console.log(schedule["scouters"]);
				const response = await invoke("paste_data", {
					body: JSON.stringify({
						scouters: num_scouters,
						schedule,
					}) /*JSON.stringify({
						api_dev_key: "sa0MdnUQgTRVzfgoQzhi-1XMqxI6b2-_",
						api_paste_code: ,
						api_option: "paste",
					}),*/,
				});
				console.log(schedule["scouters"]);

				// parses JSON response into native JavaScript objects
			}}>Export</button
		>
	{/await}
{:else}
	<label for="scouters">Enter number of scouters:</label>
	<input
		type="number"
		name="scouters"
		bind:value={num_scouters}
		id="scouters"
	/>
	<label for="code">Enter event code:</label>
	<input type="text" name="event code" bind:value={code} id="code" />
	<button on:click={() => (requested = true)}>Request</button>
{/if}

<style>
	.game-teams {
		width: 100%;
		display: flex;
	}
	.game-teams > * {
		width: 100%;
		margin: auto;
		text-align: center;
	}
</style>
