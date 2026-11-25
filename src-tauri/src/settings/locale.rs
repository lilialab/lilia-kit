//! 界面语言：偏好 → 已解析 locale（托盘等壳层文案）。

use super::model::{AppLocale, LocalePreference};

/// 按偏好解析；`system` 时读 OS，非中文回落英文。
pub fn resolve(preference: LocalePreference) -> AppLocale {
    match preference {
        LocalePreference::Zh => AppLocale::Zh,
        LocalePreference::En => AppLocale::En,
        LocalePreference::System => from_system_tag(system_locale_tag()),
    }
}

fn system_locale_tag() -> String {
    sys_locale::get_locale().unwrap_or_default()
}

fn from_system_tag(tag: String) -> AppLocale {
    let lower = tag.to_ascii_lowercase();
    if lower.starts_with("zh") {
        AppLocale::Zh
    } else {
        AppLocale::En
    }
}

#[cfg(test)]
mod tests {
    use super::from_system_tag;
    use crate::settings::model::AppLocale;

    #[test]
    fn chinese_tags_map_to_zh() {
        assert_eq!(from_system_tag("zh-CN".into()), AppLocale::Zh);
        assert_eq!(from_system_tag("zh_TW".into()), AppLocale::Zh);
    }

    #[test]
    fn non_chinese_falls_back_to_en() {
        assert_eq!(from_system_tag("en-US".into()), AppLocale::En);
        assert_eq!(from_system_tag("ja-JP".into()), AppLocale::En);
        assert_eq!(from_system_tag("".into()), AppLocale::En);
    }
}
