use std::borrow::Borrow;
use std::fs::File;
use std::io::Write;
use tokio::fs;
use serde::{Deserialize, Serialize};
use crate::status;

#[derive(Deserialize, Serialize)]
pub struct ServerConfig {
    pub(crate) status_response: status::Response,
    pub(crate) kick_message: status::TextComponent
}

impl ServerConfig {
    pub fn get_status_response(&self) -> &status::Response {
        self.status_response.borrow()
    }

    pub fn get_kick_message(&self) -> &status::TextComponent {
        self.kick_message.borrow()
    }
}

pub struct ConfigManager {
    config: Box<ServerConfig>,
}

impl ConfigManager {
    pub async fn new() -> Self {
        let mut c = ConfigManager {
            config: Box::new(ServerConfig {
                status_response: status::Response {
                    version: status::VersionInfo {
                        name: "".to_string(),
                        protocol: 0
                    },
                    players: status::Players {
                        max: 0,
                        online: 0,
                        sample: vec![]
                    },
                    favicon: "".to_string(),
                    description: status::TextComponent {
                        text: "".to_string(),
                        color: "".to_string(),
                        underlined: false,
                        bold: false,
                        italic: false,
                        strikethrough: false
                    }
                },
                kick_message: status::TextComponent {
                    text: "".to_string(),
                    color: "".to_string(),
                    underlined: false,
                    bold: false,
                    italic: false,
                    strikethrough: false
                }
            })
        };
        c.load().await;
        c
    }

    pub fn get_config(&self) -> &ServerConfig {
        self.config.as_ref()
    }

    pub async fn load(&mut self) {
        match File::open("./config.json") {
            Ok(_) => {}
            Err(_) => {
                let mut f = File::create("./config.json").unwrap();
                f.write_all(serde_json::to_string_pretty(self.config.as_ref()).unwrap().as_bytes()).unwrap();
            }
        }
        match fs::read_to_string("./config.json").await {
            Ok(f) => {
                self.config = Box::new(serde_json::from_str(f.as_str()).unwrap());
            }
            Err(e) => {
                panic!("Error loading config {}", e.to_string());
            }
        }
    }
}