use anyhow::Result;
use bot_core::{
    driver::WebDriverSession,
    utils::config::Config,
    pages::home_page::HomePage,
};
use std::error::Error;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let config = Config::from_file("config/config.toml");

    // Initialize WebDriver
    let session = WebDriverSession::new(&config.unwrap().webdriver_url).await?;
    let form_elem = &config.unwrap().home_dom.form_element;
    let form_element = session
        .driver
        .find(By::Css(form_elem)).await?;

    let home_page = HomePage::new(
        session.driver.clone(),
        form_element
    );

    // let caps = DesiredCapabilities::chrome();
    // let driver = WebDriver::new("http://Localhost:41605", caps).await?;
    // let url = "https://www.playlocal.com/";
    // Explicit wait for page load.
    // driver.set_implicit_wait_timeout(Duration::from_secs(5)).await?;
    let form_elem = ".facility-form";
    let location_elem = "search_location";
    let location_zip = "02476";
    let submit_elem = ".cpy-find-courts";

    // Navigate to  url
    // driver.goto(url).await?;
    // let form_elem = driver.find(By::Css(form_elem)).await?;

    // Find element.
    // let location_input = form_elem.find(By::Id(location_elem)).await?;

    // Type in the search terms.
    // location_input.send_keys(location_zip).await?;

    // Click the search button.
    // let submit_button = form_elem.find(By::Css(submit_elem)).await?;
    // submit_button.click().await?;

    // Look for head to imlicitly wait for the page to load.
    // driver.query(By::ClassName("firstHeading")).first().await?;
    // assert_eq!(driver.title().await?, "title");

    // Wait for page to load
    // tokio::time::sleep(Duration::from_secs(5)).await;

    // Explicitly close the browser.
    // driver.quit().await?;

    Ok(())
}
