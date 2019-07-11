use crate::hydra::{JobsetConfig, ProjectConfig};
use reqwest::header::REFERER;
use reqwest::Error;
use std::collections::HashMap;
use std::fs::read_to_string;

fn load_config(config_path: &str) -> JobsetConfig {
    let cfg = read_to_string(config_path).expect("Failed to read config file");
    serde_json::from_str(&cfg).expect("Failed to parse jobset configuration")
}

fn login(client: &reqwest::Client, host: &str, user: &str, password: &str) -> Result<(), Error> {
    let login_request_url = format!("{host}/login", host = host);
    let creds: HashMap<&str, &str> = [("username", user), ("password", password)]
        .iter()
        .cloned()
        .collect();
    client
        .post(&login_request_url)
        .header(REFERER, host)
        .json(&creds)
        .send()?;
    Ok(())
}

fn create_project(client: &reqwest::Client, host: &str, project: &str) -> Result<(), Error> {
    let create_proj_url = format!("{host}/project/{project}", host = host, project = project);
    let proj: ProjectConfig = ProjectConfig {
        displayname: String::from(project),
        enabled: true,
        visible: true,
    };
    client
        .put(&create_proj_url)
        .header(REFERER, host)
        .json(&proj)
        .send()?;
    Ok(())
}

fn create_jobset(
    client: &reqwest::Client,
    host: &str,
    jobset_config: &JobsetConfig,
    project: &str,
    jobset: &str,
) -> Result<(), Error> {
    let jobset_request_url = format!(
        "{host}/jobset/{project}/{jobset}",
        host = host,
        project = project,
        jobset = jobset
    );
    client
        .put(&jobset_request_url)
        .header(REFERER, host)
        .json(&jobset_config)
        .send()?;
    Ok(())
}

pub fn run(
    host: &str,
    config_path: &str,
    project_name: &str,
    jobset_name: &str,
    user: &str,
    password: &str,
) -> Result<(), Error> {
    println!(
        "Creating jobset '{}' in project '{}'",
        jobset_name, project_name
    );
    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let cfg = load_config(config_path);

    login(&client, host, user, password)?;
    create_project(&client, host, project_name)?;
    create_jobset(&client, host, &cfg, project_name, jobset_name)?;

    Ok(())
}
