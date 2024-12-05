export const encodeBase64 = (bytes: Uint8Array): string =>
  btoa(String.fromCharCode.apply(null, [...bytes]));

export const encodeBase64Url = (bytes: Uint8Array): string =>
  encodeBase64(bytes).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");

export const decodeBase64 = (base64: string): Uint8Array =>
  Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

export const encodeUtf8 = (str: string): Uint8Array => new TextEncoder().encode(str);
