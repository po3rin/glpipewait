extern crate exitcode;
use std::{thread, time};
extern crate surf;
use clap::{App, Arg};
use url::Url;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize)]
struct Res {
    detailed_status: Status,
}

#[derive(Serialize, Deserialize)]
struct Status {
    text: String,
}

async fn get_pipe_status(pipe_url: &str, token: &str) -> Result<Res, surf::Error> {
    let parse_url = Url::parse(pipe_url)?;

    let host = parse_url.host_str().unwrap().to_string();
    let mut path = parse_url
        .path_segments()
        .map(|c| {
            c.filter(|&i| i != "-" && i != "pipelines")
                .collect::<Vec<_>>()
        })
        .unwrap();
    let pipeline_id = path.pop().unwrap();
    let path_joined = path.join("%2F");

    let Res { detailed_status } = surf::get(format!(
        "https://{}/api/v4/projects/{}/pipelines/{}",
        host, path_joined, pipeline_id
    ))
    .header("PRIVATE-TOKEN", token)
    .recv_json()
    .await?;
    Ok(Res { detailed_status })
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let app = App::new("glpipewait")
        .version("0.1.0")
        .author("hiromu.nakamura <abctail30@gmail.com>")
        .about("wait gitlab pipeline")
        .arg(
            Arg::new("token")
                .short('t')
                .long("token")
                .env("GITLAB_ACCESS_TOKEN")
                .takes_value(true)
                .help("GitLab Access Token"),
        )
        .arg(
            Arg::new("url")
                .short('u')
                .long("url")
                .env("GITLAB_PIPELINE_URL")
                .takes_value(true)
                .help("GitLab CI pipeline URL"),
        );

    let matches = app.get_matches();
    let token = matches.value_of("token").unwrap();
    let url = matches.value_of("url").unwrap();

    let ten_secs = time::Duration::from_secs(10);

    loop {
        let res = get_pipe_status(url, token).await.unwrap();
        let status = res.detailed_status.text;

        if status == "passed" {
            std::process::exit(0);
        } else if status == "failed" {
            eprintln!("pipeline failed");
            std::process::exit(1);
        } else {
            thread::sleep(ten_secs);
        }
    }
}
