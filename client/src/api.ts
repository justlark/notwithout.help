import type { PublicPrimaryKey, PublicSigningKey } from "./crypto";
import type { ClientKeyId, FormId } from "./types";
import type { ContactMethodCode } from "./vars";

const API_URL = import.meta.env.VITE_API_URL ?? "/api";

const toSnakeCase = (key: string): string => {
  return key.replace(/[A-Z]/g, (letter) => `_${letter.toLowerCase()}`);
};

const toCamelCase = (key: string): string => {
  return key.replace(/_([a-z])/g, (_, letter) => letter.toUpperCase());
};

const serializeJson = (obj: Record<string, any>): string => {
  return JSON.stringify(obj, (key, value) => {
    // Root object key is an empty string.
    if (key === "") return value;
    const transformedKey = toSnakeCase(key);
    return { [transformedKey]: value };
  });
};

const deserializeJson = <T>(jsonString: string): T => {
  return JSON.parse(jsonString, (_, value) => {
    if (typeof value === "object" && value !== null && !Array.isArray(value)) {
      return Object.entries(value).reduce<Record<string, any>>((obj, [k, v]) => {
        obj[toCamelCase(k)] = v;
        return obj;
      }, {});
    }

    // For non-objects or arrays, return the value as is.
    return value;
  });
};

export interface PostFormRequest {
  publicPrimaryKey: PublicPrimaryKey;
  publicSigningKey: PublicSigningKey;
  orgName: string;
  description: string;
  contactMethods: Array<ContactMethodCode>;
}

export interface PostFormResponse {
  formId: FormId;
  clientKeyId: ClientKeyId;
}

const postForm = async (request: PostFormRequest) => {
  const response = await fetch(`${API_URL}/form`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: serializeJson(request),
  });

  return deserializeJson<PostFormResponse>(await response.json());
};
