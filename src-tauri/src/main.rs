// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{File, self};
use std::io::Write;
use std::sync::{Mutex, MutexGuard};
use std::collections::HashMap;
use std::error::Error;
use std::time::SystemTime;
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


#[derive(Clone,serde::Deserialize, serde::Serialize, Default)]
struct ProcessedMatch {

    overall_rank: isize,
    auto_rank: isize,
    match_number: usize,
    auto_pieces: usize,
    teleop_pieces: usize,
    teleop_score: usize,
    auto_score: usize,
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
    lifetime_auto_rank: isize,
    lifetime_balance_sucesses: usize,
    lifetime_balance_attempts: usize,
    lifetime_overall_rank: isize,
    lifetime_cycle_len: f64,
    average_cycle: f64,
    average_auto: f64,
    average_teleop: f64,
    balance_skill: f64,
    average_auto_rank: f64,
    average_teleop_peices: f64,
    average_auto_peices: f64,
    average_rank: f64,
    average_score: f64,
    team: Team,
    processed_matches: Vec<ProcessedMatch>
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
fn save(data: String)  {
    let mut file  = File::create("./save.quaker").expect("Error");
    file.write_all(&data.into_bytes()).expect("Error");

}

#[tauri::command]
fn load() -> String  {
    fs::read_to_string("./save.quaker").expect("Error")

}

#[tauri::command]
fn calculate_overview(data: Vec<TeamGameData>) -> TeamOverview {
    let mut overview = TeamOverview::default();
    overview.team = data[0].team.clone();
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
        overview.lifetime_auto_rank += (auto_grid+auto_balance_points) as isize;
        if game.balanced_auto==1 {overview.lifetime_auto_rank +=5;} else if (game.balanced_auto==3) {overview.lifetime_auto_rank -=3;} else if (game.balanced_auto==0) {overview.lifetime_auto_rank -=5}
        if game.mobility_auto {overview.lifetime_auto_rank+=2}
        if game.balanced_auto==1 || game.balanced_auto==3 || game.balanced_auto==0 {overview.lifetime_balance_attempts+=1}
        if game.balanced_auto==1 {overview.lifetime_balance_sucesses+=1}
        let tele_pieces = game.high_cones+game.high_cubes + game.mid_cones+game.mid_cubes + game.low_cones+game.low_cubes;
        overview.lifetime_teleop_pieces += tele_pieces;
        if game.balanced==1 || game.balanced==3 || game.balanced==0 {overview.lifetime_cycle_len += 115.0/(tele_pieces as f64);} else {overview.lifetime_cycle_len += 135.0/(tele_pieces as f64)}
        let mut overall_rank: isize = (auto_grid+auto_balance_points+teleop_grid+balance) as isize;
        if game.balanced_auto==1 {overall_rank +=5;} else if game.balanced_auto==3 {overall_rank -=3;} else if game.balanced_auto==0 {overall_rank -=5}
        if game.mobility_auto {overall_rank+=2}
        if game.balanced==1 {overall_rank +=2;} else if game.balanced==3 {overall_rank -=5;} else if game.balanced==0 {overall_rank -=10}
        if game.parked {overall_rank +=2}
        if game.sabotage {overall_rank -=10}
        if game.catastrophic_failure {overall_rank -=20}
        if game.won {overall_rank +=10}
        overall_rank += game.drive_grade as isize;//driving_score;
        overall_rank += game.overall_grade as isize;
        if game.defense {overall_rank +=5}
        overall_rank += (tele_pieces*3) as isize;
        overview.lifetime_overall_rank += overall_rank as isize;

        let processed_match = ProcessedMatch {
            auto_pieces: game.high_cones_auto + game.high_cubes_auto + game.mid_cones_auto + game.mid_cubes_auto + game.low_cones_auto + game.low_cubes_auto,
            teleop_pieces: game.high_cones + game.high_cubes + game.mid_cones + game.mid_cubes + game.low_cones + game.low_cubes,
            overall_rank: overall_rank,
            auto_rank: (auto_grid+auto_balance_points) as isize,
            match_number: game.match_number,
            teleop_score: teleop_grid + balance,
            auto_score: auto_grid + auto_balance_points,
        };
        overview.processed_matches.push(processed_match);
    }

    overview.average_cycle=overview.lifetime_cycle_len/(data.len() as f64);
    overview.average_auto=((overview.lifetime_auto_grid+overview.lifetime_auto_balance) as f64) / (data.len() as f64);
    overview.average_teleop=
    ((overview.lifetime_teleop_grid+overview.lifetime_balance+overview.lifetime_parked) as f64)/ (data.len() as f64);
    overview.balance_skill=(overview.lifetime_balance_sucesses as f64)/(overview.lifetime_balance_attempts as f64);
    overview.average_auto_rank=(overview.lifetime_auto_rank as f64)/(data.len() as f64);
    overview.average_auto_peices = (overview.lifetime_auto_pieces as f64)/(data.len() as f64);
    overview.average_teleop_peices = (overview.lifetime_teleop_pieces as f64)/(data.len() as f64);
    overview.average_rank=(overview.lifetime_overall_rank as f64)/(data.len() as f64);
    overview.average_score=overview.average_teleop+overview.average_auto;
    return overview;
}

#[tauri::command]
fn export(overviews: Vec<TeamOverview>)  {
    let file = File::create(format!("./{}.csv",  SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())).expect("Unable to create file");
    let mut wtr = csv::Writer::from_writer(file);
    wtr.write_record(&["wins",
        "losses",
        "lifetime_auto_grid",
        "lifetime_auto_balance",
        "lifetime_mobility",
        "lifetime_teleop_grid",
        "lifetime_balance",
        "lifetime_parked",
        "lifetime_auto_pieces",
        "lifetime_teleop_pieces",
        "lifetime_auto_rank",
        "lifetime_balance_sucesses",
        "lifetime_balance_attempts",
        "lifetime_overall_rank",
        "lifetime_cycle_len",
        "average_cycle",
        "average_auto",
        "average_teleop",
        "balance_skill",
        "average_auto_rank",
        "average_teleop_peices",
        "average_auto_peices",
        "average_rank",
        "average_score",
        "team",
        "processed_matches"]).expect("failed to write csv");
    overviews.iter().for_each(|overview| {wtr.serialize(overview).expect("Failed to write overview");});//wtr.serialize(overviews);
    wtr.flush().unwrap();
}


fn main() {
    tauri::Builder::default()
        .manage(AppState::default())
        .invoke_handler(tauri::generate_handler![greet, get_index, set_index, calculate_overview, paste_data, save, load, export])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
