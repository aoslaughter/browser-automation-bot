use anyhow::Result;
use thirtyfour::prelude::*;
use std::time::Duration;
use crate::utils::elements::*;

pub struct HomePage {
    driver: WebDriver,
    // form_element: WebElement,
}

impl HomePage {
    pub fn new(driver: WebDriver) -> Self { // , form_element: WebElement
        Self {
            driver,
            // form_element,
        }
    }

    pub async fn navigate(&self, base_url: &str) -> Result<()> {
        self.driver.goto(base_url).await?;
        
        Ok(())
    }

    pub async fn search_by_params(
        &self, 
        zip_id: &str, 
        zipcode: &str,
        date_id: &str, 
        date: &str,
        submit_css: &str,
    ) -> Result<()> {
        self.search_by_zipcode(zip_id, zipcode).await?;
        self.input_date(date_id, date).await?;
        self.submit(submit_css).await?;

        Ok(())
    }

    async fn search_by_zipcode(&self, zip_id: &str, zipcode: &str) -> Result<()> {
        // Find the search input field. search_location
        let search_input = self.driver.find(By::Id(zip_id)).await?;

        search_input.wait_until()
            .wait(Duration::from_secs(10), Duration::from_millis(500))
            .conditions(element_is_interactable())
            .await?;

        // Logging to see element state
        println!(
            "Search input displayed: {:?}", 
            search_input.is_displayed().await
        );
        println!(
            "Search input is enabled: {:?}",
            search_input.is_enabled().await
        );

        search_input.clear().await?;
        search_input.send_keys(zipcode).await?;

        Ok(())
    }

    async fn input_date(&self, date_id: &str, date: &str) -> Result<()> {
        // Find date field, search_friendly_date
        let date_input = self.driver.find(By::Id(date_id)).await?;

        date_input.wait_until()
            .wait(Duration::from_secs(10), Duration::from_millis(500))
            .conditions(element_is_interactable())
            .await?;

        // Logging to see element state
        println!(
            "Date input displayed: {:?}", 
            date_input.is_displayed().await
        );
        println!(
            "Date input is enabled: {:?}",
            date_input.is_enabled().await
        );

        // Send date, wait, and escape to close date picker
        date_input.send_keys(date).await?;
        tokio::time::sleep(Duration::from_millis(300)).await;
        date_input.send_keys(Key::Enter).await?;


        Ok(())
    }

    async fn submit(&self, submit_css: &str) -> Result<()> {
        let submit_button = self.driver.find(By::Css(submit_css)).await?;

        submit_button.wait_until()
            .wait(Duration::from_secs(10), Duration::from_millis(500))
            .conditions(element_is_interactable())
            .await?;

        // Logging to see element state
        println!(
            "Submit button displayed: {:?}", 
            submit_button.is_displayed().await
        );
        println!(
            "Submit button is enabled: {:?}",
            submit_button.is_enabled().await
        );

        submit_button.click().await?;

        Ok(())
    }
}