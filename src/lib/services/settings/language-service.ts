import { i18n } from '@/i18n';
import { isLanguageId, LANGUAGE_IDS, LANGUAGE_OPTIONS } from '@/lib/models/settings/settings';
import type { AppSettings } from '@/lib/models/settings/settings';

export class LanguageService {
  static detectSystemLanguage(): AppSettings['language'] {
    if (typeof navigator === 'undefined') return LANGUAGE_IDS.viVN;
    const languages = navigator.languages?.length ? navigator.languages : [navigator.language];
    const detected = this.resolveSupportedLanguage(languages);
    // Vietnamese is the product default. Earlier releases used English as the
    // fallback for every system language, so an English system language means
    // "no explicit language choice" and keeps the product default.
    return detected === LANGUAGE_IDS.enUS ? LANGUAGE_IDS.viVN : detected;
  }

  static resolveSupportedLanguage(languages: readonly string[]): AppSettings['language'] {
    for (const language of languages) {
      const normalized = language.trim().toLowerCase();
      const option = LANGUAGE_OPTIONS.find(candidate =>
        candidate.browserLanguagePrefixes.some(prefix => normalized === prefix || normalized.startsWith(`${prefix}-`))
      );
      if (option) return option.id;
    }
    return LANGUAGE_IDS.enUS;
  }

  static apply(language: AppSettings['language']): void {
    const locale = isLanguageId(language) ? language : LANGUAGE_IDS.viVN;
    i18n.global.locale.value = locale;
    document.documentElement.lang = locale;
  }
}
