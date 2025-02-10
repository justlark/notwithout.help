export const encodeBase64 = (bytes: Uint8Array): string =>
  btoa(String.fromCharCode.apply(null, [...bytes]));

export const decodeBase64 = (base64: string): Uint8Array =>
  Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

export const encodeBase64Url = (bytes: Uint8Array): string =>
  encodeBase64(bytes).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");

export const decodeBase64Url = (base64Url: string): Uint8Array => {
  const padding = (4 - (base64Url.length % 4)) % 4;
  const base64 = base64Url.replace(/-/g, "+").replace(/_/g, "/") + "=".repeat(padding);

  return decodeBase64(base64);
};

export const encodeUtf8 = (str: string): Uint8Array => new TextEncoder().encode(str);

export const decodeUtf8 = (bytes: Uint8Array): string => new TextDecoder().decode(bytes);

export const deserializeDate = (date: string): Date => {
  const [year, month, day] = date.split("-").map(Number);
  return new Date(year, month - 1, day);
};

export const serializeDate = (date: Date): string => {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, "0");
  const day = String(date.getDate()).padStart(2, "0");

  return `${year}-${month}-${day}`;
};

export const datetimeToCsvFormat = (date: Date): string => {
  const year = date.getUTCFullYear();
  const month = String(date.getUTCMonth() + 1).padStart(2, "0");
  const day = String(date.getUTCDate()).padStart(2, "0");
  const hour = String(date.getUTCHours()).padStart(2, "0");
  const minute = String(date.getUTCMinutes()).padStart(2, "0");
  const second = String(date.getUTCSeconds()).padStart(2, "0");

  return `${year}-${month}-${day} ${hour}:${minute}:${second}`;
};

const localeDateTimeFormat = new Intl.DateTimeFormat(undefined, {
  dateStyle: "medium",
  timeStyle: "short",
});

export const formatDateTime = (date: Date): string => localeDateTimeFormat.format(date);

const localeDateFormat = new Intl.DateTimeFormat(undefined, {
  dateStyle: "medium",
});

export const formatDate = (date: Date): string => localeDateFormat.format(date);
