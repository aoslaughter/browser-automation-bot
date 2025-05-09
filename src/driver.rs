use anyhow::Result;
use std::process::{Command, Child};
use std::time::Duration;
use thirtyfour::prelude::*;

pub struct WebDriverSession {
    pub driver: WebDriver,
    chromedriver_process: Option<Child>,
}

impl WebDriverSession {
    pub async fn new(webdriver_url: &str) -> Result<Self> {
        // Attempt ChromeDriver start
        let chromedriver_process = if webdriver_url.contains("localhost") {
            let process = Command::new("chomedriver")
                .args(&["--port=4444"])
                .spawn()
                .ok();

            // Pause to let ChromeDriver start
            tokio::time::sleep(Duration::from_secs(2)).await;
            process
        } else {
            None
        };

        // Connect to WebDriver
        let mut caps = DesiredCapabilities::chrome();
        caps.set_no_sandbox()?;

        let driver = WebDriver::new(webdriver_url, caps).await?;

        // Set default timeouts
        driver.set_implicit_wait_timeout(Duration::from_secs(10)).await?;

        Ok(Self {
            driver,
            chromedriver_process,
        })
    }

    pub async fn close(mut self) -> Result<()> {
        // Quit WebDriver
        self.driver.quit().await?;

        // Stop ChromeDriver provess if we started it
        if let Some(mut process) = self.chromedriver_process.take() {
            let _ = process.kill();
        }

        Ok(())
    }
}