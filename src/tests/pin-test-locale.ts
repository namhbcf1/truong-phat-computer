import { beforeEach } from 'vitest';

import { i18n } from '@/i18n';
import { LANGUAGE_IDS } from '@/lib/models/settings/settings';

/*
 * Component fixtures assert the authored English copy. The product applies
 * Vietnamese at startup through LanguageService, so pinning the composer keeps
 * these tests independent of the shipped default language.
 */
beforeEach(() => {
  i18n.global.locale.value = LANGUAGE_IDS.enUS;
});
