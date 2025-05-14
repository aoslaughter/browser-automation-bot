use anyhow::Result;
use thirtyfour::prelude::*;
use crate::utils::sites::SiteConfig;

pub struct HomePage {
    driver: WebDriver,
    form_element: WebElement,
}

impl HomePage {
    pub fn new(driver: WebDriver, form_element: WebElement) -> Self {
        Self {
            driver,
            form_element,
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
        self.search_by_zipcode(zip_id, zipcode);
        self.input_date(date_id, date);
        self.submit(submit_css);

        Ok(())
    }

    async fn search_by_zipcode(&self, zip_id: &str, zipcode: &str) -> Result<()> {
        // Find the search input field. search_location
        let search_input = self.form_element.find(By::Id(zip_id)).await?;
        search_input.clear().await?;
        search_input.send_keys(zipcode).await?;

        Ok(())
    }

    async fn input_date(&self, date_id: &str, date: &str) -> Result<()> {
        // Find date field, search_friendly_date
        let date_input = self.form_element.find(By::Id(date_id)).await?;
        date_input.clear().await?;
        date_input.send_keys(date).await?;

        Ok(())
    }

    async fn submit(&self, submit_css: &str) -> Result<()> {
        let submit_button = self.form_element.find(By::Css(submit_css)).await?;
        submit_button.click().await?;

        Ok(())
    }
}