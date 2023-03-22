use super::getenv;
use crate::commands::{build, BuildParams};
use crate::config::Config;
use crate::rbx::{Message, Place};
use anyhow::Ok;

pub struct DeployParams {
    pub branch_name: Option<String>,
    pub message: Option<String>,
    pub api_key: Option<String>,
}

pub async fn deploy(params: &DeployParams) -> anyhow::Result<Option<String>> {
    let api_key = getenv(params.api_key.clone(), "OPENCLOUD_KEY".to_string());
    let branch = match params.branch_name.clone() {
        Some(v) => v,
        None => "main".to_string(),
    };

    println!("Publishing to {} universe", branch.clone());

    let config = Config::new(branch.clone());
    let universe_id = config.get_universe_id().unwrap();
    let places = config.get_places();

    let place = Place::new(&api_key, universe_id);

    for (place_name, place_id) in places.unwrap().iter() {
        let deploy_dir = format!("deploy/{}", place_name);
        let path = build(&BuildParams {
            project_name: Some(place_name.to_string()),
            output_name: Some(deploy_dir),
        })
        .unwrap();

        place.publish(&path, place_id.as_u64().unwrap()).await;
    }

    if !params.message.is_none() {
        let topic = format!("updates-{}", branch);
        Message::new(&api_key, universe_id)
            .publish(&topic, &params.message.clone().unwrap())
            .await;
    }
    Ok(None)
}
