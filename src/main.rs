use clap::Parser;
use reqwest;
use tokio;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    // User specified location
    #[arg(short, long)]
    location: Option<String>,

    #[arg(short, long, default_value = "3")]
    format: Option<String>,
}

fn args2url(args: Args) -> String {
    format!(
        "https://wttr.in/{}{}",
        args.location.map_or_else(|| "".to_string(), |loc| loc),
        args.format
            .map_or_else(|| "".to_string(), |fmt| format!("?format={}", fmt))
    )
}

async fn get_weather(args: Args) -> Result<String, reqwest::Error> {
    let url = args2url(args);
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?)
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let weather = get_weather(args).await;
    print!("{}", weather.unwrap());
}
