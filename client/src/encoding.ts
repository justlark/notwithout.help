import type { SecretLinkKey } from "./crypto";
import type { ClientKeyId, FormId } from "./types";

export const encodeBase64 = (bytes: Uint8Array): string =>
  btoa(String.fromCharCode.apply(null, [...bytes]));

export const encodeBase64Url = (bytes: Uint8Array): string =>
  encodeBase64(bytes).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");

export const decodeBase64 = (base64: string): Uint8Array =>
  Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

export const encodeUtf8 = (str: string): Uint8Array => new TextEncoder().encode(str);

export const deserializeDate = (date: string): Date => {
  const [year, month, day] = date.split("-").map(Number);
  return new Date(year, month - 1, day);
};

export const serializeDate = (date: Date): string =>
  `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`;

const localeDateTimeFormat = new Intl.DateTimeFormat(undefined, {
  dateStyle: "medium",
  timeStyle: "short",
});

export const formatDateTime = (date: Date): string => localeDateTimeFormat.format(date);

export const parseShareLinkFragment = (fragment: string): { formId: FormId } => {
  const [, formId] = fragment.split("/");

  return {
    formId: formId as FormId,
  };
};

export const parseSecretLinkFragment = (
  fragment: string,
): { formId: FormId; clientKeyId: ClientKeyId; secretLinkKey: SecretLinkKey } => {
  const [, formId, clientKeyId, secretLinkKey] = fragment.split("/");

  return {
    formId: formId as FormId,
    clientKeyId: clientKeyId as ClientKeyId,
    secretLinkKey: decodeBase64(secretLinkKey) as SecretLinkKey,
  };
};
