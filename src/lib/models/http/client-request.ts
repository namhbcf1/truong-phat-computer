import type { AppDistribution } from '@/lib/models/updates/app-update';

/** Collected facts, independent of HTTP or IPC encoding. Each caller owns its failure policy. */
export interface ClientRequestMetadata {
  locale: string;
  distribution: AppDistribution;
  installId?: string;
  osVersion?: string;
  timezone?: string;
}

/** Stable desktop metadata shared by update telemetry and official AI signatures. */
export const CLIENT_REQUEST_HEADERS = {
  distribution: 'x-truongphatcomputer-distribution',
  installId: 'x-truongphatcomputer-install-id',
  locale: 'x-truongphatcomputer-locale',
  osVersion: 'x-truongphatcomputer-os-version',
  timezone: 'x-truongphatcomputer-timezone',
  appVersion: 'x-truongphatcomputer-app-version',
  requestId: 'x-request-id',
} as const;
