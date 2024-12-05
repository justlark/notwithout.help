import { decodeBase64, encodeBase64, type PublicPrimaryKey, type PublicSigningKey } from "./crypto";
import type { ClientKeyId, FormId } from "./types";
import type { ContactMethodCode } from "./vars";

const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8787";

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

export const postForm = async (request: PostFormRequest): Promise<PostFormResponse> => {
  const requestBody = {
    public_primary_key: encodeBase64(request.publicPrimaryKey),
    public_signing_key: encodeBase64(request.publicSigningKey),
    org_name: request.orgName,
    description: request.description,
    contact_methods: request.contactMethods,
  };

  const response = await fetch(`${API_URL}/forms`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  const responseBody = await response.json();

  return {
    formId: responseBody.form_id as FormId,
    clientKeyId: responseBody.client_key_id as ClientKeyId,
  };
};

export default {
  postForm,
};
