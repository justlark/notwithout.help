import {
  encodeBase64,
  type ApiAccessToken,
  type ApiChallengeSignature,
  type ApiChallengeToken,
  type EncryptedKeyComment,
  type PublicPrimaryKey,
  type PublicSigningKey,
  type WrappedPrivatePrimaryKey,
} from "./crypto";
import type { ClientKeyId, FormId } from "./types";
import type { ContactMethodCode } from "./vars";

//
// TODO: Handle errors from the API.
//

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

  const { form_id, client_key_id } = await response.json();

  return {
    formId: form_id as FormId,
    clientKeyId: client_key_id as ClientKeyId,
  };
};

export const getChallengeToken = async (
  formId: FormId,
  clientKeyId: ClientKeyId,
): Promise<ApiChallengeToken> => {
  const response = await fetch(`${API_URL}/challenges/${formId}/${clientKeyId}`, { method: "GET" });

  const { challenge } = await response.json();

  return challenge as ApiChallengeToken;
};

export interface PatchKeyRequest {
  formId: FormId;
  clientKeyId: ClientKeyId;
  wrappedPrivatePrimaryKey: WrappedPrivatePrimaryKey;
  encryptedComment: EncryptedKeyComment;
  accessToken: ApiAccessToken;
}

export const patchKey = async (request: PatchKeyRequest) => {
  const requestBody = {
    wrapped_private_primary_key: encodeBase64(request.wrappedPrivatePrimaryKey),
    encrypted_comment: encodeBase64(request.encryptedComment),
  };

  await fetch(`${API_URL}/keys/${request.formId}/${request.clientKeyId}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${request.accessToken}`,
    },
    body: JSON.stringify(requestBody),
  });
};

export interface PostAccessTokenRequest {
  signature: ApiChallengeSignature;
  challenge: ApiChallengeToken;
}

export const postAccessToken = async (request: PostAccessTokenRequest) => {
  const requestBody = {
    signature: encodeBase64(request.signature),
    challenge: request.challenge,
  };

  const response = await fetch(`${API_URL}/tokens`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  const { token } = await response.json();

  return token as ApiAccessToken;
};

export default {
  postForm,
  getChallengeToken,
  patchKey,
  postAccessToken,
};
