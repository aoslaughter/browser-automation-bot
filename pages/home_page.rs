use anyhow::Result;
use thirtyfour::prelude::*;

pub struct HomePage {
    driver: WebDriver,
    form_element: WebElement,
    url: String,
}

impl HomePage {
    pub fn new(driver: WebDriver, form_element: WebElement, url: &str) -> Self {
        Self { driver, form_element, url }
    }

    pub async fn navigate(&self) -> Result<()> {
        self.driver.goto(self.url).await?;
        Ok(())
    }

    pub async fn isearch_by_params(
        &self, 
        zip_id: &str, 
        zipcode: &str,
        date_id: &str, 
        date: &str,
        submit_css: &str,
    ) -> Result<()> {
        search_by_zipcode(zip_id, zipcode);
        input_date(date_id, date);
        submit(submit_css);

        Ok(())
    }

    async fn search_by_zipcode(&self, zip_id: &str, zipcode: &str) -> Result<()> {
        // Find the search input field. search_location
        let search_input = self.form_element.find(By::Id(zip_id)).await?;
        search_input.clear().await?;
        search_input.send_keys(zipcode).await?;
    }

    async fn input_date(&self, date_id: &str, date: &str) -> Result<()> {
        // Find date field, search_friendly_date
        let date_input = self.form_element.find(By::Id(date_id)).await?;
        date_input.clear().await?;
        dat_input.send_keys(date).await?;
    }

    async fn submit(&self, submit_css: &str) -> Result<()> {
        let submit_button = self.form_element.find(By::Css(submit_css)).await?;
        submit_button.click().await?;
    }
}