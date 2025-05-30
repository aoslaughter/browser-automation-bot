use url::{Url, form_urlencoded};
use std::collections::HashMap;

pub fn query_builder(base_url: &String, params: &HashMap<String, String>) -> Result<Url, url::ParseError> {
    let mut url = Url::parse(base_url)?;

    // for (k, v) in params {
    //     url.query_pairs_mut().append_pair(
    //         k.as_str(),
    //         v.as_str()
    //     )
    // }

    url.query_pairs_mut()
        .extend_pairs(params.iter().map(|(k, v)| (k.as_str(), v.as_str())));

    Ok(url)
}