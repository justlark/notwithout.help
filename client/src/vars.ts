import type { AtLeastOne } from "./types";

export const TOAST_TTL = 2000;

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

export const CONTACT_METHOD_TYPES: AtLeastOne<(typeof CONTACT_METHODS)[number]["code"]> = [
  CONTACT_METHODS[0].code,
  ...CONTACT_METHODS.slice(1).map((method) => method.code),
];

export type ContactMethodCode = (typeof CONTACT_METHOD_TYPES)[number];
