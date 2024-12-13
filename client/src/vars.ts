import type { ClientKeyId, FormId, SecretLinkKey } from "./crypto";
import { encodeBase64Url } from "./encoding";

export const TOAST_INFO_TTL = 2500;
export const TOAST_ERROR_TTL = 3500;

export const newShareLink = (formId: FormId) =>
  new URL(`${window.location.origin}/share/#/${formId}`);

export const newSecretLink = (
  formId: FormId,
  clientKeyId: ClientKeyId,
  secretLinkKey: SecretLinkKey,
) =>
  new URL(
    `${window.location.origin}/view/#/${formId}/${clientKeyId}/${encodeBase64Url(secretLinkKey)}`,
  );

export const CONTACT_METHODS = [
  "Email",
  "SMS",
  "Signal",
  "Telegram",
  "Discord",
  "Mastodon",
  "Bluesky",
  "Threads",
  "Twitter",
  "Matrix",
  "WhatsApp",
  "Instagram",
  "Facebook",
  "Snapchat",
  "WeChat",
] as const;

export type ContactMethod = (typeof CONTACT_METHODS)[number];

export const TITLE_LEADS = [
  "Can I mobilize my community?",
  "Can I enact lasting change?",
  "Can I make a real difference?",
  "Can I start a movement?",
  "Can I change the world?",
];

const pickRandom = <T>(arr: T[]) => arr[Math.floor(Math.random() * arr.length)];

export const randomTitleLead = () => pickRandom(TITLE_LEADS);
