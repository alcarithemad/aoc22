pub mod rps;

pub async fn download(day: &str) -> Result<String, anyhow::Error> {
    let path = std::path::PathBuf::from(format!("inputs/day{day}/input"));
    if !path.exists() {
        println!("downloading input for day {day}");
        let url = format!("https://adventofcode.com/2022/day/{day}/input");
        let session = std::env::var("AOC_SESSION")?;
        let client = reqwest::ClientBuilder::new()
            .user_agent("alcarithemad@gmail.com")
            .build()?;
        let input_data = client
            .get(url)
            .header("Cookie", format!("session={session}"))
            .send()
            .await?
            .text()
            .await?;
        std::fs::write(path, &input_data)?;
        Ok(input_data)
    } else {
        let input_data = std::fs::read_to_string(path)?;
        Ok(input_data)
    }
}

#[macro_export]
macro_rules! get_input {
    () => {{
        let day = std::path::PathBuf::from(file!())
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".rs")
            .unwrap()
            .strip_prefix("day")
            .unwrap()
            .to_owned();
        let data = aoc22::download(&*day).await?;
        data
    }};
}
