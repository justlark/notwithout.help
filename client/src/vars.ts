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

export const contactMethodByCode = (code: string) =>
  CONTACT_METHODS.find((method) => method.code === code)?.name ?? "Other";

export type ContactMethodCode = (typeof CONTACT_METHOD_TYPES)[number];

export const TITLE_LEADS = [
  "How can I get folks organized?",
  "How can I enact lasting change?",
  "How can I make a real difference?",
  "How can I start a movement?",
  "How can I start a campaign?",
];

const pickRandom = <T>(arr: T[]) => arr[Math.floor(Math.random() * arr.length)];

export const randomTitleLead = () => pickRandom(TITLE_LEADS);
