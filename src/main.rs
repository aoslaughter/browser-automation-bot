use anyhow::Result;
use bot_core::driver::WebDriverSession;
use std::error::Error;
use std::time::Duration;
use thirtyfour::prelude::*;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let caps = DesiredCapabilities::chrome();
    let driver = WebDriver::new("http://Localhost:41605", caps).await?;
    let url = "https://www.playlocal.com/";
    let form_elem = ".facility-form";
    let location_elem = "search_location";
    let location_zip = "02476";
    let submit_elem = ".cpy-find-courts";
    
    // Explicit wait for page load.
    driver.set_implicit_wait_timeout(Duration::from_secs(5)).await?;

    // Navigate to  url
    driver.goto(url).await?;
    let form_elem = driver.find(By::Css(form_elem)).await?;

    // Find element.
    let location_input = form_elem.find(By::Id(location_elem)).await?;

    // Type in the search terms.
    location_input.send_keys(location_zip).await?;

    // Click the search button.
    let submit_button = form_elem.find(By::Css(submit_elem)).await?;
    submit_button.click().await?;

    // Look for head to imlicitly wait for the page to load.
    // driver.query(By::ClassName("firstHeading")).first().await?;
    // assert_eq!(driver.title().await?, "title");

    // Wait for page to load
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Explicitly close the browser.
    driver.quit().await?;

    Ok(())
}
