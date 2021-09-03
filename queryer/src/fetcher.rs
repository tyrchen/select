use anyhow::{anyhow, Result};
use async_trait::async_trait;
use regex::Regex;
use std::process::Command;
use tokio::fs;
use tracing::info;

// Rust 的 async trait 还没有稳定，可以用 async_trait 宏
#[async_trait]
pub trait Fetch {
    type Error;
    async fn fetch(&self) -> Result<String, Self::Error>;
}

/// 从文件源或者 http 源中获取数据，返回字符串
pub async fn retrieve_data(source: impl AsRef<str>) -> Result<String> {
    let name = source.as_ref();
    let err = anyhow!("Please make sure the source after FROM starts with http://, https://, file:// and stdout://");
    if name.len() < 4 {
        return Err(err);
    }
    match &name[..4] {
        // process http / https
        "http" => UrlFetcher(name).fetch().await,
        // process file://<filename>
        "file" if &name[4..7] == "://" => FileFetcher(&name[7..]).fetch().await,
        // process stdout::/<cmd>
        "stdo" if &name[4..9] == "ut://" => StdoutFetcher(&name[9..]).fetch().await,
        _ => Err(err),
    }
}

struct UrlFetcher<'a>(pub(crate) &'a str);
struct FileFetcher<'a>(pub(crate) &'a str);
struct StdoutFetcher<'a>(pub(crate) &'a str);

#[async_trait]
impl<'a> Fetch for UrlFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(reqwest::get(self.0).await?.text().await?)
    }
}

#[async_trait]
impl<'a> Fetch for FileFetcher<'a> {
    type Error = anyhow::Error;

    ///
    async fn fetch(&self) -> Result<String, Self::Error> {
        Ok(fs::read_to_string(&self.0).await?)
    }
}

#[async_trait]
impl<'a> Fetch for StdoutFetcher<'a> {
    type Error = anyhow::Error;

    async fn fetch(&self) -> Result<String, Self::Error> {
        let result = if cfg!(target_os = "windows") {
            // windows may not work
            Command::new("cmd")
                .args(&["/C", self.0])
                .output()
                .expect("failed to execute process")
                .stdout
        } else {
            // for *nix, this is a shitty hack that use awk to parse the command output
            let args = (1..12)
                .map(|i| format!("${}", i))
                .collect::<Vec<_>>()
                .join("\",\"\",\"");
            let cmd = format!("{} | awk '{{print {}}}'", self.0, args);
            info!("cmd: {:?}", cmd);
            Command::new("bash")
                .args(&["-c", &cmd])
                .output()
                .expect("failed to execute process")
                .stdout
        };
        let output = String::from_utf8(result)?;
        let re = Regex::new(r"(,,)+").unwrap();
        let transformed = re.replace_all(&output, ",");
        let transformed = transformed.replace(",\n", "\n");
        info!("transformed: {}", transformed);
        Ok(transformed)
    }
}
