use crate::commands::{build, BuildParams};
use crate::config::Config;
use crate::rbx::Place;
use anyhow::Ok;
use std::env;

pub struct DeployParams {
    pub branch_name: Option<String>,
    pub api_key: Option<String>,
}

pub async fn deploy(params: &DeployParams) -> anyhow::Result<Option<String>> {
    let branch = match params.branch_name.clone() {
        Some(v) => v,
        None => "main".to_string(),
    };

    let api_key = match params.api_key.clone() {
        Some(v) => v,
        None => env::var("OPENCLOUD_KEY").expect("OPENCLOUD_KEY not set"),
    };

    println!("Publishing to {} universe", branch.clone());

    let config = Config::new(branch);
    let universe_id = config.get_universe_id();
    let places = config.get_places();

    let place = Place::new(&api_key, universe_id.unwrap());

    for (place_name, place_id) in places.unwrap().iter() {
        let deploy_dir = format!("deploy/{}", place_name);
        let path = build(&BuildParams {
            project_name: Some(place_name.to_string()),
            output_name: Some(deploy_dir),
        })
        .unwrap();

        place.publish(&path, place_id.as_u64().unwrap()).await;
    }
    Ok(None)
}
