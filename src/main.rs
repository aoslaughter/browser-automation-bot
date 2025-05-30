#![allow(unused_imports)]
#![allow(unused_variables)]

use anyhow::Result;
use bot_core::{
    driver::WebDriverSession,
    utils::{
        config::Config,
        conversions::*,
    },
    urls::builders::*,
    pages::home_page::HomePage,
};
use std::{error::Error, hash::Hash};
use std::time::Duration;
use std::collections::HashMap;
use thirtyfour::prelude::*;
use tokio;
use chrono::{
    Datelike, Local, TimeDelta, TimeZone
};

#[tokio::main]
async fn main() -> Result<()> {
    // Load configuration
    let mut config = Config::from_file("config/config.toml").unwrap();

    // Initialize WebDriver
    let session = WebDriverSession::new(&config.webdriver_url).await?;
    
    let (time_of_day, target_date) = date_time_generator(
        &config.reservation.weeks
    ).unwrap();
    
    config.set_date(target_date.clone());
    config.set_time(time_of_day);

    let params = config.to_search_params();
    let url = &config.site.base_url;

    let results_url = query_builder(&url, &params).unwrap();

    session.navigate(&results_url.to_string()).await?;

    tokio::time::sleep(Duration::from_secs(2)).await;

    // Explicitly close the browser.
    // driver.quit().await?;

    session.close().await?;

    Ok(())
}


    // Navigate to  url
    // driver.goto(url).await?;
    // let form_elem = driver.find(By::Css(form_elem)).await?;

    // let home_page = HomePage::new(
    //     session.driver.clone(),
    // );

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

    // home_page.search_by_params(
    //     &config.home_dom.location_element,
    //     &config.search.location,
    //     &config.home_dom.date_element,
    //     &target_date,
    //     &config.home_dom.date_picker,
    //     &config.home_dom.submit_element
    // ).await?;

    // let caps = DesiredCapabilities::chrome();
    // let driver = WebDriver::new("http://Localhost:41605", caps).await?;
    // let url = "https://www.playlocal.com/";
    // Explicit wait for page load.
    // driver.set_implicit_wait_timeout(Duration::from_secs(5)).await?;
    // let form_elem = ".facility-form";
    // let location_elem = "search_location";
    // let location_zip = "02476";
    // let submit_elem = ".cpy-find-courts";

    // Wait for page to load