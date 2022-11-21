use std::process::{Command, Output};
use std::io::{Result, Error, ErrorKind};
use std::env;

#[derive(Debug)]
#[allow(dead_code)]
pub enum Browser {
    Default,
    Firefox,
    InternetExplorer,
    Chrome,
    Opera,
    Safari
}

pub fn open(url: &str) -> Result<Output> {
    open_browser(Browser::Default, url)
}

//

pub fn open_browser(browser: Browser, url: &str) -> Result<Output> {
    let os = std::env::consts::OS;
    match os {
        "macos" => open_on_macos(browser, url),
        "windows" => open_on_windows(browser, url),
        "linux" => open_on_linux(browser, url),
        _ => Err(Error::new(ErrorKind::NotFound, format!("Platform {} not yet supported by this library", os)))
    }
}

fn open_on_macos(browser: Browser, url: &str) -> Result<Output> {
    let mut cmd = Command::new("open");
    match browser {
        Browser::Default => cmd.arg(format!("https://www.google.com/search?q={url}")).output(),
        _ => {
            let app: Option<&str> = match browser {
                Browser::Firefox => Some("Firefox"),
                Browser::Chrome => Some("Google Chrome"),
                Browser::Opera => Some("Opera"),
                Browser::Safari => Some("Safari"),
                _ => None
            };
            match app {
                Some(name) => cmd.arg("-a").arg(name).arg(url).output(),
                None => Err(Error::new(ErrorKind::NotFound, format!("Unsupported browser {:?}", browser)))
            }
        }
    }
}

fn open_on_windows(browser: Browser, url: &str) -> Result<Output> {
    match browser {
        Browser::Default => Command::new("start").arg("link").arg(url).output(),
        _ => Err(Error::new(
                ErrorKind::NotFound,
                "Only the default browser is supported on this platform right now"
            ))
    }
}

fn open_on_linux(browser: Browser, url: &str) -> Result<Output> {
    match browser {
        Browser::Default => Command::new(env::var("BROWSER").or::<Result<String>>(Ok("xdg-open".to_string())).unwrap())
            .arg(url)
            .output(),
        _ => Err(Error::new(
                ErrorKind::NotFound,
                "Only the default browser is supported on this platform right now"
            ))
    }
}

