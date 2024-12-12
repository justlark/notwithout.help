import type { ClientKeyId, FormId, SecretLinkKey } from "./crypto";
import { encodeBase64Url } from "./encoding";
import type { AtLeastOne } from "./types";

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
  { name: "Email", code: "email" },
  { name: "SMS", code: "sms" },
  { name: "Signal", code: "signal" },
  { name: "Telegram", code: "telegram" },
  { name: "Discord", code: "discord" },
  { name: "Mastodon", code: "mastodon" },
  { name: "Bluesky", code: "bluesky" },
  { name: "Threads", code: "threads" },
  { name: "Twitter", code: "twitter" },
  { name: "Matrix", code: "matrix" },
  { name: "WhatsApp", code: "whatsapp" },
  { name: "Instagram", code: "instagram" },
  { name: "Facebook", code: "facebook" },
  { name: "Snapchat", code: "snapchat" },
  { name: "WeChat", code: "wechat" },
  { name: "Other", code: "other" },
] as const;

export const CONTACT_METHOD_CODES: AtLeastOne<(typeof CONTACT_METHODS)[number]["code"]> = [
  CONTACT_METHODS[0].code,
  ...CONTACT_METHODS.slice(1).map((method) => method.code),
];

export type ContactMethodCode = (typeof CONTACT_METHOD_CODES)[number];

export const contactMethodByCode = (code: ContactMethodCode) =>
  CONTACT_METHODS.find((method) => method.code === code);

export const TITLE_LEADS = [
  "Can I mobilize my community?",
  "Can I enact lasting change?",
  "Can I make a real difference?",
  "Can I start a movement?",
  "Can I change the world?",
];

const pickRandom = <T>(arr: T[]) => arr[Math.floor(Math.random() * arr.length)];

export const randomTitleLead = () => pickRandom(TITLE_LEADS);
