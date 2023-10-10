// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;
use std::error::Error;
use clipboard::{ClipboardProvider, ClipboardContext};

use tauri::State;

#[derive(Default)]
struct AppState {
    teams: Mutex<HashMap<String, Mutex<Vec<TeamGameData>>>>,
    index: Mutex<Index>

}

impl AppState {
    // fn add_data(self: &Self, data: TeamGameData) {
    //     let lock_result = self.teams.lock();
    //     if let Ok(mut lock) = lock_result {
    //         if lock.contains_key(&data.team.number) {
    //             self.add_data_to_team(lock, data.clone(), data.team)
    //         } else {
    //             lock.insert(data.team.number.clone(), Mutex::new(vec![data]));
    //         }
    //     }
    // }

    // fn add_data_to_team(self: &Self, map_lock: MutexGuard<'_, HashMap<String, Mutex<Vec<TeamGameData>>>>, data: TeamGameData, team: Team) {
    //     let lock_result = map_lock.get(&data.team.number).expect("UH OH SPAGHETTIO").lock();
    //     if let Ok(mut lock) = lock_result {
    //         lock.push(data);
    //     }
    // }

    fn set_index(self: &Self, index: usize) {
        let lock_result = self.index.lock();
        if let Ok(mut lock) = lock_result {
            lock.0 = index;
        }
    }

    // fn get_team_data(self: &Self, team: Team) -> Option<Vec<TeamGameData>> {
    //     let first_lock = self.teams.lock().unwrap();
    //     let lock_result = first_lock.get(&team.number).expect("UH OH SPAGHETTIO").lock();
    //     if let Ok(lock) = lock_result {
    //         return Some(lock.clone())
    //     }
    //     None
    // }
    
    fn get_index(self: &Self) -> usize {
        let lock_result = self.index.lock();
        if let Ok(lock) = lock_result {
            return lock.0
        }
        0
    }

    // fn get_overview(self: &Self, team: Team) -> TeamOverview {
    //     let data = self.get_team_data(team.clone()).unwrap();
    //     let mut wins = 0;
    //     let mut losses = 0;
    //     let mut cones = 0;
    //     let mut cubes = 0;
    //     for game in data {
    //         if game.won {wins += 1;} else {losses +=1}
    //         cones += game.cones as usize;
    //         cubes += game.cubes as usize;
    //     }
    //     return TeamOverview { wins, losses, lifetime_cones: cones, lifetime_cubes: cubes, team: team, average_pieces: 0.0 }
    // }

}

#[derive(Default)]
struct Index(usize);


#[derive(Clone,serde::Deserialize, serde::Serialize, Default)]
struct Team {

    number: String,
}


#[derive(serde::Deserialize, serde::Serialize, Default)]
struct TeamOverview {
    wins: usize,
    losses: usize,
    lifetime_auto_grid: usize,
    lifetime_auto_balance: usize,
    lifetime_mobility: usize,
    lifetime_teleop_grid: usize,
    lifetime_balance: usize,
    lifetime_parked: usize,
    lifetime_auto_pieces: usize,
    lifetime_teleop_pieces: usize,
    lifetime_auto_rank: usize,
    lifetime_balance_sucesses: usize,
    lifetime_balance_attempts: usize,
    lifetime_overall_rank: usize,
    lifetime_cycle_len: f64,
    team: Team,
 
}

#[derive(Clone,serde::Deserialize, serde::Serialize)]
struct TeamGameData {
    high_cones_auto: usize,
    mid_cones_auto: usize,
    low_cones_auto: usize,
    high_cubes_auto: usize,
    mid_cubes_auto: usize,
    low_cubes_auto: usize,
    mobility_auto: bool,
    balanced_auto: usize,
    high_cones: usize,
    mid_cones: usize,
    low_cones: usize,
    high_cubes: usize,
    mid_cubes: usize,
    low_cubes: usize,
    defense: bool,

    balanced: usize,
    parked: bool,
    catastrophic_failure: bool,
    sabotage: bool,
    drive_grade: usize,
    overall_grade: usize,
    notes: String,
    team: Team,
    match_number: usize,
    done: bool,
    won: bool,
}
// earn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_index(state: State<AppState>) -> usize {
    return state.get_index();
}

#[tauri::command]
fn set_index(state: State<AppState>, value: usize) {
    return state.set_index(value);
}



#[tauri::command]
fn paste_data(body: String) -> String {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(body).unwrap();
    // let client: reqwest::blocking::Client = reqwest::blocking::Client::new();
    // println!("{}", body);
    // let resp = client.post("https://pastebin.com/api/api_post.php").body(body).send().expect("failed to paste");
    // resp.text().unwrap()
    String::from("")

}

#[tauri::command]
fn calculate_overview(data: Vec<TeamGameData>) -> TeamOverview {
    let mut overview = TeamOverview { wins: 0, losses: 0, lifetime_cones: 0, lifetime_cubes: 0, team: data[0].team.clone() , average_pieces: 0.0};
    for game in data.clone() {
        if game.won {overview.wins += 1;} else {overview.losses +=1}

        let auto_grid = 6*(game.high_cones_auto+game.high_cubes_auto) + 4*(game.mid_cones_auto+game.mid_cubes_auto) + 3*(game.low_cones_auto+game.low_cubes_auto);
        overview.lifetime_auto_grid += auto_grid;
        let mut auto_balance_points: usize = 0;
        if game.balanced_auto==1 {let auto_balance_points=12;} else if (game.balanced_auto==3) {let auto_balance_points=8;}
        overview.lifetime_auto_balance+=auto_balance_points;
        if game.mobility_auto {overview.lifetime_mobility+=1;}
        let teleop_grid=5*(game.high_cones+game.high_cubes) + 3*(game.mid_cones+game.mid_cubes) + 2*(game.low_cones+game.low_cubes);
        overview.lifetime_teleop_grid += teleop_grid;
        let mut balance: usize = 0;
        if game.balanced==1 {balance+=10} else if game.balanced==3 {balance+=6}
        overview.lifetime_balance+=balance;
        if game.parked {overview.lifetime_parked+=2;}
        let points=(auto_grid+auto_balance_points+teleop_grid+balance);
        overview.lifetime_auto_pieces += game.high_cones_auto+game.high_cubes_auto + game.mid_cones_auto+game.mid_cubes_auto + game.low_cones_auto + game.low_cubes_auto;
        overview.lifetime_auto_rank += (auto_grid+auto_balance_points);
        if game.balanced_auto==1 {overview.lifetime_auto_rank +=5;} else if (game.balanced_auto==3) {overview.lifetime_auto_rank -=3;} else if (game.balanced_auto==0) {overview.lifetime_auto_rank -=5}
        if game.mobility_auto {overview.lifetime_auto_rank+=2}
        if game.balanced_auto==1 || game.balanced_auto==3 || game.balanced_auto==0 {overview.lifetime_balance_attempts+=1}
        if game.balanced_auto==1 {overview.lifetime_balance_sucesses+=1}
        let tele_pieces = game.high_cones+game.high_cubes + game.mid_cones+game.mid_cubes + game.low_cones+game.low_cubes;
        overview.lifetime_teleop_pieces += tele_pieces;
        if game.balanced==1 || game.balanced==3 || game.balanced==0 {overview.lifetime_cycle_len += 115.0/(tele_pieces as f64);} else {overview.lifetime_cycle_len += 135.0/(tele_pieces as f64)}
        let mut overall_rank = (auto_grid+auto_balance_points+teleop_grid+balance);
        if game.balanced_auto==1 {overall_rank +=5;} else if game.balanced_auto==3 {overall_rank -=3;} else if game.balanced_auto==0 {overall_rank -=5}
        if game.mobility_auto {overall_rank+=2}
        if game.balanced==1 {overall_rank +=2;} else if game.balanced==3 {overall_rank -=5;} else if game.balanced==0 {overall_rank -=10}
        if game.parked {overall_rank +=2}
        if game.sabotage {overall_rank -=10}
        if game.catastrophic_failure {overall_rank -=20}
        if game.won {overall_rank +=10}
        overall_rank += game.drive_grade;//driving_score;
        overall_rank += game.overall_grade;
        if game.defense {overall_rank +=5}
        overall_rank += tele_pieces*3;
        overview.lifetime_overall_rank += overall_rank;
    }

    overview.average_cycle=lifetime_cycle_len/data.len();
    overview.average_auto=overview.lifetime_auto_grid+overview.lifetime_auto_balance/ data.len();
    overview.average_teleop=lifetime_teleop_grid+lifetime_teleop_balance+lifetime_parked/ data.len();
    overview.balance_skill=lifetime_balance_sucesses/lifetime_balance_attempts;
    overview.average_auto_rank=lifetime_auto_rank/data.len();
    overview.average_teleop_peices = lifetime_teleop_peices/data.len();
    average_rank=lifetime_overall_rank/data.len();
    average_score=average_teleop+avearage_auto;
    return overview;
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet, get_index, set_index, calculate_overview, paste_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
