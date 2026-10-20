//! Adobado Kit community and project links (Help menu, About, start screen, CLI, MCP).

use serde_json::{Value, json};

/// The public repository name for this app.
pub const APP: &str = "DesignAdobado";
/// Where users can report a problem or request a feature.
pub const SUPPORT: &str = "https://github.com/smouj/DesignAdobado/issues";
/// The Adobado Kit website.
pub const WEBSITE: &str = "https://smouj.github.io/Adobado_Kit/";
/// DesignAdobado's project page.
pub const APP_PAGE: &str = "https://github.com/smouj/DesignAdobado";
/// DesignAdobado's source repository.
pub const GITHUB: &str = "https://github.com/smouj/DesignAdobado";
/// Where to report bugs and request features.
pub const ISSUES: &str = "https://github.com/smouj/DesignAdobado/issues";

/// All links as JSON (the `app.links` command).
pub fn all() -> Value {
    json!({
        "support": SUPPORT,
        "website": WEBSITE,
        "appPage": APP_PAGE,
        "github": GITHUB,
        "issues": ISSUES,
    })
}

/// A link by key. `discord` remains a compatibility alias for the former support link.
pub fn get(key: &str) -> Option<&'static str> {
    Some(match key {
        "support" | "discord" => SUPPORT,
        "website" => WEBSITE,
        "appPage" => APP_PAGE,
        "github" => GITHUB,
        "issues" => ISSUES,
        _ => return None,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn links_follow_the_app_slug() {
        assert_eq!(APP_PAGE, GITHUB);
        assert!(GITHUB.ends_with(&format!("/smouj/{APP}")));
        assert!(ISSUES.starts_with(GITHUB));
        for k in ["support", "website", "appPage", "github", "issues"] {
            assert!(get(k).is_some_and(|u| u.starts_with("https://")), "{k}");
            assert_eq!(all()[k], get(k).unwrap());
        }
        assert_eq!(get("discord"), Some(SUPPORT));
    }
}
