extern crate preferences;

use self::preferences::{AppInfo, PreferencesMap, Preferences};

use std::fs::{create_dir_all, File};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
const APP_INFO: AppInfo = AppInfo { name: env!("CARGO_PKG_NAME"), author: env!("CARGO_PKG_AUTHORS") };
const PREF_KEY: &str = "config";

lazy_static! {
    static ref CONFIG: PreferencesMap = setup();
}

fn setup() -> PreferencesMap {
    let mut path = preferences::prefs_base_dir().expect("No base dir for config files")
        .join(APP_INFO.name)
        .join(PREF_KEY);
    path.set_extension("prefs.json");

    if (!path.exists()) {
        match path.parent() {
            Some(parent) => { create_dir_all(parent).expect("Could not create config file parent directories") }
            None => {},
        };

        let mut pref = PreferencesMap::<String>::new();
        set_to_default(&mut pref);
        pref.save(&APP_INFO, PREF_KEY).expect("Could not save config file");

        info!("Configuration file created: {}", path.into_os_string().into_string().unwrap());
    }

    PreferencesMap::<String>::load(&APP_INFO, PREF_KEY).expect("Could not load config file")
}
pub enum ConfigItem {
    DatabaseUser,
    DatabasePassword,
    Custom(String),
}

impl ConfigItem {
    fn as_key(self) -> String {
        match self {
            ConfigItem::DatabaseUser => String::from("db_user"),
            ConfigItem::DatabasePassword => String::from("db_password"),
            ConfigItem::Custom(s) => s,
        }
    }
}

/// Return an entry from the configuration file
pub fn get(item: ConfigItem) -> Option<String> {
    match CONFIG.get(&item.as_key()) {
        None => None,
        Some(s) => Some(s.clone()),
    }
}

fn set_to_default(pref: &mut PreferencesMap) {
    pref.insert(ConfigItem::DatabaseUser.as_key(), String::from("root"));
    pref.insert(ConfigItem::DatabasePassword.as_key(), String::from("pw"));
}