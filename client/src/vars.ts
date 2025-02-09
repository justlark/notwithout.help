import type { ClientKeyId, FormId, ProtectedSecretLinkKey, SecretLinkKey } from "./crypto";
import { encodeBase64Url } from "./encoding";

export const TOAST_INFO_TTL = 2500;
export const TOAST_ERROR_TTL = 3500;

export const newShareLink = (formId: FormId) =>
  new URL(`${window.location.origin}/share/#/${formId}`);

export const newSecretLink = (
  formId: FormId,
  clientKeyId: ClientKeyId,
  secretLinkKey: SecretLinkKey | ProtectedSecretLinkKey,
) =>
  new URL(
    `${window.location.origin}/view/#/${formId}/${clientKeyId}/${encodeBase64Url(secretLinkKey)}`,
  );

export const newEditLink = (
  formId: FormId,
  clientKeyId: ClientKeyId,
  secretLinkKey: SecretLinkKey,
) =>
  new URL(
    `${window.location.origin}/edit/#/${formId}/${clientKeyId}/${encodeBase64Url(secretLinkKey)}`,
  );

export const CONTACT_METHODS: ReadonlyArray<string> = [
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
];

export const TITLE_LEADS: ReadonlyArray<string> = [
  "Can I mobilize my community?",
  "Can I enact lasting change?",
  "Can I make a real difference?",
  "Can I start a movement?",
  "Can I change the world?",
];

const pickRandom = <T>(arr: ReadonlyArray<T>) => arr[Math.floor(Math.random() * arr.length)];

export const randomTitleLead = () => pickRandom(TITLE_LEADS);
