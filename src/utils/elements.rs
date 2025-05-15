use thirtyfour::{
    common::types::DynElementPredicate, WebElement
};

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