//! Native surfaces reuse the same locale resources as Vue.
use serde_json::Value;
use std::sync::OnceLock;
use tauri_plugin_store::StoreExt;

pub struct NativeLabels {
    pub locale: String,
    messages: &'static Value,
}
impl NativeLabels {
    pub fn load(app: &tauri::AppHandle) -> Self {
        let saved = app
            .store_builder("settings.json")
            .disable_auto_save()
            .build()
            .ok()
            .and_then(|store| store.get("settings"))
            .and_then(|value| {
                value
                    .get("language")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            });
        match saved {
            Some(language) => Self::for_locale(&language),
            None => Self::for_system_locale(tauri_plugin_os::locale().as_deref()),
        }
    }
    /// First launch follows the frontend language policy: a supported, non-English
    /// system language is honored, while English keeps the Vietnamese product default.
    pub fn for_system_locale(locale: Option<&str>) -> Self {
        let detected = supported_locale(locale.unwrap_or_default());
        Self::for_locale(if detected == "en-US" {
            "vi-VN"
        } else {
            detected
        })
    }
    pub fn for_locale(locale: &str) -> Self {
        let locale = supported_locale(locale);
        static MESSAGES: OnceLock<[Value; 12]> = OnceLock::new();
        let values = MESSAGES.get_or_init(|| {
            [
                include_str!("../../../src/locales/en-US.json"),
                include_str!("../../../src/locales/zh-CN.json"),
                include_str!("../../../src/locales/zh-TW.json"),
                include_str!("../../../src/locales/ja-JP.json"),
                include_str!("../../../src/locales/ko-KR.json"),
                include_str!("../../../src/locales/tr-TR.json"),
                include_str!("../../../src/locales/pt-BR.json"),
                include_str!("../../../src/locales/ru-RU.json"),
                include_str!("../../../src/locales/fr-FR.json"),
                include_str!("../../../src/locales/de-DE.json"),
                include_str!("../../../src/locales/es-ES.json"),
                include_str!("../../../src/locales/vi-VN.json"),
            ]
            .map(|text| serde_json::from_str(text).expect("validated locale resource"))
        });
        let index = match locale {
            "zh-CN" => 1,
            "zh-TW" => 2,
            "ja-JP" => 3,
            "ko-KR" => 4,
            "tr-TR" => 5,
            "pt-BR" => 6,
            "ru-RU" => 7,
            "fr-FR" => 8,
            "de-DE" => 9,
            "es-ES" => 10,
            "vi-VN" => 11,
            _ => 0,
        };
        Self {
            locale: locale.into(),
            messages: &values[index],
        }
    }
    pub fn text(&self, key: &str) -> &str {
        self.messages
            .pointer(key)
            .and_then(Value::as_str)
            .unwrap_or("—")
    }
}

/// Match the frontend language-prefix policy for installations that have not
/// persisted a language yet. Traditional Chinese must precede generic Chinese.
fn supported_locale(locale: &str) -> &'static str {
    let locale = locale.trim().to_ascii_lowercase();
    for (prefix, supported) in [
        ("zh-tw", "zh-TW"),
        ("zh-hk", "zh-TW"),
        ("zh-mo", "zh-TW"),
        ("zh-hant", "zh-TW"),
        ("zh", "zh-CN"),
        ("ja", "ja-JP"),
        ("ko", "ko-KR"),
        ("tr", "tr-TR"),
        ("pt", "pt-BR"),
        ("ru", "ru-RU"),
        ("fr", "fr-FR"),
        ("de", "de-DE"),
        ("es", "es-ES"),
        ("vi", "vi-VN"),
        ("en", "en-US"),
    ] {
        if locale == prefix || locale.starts_with(&format!("{prefix}-")) {
            return supported;
        }
    }
    // Vietnamese is the product language and also covers unsupported system locales.
    "vi-VN"
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn runtime_prompts_are_complete_in_every_supported_locale() {
        for locale in [
            "en-US", "zh-CN", "zh-TW", "ja-JP", "ko-KR", "tr-TR", "pt-BR", "ru-RU", "fr-FR",
            "de-DE", "es-ES", "vi-VN",
        ] {
            let labels = NativeLabels::for_locale(locale);
            for key in [
                "updateRequired",
                "update",
                "exit",
                "openFailed",
                "recoveryFailed",
            ] {
                let value = labels.text(&format!("/webviewRuntime/{key}"));
                assert!(!value.is_empty() && value != "—", "{locale}: {key}");
            }
            assert!(labels.text("/webviewRuntime/openFailed").contains("{url}"));
        }
    }

    #[test]
    fn new_locales_render_native_menu_actions_without_fallback() {
        for (locale, open, settings, quit) in [
            (
                "fr-FR",
                "Ouvrir Trường Phát Computer",
                "Réglages",
                "Quitter",
            ),
            (
                "de-DE",
                "Trường Phát Computer öffnen",
                "Einstellungen",
                "Beenden",
            ),
            ("es-ES", "Abrir Trường Phát Computer", "Ajustes", "Salir"),
            ("vi-VN", "Mở Trường Phát Computer", "Cài đặt", "Thoát"),
        ] {
            let labels = NativeLabels::for_locale(locale);
            assert_eq!(labels.text("/systemStatus/openMain"), open);
            assert_eq!(labels.text("/systemStatus/settings"), settings);
            assert_eq!(labels.text("/systemStatus/quit"), quit);
        }
    }

    #[test]
    fn native_defaults_match_supported_browser_language_prefixes() {
        for (input, expected) in [
            ("zh-CN", "zh-CN"),
            ("zh-Hant-HK", "zh-TW"),
            ("zh-SG", "zh-CN"),
            ("ja", "ja-JP"),
            ("ko-KR", "ko-KR"),
            ("ru-RU", "ru-RU"),
            ("ru", "ru-RU"),
            ("fr", "fr-FR"),
            ("de", "de-DE"),
            ("es", "es-ES"),
            ("en-GB", "en-US"),
            ("tr", "tr-TR"),
            (" TR-tr ", "tr-TR"),
            ("pt-BR", "pt-BR"),
            ("pt-PT", "pt-BR"),
            ("pt", "pt-BR"),
            ("tricky", "vi-VN"),
            ("fr-FR", "fr-FR"),
            (" FR-ca ", "fr-FR"),
            ("de-DE", "de-DE"),
            ("de-AT", "de-DE"),
            ("es-ES", "es-ES"),
            ("es-MX", "es-ES"),
            ("vi", "vi-VN"),
            ("vi-VN", "vi-VN"),
            ("french", "vi-VN"),
            ("desktop", "vi-VN"),
            ("estonian", "vi-VN"),
            ("it-IT", "vi-VN"),
        ] {
            assert_eq!(NativeLabels::for_locale(input).locale, expected);
        }
    }

    #[test]
    fn first_launch_keeps_the_vietnamese_product_default() {
        for (input, expected) in [
            (None, "vi-VN"),
            (Some("en-US"), "vi-VN"),
            (Some("en-GB"), "vi-VN"),
            (Some("vi-VN"), "vi-VN"),
            (Some("de-DE"), "de-DE"),
            (Some("ja-JP"), "ja-JP"),
            (Some("it-IT"), "vi-VN"),
        ] {
            assert_eq!(NativeLabels::for_system_locale(input).locale, expected);
        }
    }
}
