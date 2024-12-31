use actix_cors::Cors;
use actix_web::{middleware::Logger, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::debug;
use mpris::PlayerFinder;
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub mod config;

#[derive(Debug, Serialize, Deserialize)]
pub struct Metadata {
    pub identity: String,
    pub title: String,
    pub artists: Vec<String>,
    pub tracks: Vec<String>,
    pub album: String,
    pub length: u64,
    pub position: u64,
}

// Pauses currently playing media and prints metadata information about that
// media.
// If no player is running, exits with an error.
fn player(identity: String) {
    if let Ok(finder) = PlayerFinder::new() {
        if let Ok(players) = finder.find_all() {
            for player in players {
                let running = player.is_running();
                if running && player.identity() == identity {
                    debug!("Pausing player: {}", player.identity());
                    if let Ok(_) = player.play_pause() {
                        debug!("Player paused");
                    } else {
                        debug!("Could not pause player");
                    }
                    let metadata = player
                        .get_metadata()
                        .expect("Could not get metadata for player");
                    debug!("{:#?}", metadata);
                } else {
                    debug!("No player is running");
                }
            }
        }
    };
}

// Get the metadata of the currently playing media to return to the client.
fn listplayers() -> Vec<Metadata> {
    let finder = PlayerFinder::new().unwrap();
    let players = finder.find_all().unwrap();
    let mut metadatas = Vec::new();
    for player in players {
        let mut tracks = Vec::new();

        if player.get_track_list().is_ok() {
            let tracks_ = player.get_track_list().unwrap();
            for track in tracks_.ids() {
                tracks.push(track.to_string());
            }
        } else {
            debug!("Could not get track list");
        }

        if player.is_running() {
            let metadata = player.get_metadata();
            if let Ok(metadata) = metadata {
                let title = metadata.title().unwrap_or("").to_string();
                let identity = player.identity().to_string();
                let artists: Vec<String> = metadata
                    .artists()
                    .unwrap_or(vec![])
                    .into_iter()
                    .map(|s| s.to_string())
                    .collect();
                let album = metadata.album_name().unwrap_or("").to_string();
                let length = metadata
                    .length()
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                let position = player
                    .get_position()
                    .unwrap_or(Duration::from_secs(0))
                    .as_secs();
                metadatas.push(Metadata {
                    identity,
                    title,
                    tracks,
                    artists,
                    album,
                    length,
                    position,
                });
            }
        }
    }
    metadatas
}

#[actix_web::main]
async fn main() {
    let port = config::config_env();
    let _ = HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .wrap(Logger::default())
            .service(root)
            .service(get_players_list)
            .service(pause_player)
    })
    .bind(format!("0.0.0.0:{}", port))
    .expect("Unable to start the server")
    .run()
    .await;
}

#[actix_web::get("/")]
pub async fn root(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello worldsss!")
}

#[actix_web::get("/listplayers")]
pub async fn get_players_list(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().json(listplayers())
}

#[actix_web::get("/pause/{identity}")]
pub async fn pause_player(req: HttpRequest) -> impl Responder {
    let identity = req.match_info().get("identity").unwrap();
    player(identity.to_string());
    HttpResponse::Ok().body(format!("Pausing player: {}", identity))
}
