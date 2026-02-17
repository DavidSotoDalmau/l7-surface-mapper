use crate::models::ResponseInfo;

pub fn detect_cms(resp: &ResponseInfo) -> Option<String> {

    if let Some(body) = &resp.body_sample {
        let lower = body.to_lowercase();

        // WordPress
        if lower.contains("/wp-content/") ||
           lower.contains("/wp-includes/") ||
           lower.contains("wp-json") {
            return Some("WordPress".into());
        }

        // Drupal
        if lower.contains("drupalsettings") ||
           lower.contains("x-drupal-cache") ||
           lower.contains("/sites/default/") {
            return Some("Drupal".into());
        }

        // Joomla
        if lower.contains("/templates/") &&
           lower.contains("joomla") {
            return Some("Joomla".into());
        }

        // Adobe Experience Manager
        if lower.contains("/etc.clientlibs/") ||
           lower.contains("/content/dam/") ||
           lower.contains("granite") {
            return Some("Adobe Experience Manager (AEM)".into());
        }

        // Sitecore
        if lower.contains("/sitecore/") ||
           lower.contains("sc_analytics_global_cookie") {
            return Some("Sitecore".into());
        }

        // Liferay
        if lower.contains("liferay.themedisplay") ||
           lower.contains("/c/portal/") {
            return Some("Liferay".into());
        }

        // Magento
        if lower.contains("/static/frontend/") ||
           lower.contains("mage/") {
            return Some("Magento".into());
        }

        // Shopify
        if lower.contains("cdn.shopify.com") {
            return Some("Shopify".into());
        }

        // SAP Hybris
        if lower.contains("yacceleratorstorefront") {
            return Some("SAP Hybris".into());
        }
    }

    None
}
