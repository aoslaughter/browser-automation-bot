use thirtyfour::{
    common::types::DynElementPredicate, WebElement, By
};
use anyhow::Result;
use std::time::Duration;

pub fn is_displayed() -> Box<DynElementPredicate> {
    DynElementPredicate::boxed(|el: WebElement| async move {
        el.is_displayed().await
    })
}

pub fn is_enabled() -> Box<DynElementPredicate> {
    DynElementPredicate::boxed(|el: WebElement| async move {
        el.is_enabled().await
    })
}

pub fn element_is_interactable() -> Vec<Box<DynElementPredicate>> {
    vec![is_displayed(), is_enabled()]
}

pub async fn select_from_datepicker(element: WebElement, day: u32) -> Result<()> {
    // Collect selectable, active <td> elements
    let date_cells = element.find_all(By::Css(
            "td:not(.ui-datepicker-other-month ui-datepicker-unselectable)"
        )).await?;

    // Loop through elements
    for cell in date_cells {
        // Find <a> tags
        if let Ok(link) = cell.find(By::Tag("a")).await {
            if let Ok(text) = link.text().await {
                // Identify the <a> tag containing the right date
                if text.trim() == day.to_string() {
                    link.click().await?;
                    tokio::time::sleep(Duration::from_millis(250)).await;
                    return Ok(())
                }
            }
        }
    }

        Ok(())
}