export const encodeBase64 = (bytes: Uint8Array): string =>
  btoa(String.fromCharCode.apply(null, [...bytes]));

export const decodeBase64 = (base64: string): Uint8Array =>
  Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

export const encodeBase64Url = (bytes: Uint8Array): string =>
  encodeBase64(bytes).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");

export const decodeBase64Url = (base64Url: string): Uint8Array => {
  const padding = 4 - (base64Url.length % 4);
  const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/") + "=".repeat(padding);

  return decodeBase64(base64);
};

export const encodeUtf8 = (str: string): Uint8Array => new TextEncoder().encode(str);

export const decodeUtf8 = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);

export const deserializeDate = (date: string): Date => {
  const [year, month, day] = date.split("-").map(Number);
  return new Date(year, month - 1, day);
};

export const serializeDate = (date: Date): string =>
  `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;

export const datetimeToCsvFormat = (date: Date): string =>
  `${date.getUTCFullYear()}-${date.getUTCMonth() + 1}-${date.getUTCDate()} ${date.getUTCHours()}:${date.getUTCMinutes()}:${date.getUTCSeconds()}`;

const localeDateTimeFormat = new Intl.DateTimeFormat(undefined, {
  dateStyle: "medium",
  timeStyle: "short",
});

export const formatDateTime = (date: Date): string => localeDateTimeFormat.format(date);

const localeDateFormat = new Intl.DateTimeFormat(undefined, {
  dateStyle: "medium",
});

export const formatDate = (date: Date): string => localeDateFormat.format(date);
