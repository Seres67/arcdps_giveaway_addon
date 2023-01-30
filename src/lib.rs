mod plugin;

use std::error::Error;
use std::sync::{Mutex, MutexGuard};
use arcdps::{extras::{ExtrasAddonInfo}, imgui};
use arcdps::callbacks::ArcDpsExport;
use arcdps::extras::{UserInfo, UserInfoIter, UserRole};
use arcdps::imgui::Ui;
use log::info;
use crate::plugin::Plugin;
use once_cell::sync::Lazy;
use rand::Rng;

static PLUGIN: Lazy<Mutex<Plugin>> = Lazy::new(|| Mutex::new(Plugin::new()));

arcdps::export! {
    name: "Giveaway Addon",
    sig: 0x67e78e3d,
    init,
    extras_init,
    extras_squad_update,
    imgui
}

fn init() -> Result<(), Box<dyn Error>> {
    Ok(())
}

fn extras_init(extras_info: ExtrasAddonInfo, account_name: Option<&str>) {
    info!(target: "file",
        "extras version {} on account {}",
        extras_info.string_version.unwrap_or("unknown"),
        account_name.unwrap_or("unknown")
    );
    PLUGIN.lock().unwrap().player = account_name.unwrap().to_string();
}

fn extras_squad_update(users: UserInfoIter)
{
    let mut plugin = PLUGIN.lock().unwrap();
    for user in users {
        if let UserInfo {
            account_name: Some(name),
            role: UserRole::SquadLeader | UserRole::Lieutenant | UserRole::Member,
            subgroup: _subgroup,
            join_time: _join_time,
            ready_status: _ready_status,
        } = user
        {
            add_player_to_squad(&mut plugin, name);
            // if user.account_name.unwrap() == plugin.player {
            //     continue;
            // }
            // plugin.squad.push(user.account_name.unwrap().to_string());
        } else if let UserInfo {
            account_name: Some(name),
            role: UserRole::None,
            subgroup: _subgroup,
            join_time: _join_time,
            ready_status: _ready_status,
        } = user
        {
            remove_player_from_squad(&mut plugin, name);
            // if name == plugin.player {
            //     plugin.squad.clear();
            // } else if plugin.squad.contains(&name.to_string()) {
            //     let index = plugin.squad.iter().position(|x| *x == name).unwrap_or(50);
            //     if index == 50 {
            //         continue;
            //     }
            //     plugin.squad.remove(index);
            // }
        }
    }
}

fn add_player_to_squad(plugin: &mut MutexGuard<Plugin>, name: &str)
{
    if plugin.squad.contains(&name.to_string()) || name == plugin.player {
        return;
    }
    plugin.squad.push(name.to_string());
}

fn remove_player_from_squad(plugin: &mut MutexGuard<Plugin>, name: &str)
{
    if name == plugin.player {
        plugin.squad.clear();
        plugin.winner = String::from("No winner yet!");
        return;
    }
    if !plugin.squad.contains(&name.to_string()) {
        return;
    }
    let index = plugin.squad.iter().position(|x| *x == name).unwrap_or(50);
    if index == 50 {
        return;
    }
    plugin.squad.remove(index);
}

fn imgui(ui: &Ui, _not_loading_or_character_selection: bool)
{
    let mut plugin = PLUGIN.lock().unwrap();
    imgui::Window::new("Giveaway##giveaway-window").build(ui, || {
        ui.text(String::from("Number of players in squad:") + plugin.squad.len().to_string().as_str());
        ui.text(String::from("Picked player: ") + plugin.winner.clone().as_str());
        if ui.button("Pick player") {
            pick_random_player(&mut plugin);
        }
    });
}

fn pick_random_player(plugin: &mut MutexGuard<Plugin>)
{
    if plugin.squad.is_empty() {
        return;
    }
    let picked_index = rand::thread_rng().gen_range(0..plugin.squad.len());
    plugin.winner = plugin.squad[picked_index].to_owned();
}