export type Team = {
	name: string;
	number: string;
};

export type ProcessedMatch = {
	overall_rank: number;
	auto_rank: number;
	match_number: number;
	auto_pieces: number;
	teleop_pieces: number;
};

export type TeamOverview = {
	wins: number;
	losses: number;
	lifetime_auto_grid: number;
	lifetime_auto_balance: number;
	lifetime_mobility: number;
	lifetime_teleop_grid: number;
	lifetime_balance: number;
	lifetime_parked: number;
	lifetime_auto_pieces: number;
	lifetime_teleop_pieces: number;
	lifetime_auto_rank: number;
	lifetime_balance_sucesses: number;
	lifetime_balance_attempts: number;
	lifetime_overall_rank: number;
	lifetime_cycle_len: number;
	average_cycle: number;
	average_auto: number;
	average_teleop: number;
	balance_skill: number;
	average_auto_rank: number;
	average_teleop_peices: number;
	average_auto_peices: number;
	average_rank: number;
	average_score: number;
	team: Team;
	processed_matches: ProcessedMatch[];
};

export type TeamGameData = {
	high_cones_auto: number;
	mid_cones_auto: number;
	low_cones_auto: number;
	high_cubes_auto: number;
	mid_cubes_auto: number;
	low_cubes_auto: number;
	mobility_auto: boolean;
	balanced_auto: 1 | 0 | 2;
	high_cones: number;
	mid_cones: number;
	low_cones: number;
	high_cubes: number;
	mid_cubes: number;
	low_cubes: number;
	defense: boolean;
	balanced: 1 | 0 | 2;
	parked: boolean;
	catastrophic_failure: boolean;
	sabotage: boolean;
	drive_grade: number;
	overall_grade: number;
	notes: string;
	team: Team;
	match_number: number;
	done: boolean;
	won: boolean;
};
