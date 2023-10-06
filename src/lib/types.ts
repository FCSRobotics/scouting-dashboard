export type Team = {
	name: string;
	number: string;
};

export type TeamOverview = {
	wins: number;
	losses: number;
	lifetime_cones: number;
	lifetime_cubes: number;
	team: Team;
	average_pieces: number;
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
