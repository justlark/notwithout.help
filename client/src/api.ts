import {
  type ApiAccessToken,
  type ApiChallengeSignature,
  type ApiChallengeToken,
  type EncryptedKeyComment,
  type EncryptedSubmissionBody,
  type PublicPrimaryKey,
  type PublicSigningKey,
  type WrappedPrivatePrimaryKey,
} from "./crypto";
import { decodeBase64, encodeBase64 } from "./encoding";
import type { ClientKeyId, FormId } from "./types";
import type { ContactMethodCode } from "./vars";

//
// TODO: Handle errors from the API.
//

const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8787";

export type ApiErrorKind = "not-found" | "unauthorized" | "content-too-large" | "unexpected";

export class ApiError extends Error {
  public readonly kind: ApiErrorKind;

  constructor(response: Response) {
    super(`HTTP error ${response.status}`);

    if (response.status === 404) {
      this.kind = "not-found";
    } else if (response.status === 401) {
      this.kind = "unauthorized";
    } else if (response.status === 413) {
      this.kind = "content-too-large";
    } else {
      this.kind = "unexpected";
    }
  }
}

// This is the shape of the object that's serialized, sealed, and sent to the
// server.
export interface SubmissionBody {
  name: string;
  contact: string;
  contact_method: ContactMethodCode;
}

export interface GetFormParams {
  formId: FormId;
}

export interface GetFormResponse {
  orgName: string;
  description: string;
  contactMethods: Array<ContactMethodCode>;
  publicPrimaryKey: PublicPrimaryKey;
}

export const getForm = async ({ formId }: GetFormParams): Promise<GetFormResponse> => {
  const response = await fetch(`${API_URL}/forms/${formId}`);

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { org_name, description, contact_methods, public_primary_key } = await response.json();

  return {
    orgName: org_name,
    description,
    contactMethods: contact_methods,
    publicPrimaryKey: decodeBase64(public_primary_key) as PublicPrimaryKey,
  };
};

export interface PostFormParams {
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

export const postForm = async ({
  publicPrimaryKey,
  publicSigningKey,
  orgName,
  description,
  contactMethods,
}: PostFormParams): Promise<PostFormResponse> => {
  const requestBody = {
    public_primary_key: encodeBase64(publicPrimaryKey),
    public_signing_key: encodeBase64(publicSigningKey),
    org_name: orgName,
    description: description,
    contact_methods: contactMethods,
  };

  const response = await fetch(`${API_URL}/forms`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { form_id, client_key_id } = await response.json();

  return {
    formId: form_id as FormId,
    clientKeyId: client_key_id as ClientKeyId,
  };
};

export interface DeleteFormParams {
  formId: FormId;
  accessToken: ApiAccessToken;
}

export const deleteForm = async ({ formId, accessToken }: DeleteFormParams) => {
  const response = await fetch(`${API_URL}/forms/${formId}`, {
    method: "DELETE",
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    throw new ApiError(response);
  }
};

export interface GetChallengeTokenParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
}

export const getChallengeToken = async ({
  formId,
  clientKeyId,
}: GetChallengeTokenParams): Promise<ApiChallengeToken> => {
  const response = await fetch(`${API_URL}/challenges/${formId}/${clientKeyId}`, {
    method: "POST",
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { challenge } = await response.json();

  return challenge as ApiChallengeToken;
};

export interface PatchKeyParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
  wrappedPrivatePrimaryKey: WrappedPrivatePrimaryKey;
  encryptedComment: EncryptedKeyComment;
  accessToken: ApiAccessToken;
}

export const patchKey = async ({
  formId,
  clientKeyId,
  wrappedPrivatePrimaryKey,
  encryptedComment,
  accessToken,
}: PatchKeyParams) => {
  const requestBody = {
    wrapped_private_primary_key: encodeBase64(wrappedPrivatePrimaryKey),
    encrypted_comment: encodeBase64(encryptedComment),
  };

  const response = await fetch(`${API_URL}/keys/${formId}/${clientKeyId}`, {
    method: "PATCH",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    throw new ApiError(response);
  }
};

export interface GetKeyParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
  accessToken: ApiAccessToken;
}

export const getKey = async ({
  formId,
  clientKeyId,
  accessToken,
}: GetKeyParams): Promise<WrappedPrivatePrimaryKey> => {
  const response = await fetch(`${API_URL}/keys/${formId}/${clientKeyId}`, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { wrapped_private_primary_key } = await response.json();

  return decodeBase64(wrapped_private_primary_key) as WrappedPrivatePrimaryKey;
};

export interface PostAccessTokenParams {
  signature: ApiChallengeSignature;
  challenge: ApiChallengeToken;
}

export const postAccessToken = async ({ signature, challenge }: PostAccessTokenParams) => {
  const requestBody = {
    signature: encodeBase64(signature),
    challenge: challenge,
  };

  const response = await fetch(`${API_URL}/tokens`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { token } = await response.json();

  return token as ApiAccessToken;
};

export interface PostSubmissionParams {
  formId: FormId;
  encryptedBody: EncryptedSubmissionBody;
}

export const postSubmission = async ({ formId, encryptedBody }: PostSubmissionParams) => {
  const requestBody = {
    encrypted_body: encodeBase64(encryptedBody),
  };

  const response = await fetch(`${API_URL}/submissions/${formId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    throw new ApiError(response);
  }
};

export interface GetSubmissionsParams {
  formId: FormId;
  accessToken: ApiAccessToken;
}

export interface GetSubmissionsResponse {
  encryptedBody: EncryptedSubmissionBody;
  createdAt: Date;
}

export const getSubmissions = async ({
  formId,
  accessToken,
}: GetSubmissionsParams): Promise<Array<GetSubmissionsResponse>> => {
  const response = await fetch(`${API_URL}/submissions/${formId}`, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const submissions: Array<any> = await response.json();

  return submissions.map(({ encrypted_body, created_at }) => ({
    encryptedBody: decodeBase64(encrypted_body) as EncryptedSubmissionBody,
    createdAt: new Date(created_at),
  }));
};

export default {
  getForm,
  postForm,
  deleteForm,
  getChallengeToken,
  getKey,
  patchKey,
  postAccessToken,
  postSubmission,
  getSubmissions,
};
