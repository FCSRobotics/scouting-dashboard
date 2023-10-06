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


#[derive(Clone,serde::Deserialize, serde::Serialize)]
struct Team {

    number: String,
}


#[derive(serde::Deserialize, serde::Serialize)]
struct TeamOverview {
    wins: usize,
    losses: usize,
    lifetime_cones: usize,
    lifetime_cubes: usize,
    team: Team,
    average_pieces: f64, 
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
        overview.lifetime_cones += game.high_cones as usize;
        overview.lifetime_cubes += game.high_cubes as usize;
        
    }
    overview.average_pieces = ((overview.lifetime_cones + overview.lifetime_cubes) as f64) / (data.len() as f64);
    return overview;
}

fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet, get_index, set_index, calculate_overview, paste_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
