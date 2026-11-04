import { expect, it } from 'vitest';
import { clientRequestHeaders } from '@/lib/utils/http/client-request';

it('encodes optional collected metadata without inventing missing fields', () => {
  const metadata = { locale: 'fr-FR', distribution: 'installed' as const };
  expect(clientRequestHeaders(metadata)).toEqual({
    'Accept-Language': 'fr-FR',
    'x-truongphatcomputer-locale': 'fr-FR',
    'x-truongphatcomputer-distribution': 'installed',
  });
  expect(metadata).toEqual({ locale: 'fr-FR', distribution: 'installed' });
  expect(clientRequestHeaders({ ...metadata, installId: 'fixture', osVersion: '11', timezone: 'UTC' })).toMatchObject({
    'x-truongphatcomputer-install-id': 'fixture',
    'x-truongphatcomputer-os-version': '11',
    'x-truongphatcomputer-timezone': 'UTC',
  });
});
