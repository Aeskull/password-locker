use crate::prelude::*;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct PWEntry {
    time: SystemTime,
    name: String,
    password: String,
}

impl PWEntry {
    pub fn new() -> Self {
        let time = SystemTime::now();
        let name = inquire::Text::new("Enter the name of the entry:")
            .prompt_skippable()
            .unwrap_or_default()
            .unwrap_or_default();
        let password = Password::new("Now enter the password for the corresponding name:")
            .prompt_skippable()
            .unwrap_or_default()
            .unwrap_or_default();

        Self {
            time,
            name,
            password,
        }
    }

    pub fn from(name: String) -> Self {
        Self {
            time: SystemTime::now(),
            name,
            password: String::new(),
        }
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }

    pub fn set_name(&mut self, other: String) {
        self.name = other;
        self.update_time();
    }

    pub fn set_pass(&mut self, other: String) {
        self.password = other;
        self.update_time();
    }

    fn update_time(&mut self) {
        self.time = SystemTime::now();
    }
}

impl std::cmp::Ord for PWEntry {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

impl std::cmp::PartialOrd for PWEntry {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.time.partial_cmp(&other.time) {
            Some(core::cmp::Ordering::Equal) => {}
            Some(core::cmp::Ordering::Greater) => return Some(core::cmp::Ordering::Less),
            Some(core::cmp::Ordering::Less) => return Some(core::cmp::Ordering::Greater),
            ord => return ord
        }
        match self.name.partial_cmp(&other.name) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.password.partial_cmp(&other.password)
    }
}

impl std::fmt::Display for PWEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n{}\n{}", self.name, self.password)
    }
}