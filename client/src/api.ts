import {
  type ApiAccessToken,
  type ApiChallengeSignature,
  type ApiChallengeToken,
  type ClientKeyId,
  type EncryptedKeyComment,
  type EncryptedSubmissionBody,
  type FormId,
  type PublicPrimaryKey,
  type PublicSigningKey,
  type SecretLinkPasswordNonce,
  type SecretLinkPasswordSalt,
  type WrappedPrivatePrimaryKey,
} from "./crypto";
import { decodeBase64, encodeBase64 } from "./encoding";

//
// TODO: Handle errors from the API.
//

const API_URL = import.meta.env.VITE_API_URL ?? "http://localhost:8787";

export type ApiErrorKind =
  | "bad-request"
  | "not-found"
  | "unauthorized"
  | "forbidden"
  | "content-too-large"
  | "unexpected";

export class ApiError extends Error {
  public readonly kind: ApiErrorKind;

  constructor(response: Response) {
    super(`HTTP error ${response.status}`);

    if (response.status === 400) {
      this.kind = "bad-request";
    } else if (response.status === 404) {
      this.kind = "not-found";
    } else if (response.status === 401) {
      this.kind = "unauthorized";
    } else if (response.status === 403) {
      this.kind = "forbidden";
    } else if (response.status === 413) {
      this.kind = "content-too-large";
    } else {
      this.kind = "unexpected";
    }
  }
}

export type AccessRole = "read" | "admin";

// We version the submission body so we can change the shape of the object in
// the future while preserving backwards compatibility. This is particularly
// important because the submission body is encrypted client-side, so we can't
// implement any sort of migrations.
export const CURRENT_VERSION = 1;

// This is the shape of the object that's serialized, sealed, and sent to the
// server.
export interface SubmissionBody {
  version: 1;
  name: string;
  contact: string;
  contact_method: string;
  roles: Array<string> | undefined;
  comment: string | undefined;
}

// Not to be confused with an "access role."
export interface OrgRole {
  id: string;
  name: string;
  details: Array<string>;
}

export interface GetFormParams {
  formId: FormId;
}

export interface GetFormResponse {
  orgName: string;
  description: string;
  contactMethods: Array<string>;
  publicPrimaryKey: PublicPrimaryKey;
  expirationDate: Date | undefined;
  roles: Array<OrgRole> | undefined;
}

const getForm = async ({ formId }: GetFormParams): Promise<GetFormResponse> => {
  const response = await fetch(`${API_URL}/forms/${formId}`);

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { org_name, description, contact_methods, public_primary_key, expires_at, roles } =
    await response.json();

  return {
    orgName: org_name,
    description,
    contactMethods: contact_methods,
    publicPrimaryKey: decodeBase64(public_primary_key) as PublicPrimaryKey,
    expirationDate: expires_at ? new Date(expires_at) : undefined,
    roles,
  };
};

export interface PostFormParams {
  publicPrimaryKey: PublicPrimaryKey;
  publicSigningKey: PublicSigningKey;
  orgName: string;
  description: string;
  contactMethods: Array<string>;
  expirationDate: Date | undefined;
  roles: Array<OrgRole>;
}

export interface PostFormResponse {
  formId: FormId;
  clientKeyId: ClientKeyId;
}

const postForm = async ({
  publicPrimaryKey,
  publicSigningKey,
  orgName,
  description,
  contactMethods,
  expirationDate,
  roles,
}: PostFormParams): Promise<PostFormResponse> => {
  const requestBody = {
    public_primary_key: encodeBase64(publicPrimaryKey),
    public_signing_key: encodeBase64(publicSigningKey),
    org_name: orgName,
    description: description,
    contact_methods: contactMethods,
    expires_at: expirationDate?.toISOString(),
    roles: roles,
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

const deleteForm = async ({ formId, accessToken }: DeleteFormParams) => {
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

export interface PatchFormParams {
  formId: FormId;
  orgName: string;
  description: string;
  contactMethods: Array<string>;
  expirationDate: Date | undefined;
  roles: Array<OrgRole>;
  accessToken: ApiAccessToken;
}

const patchForm = async ({
  formId,
  orgName,
  description,
  contactMethods,
  expirationDate,
  accessToken,
  roles,
}: PatchFormParams) => {
  const requestBody = {
    org_name: orgName,
    description: description,
    contact_methods: contactMethods,
    expires_at: expirationDate?.toISOString(),
    roles: roles,
  };

  const response = await fetch(`${API_URL}/forms/${formId}`, {
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

export interface GetChallengeTokenParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
}

const getChallengeToken = async ({
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

const patchKey = async ({
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

export interface PostKeyParams {
  formId: FormId;
  publicSigningKey: PublicSigningKey;
  wrappedPrivatePrimaryKey: WrappedPrivatePrimaryKey;
  encryptedComment: EncryptedKeyComment;
  role: AccessRole;
  accessToken: ApiAccessToken;
}

export interface PostKeyResponse {
  clientKeyId: ClientKeyId;
}

const postKey = async ({
  formId,
  publicSigningKey,
  wrappedPrivatePrimaryKey,
  encryptedComment,
  role,
  accessToken,
}: PostKeyParams) => {
  const requestBody = {
    public_signing_key: encodeBase64(publicSigningKey),
    wrapped_private_primary_key: encodeBase64(wrappedPrivatePrimaryKey),
    encrypted_comment: encodeBase64(encryptedComment),
    role: role,
  };

  const response = await fetch(`${API_URL}/keys/${formId}`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${accessToken}`,
    },
    body: JSON.stringify(requestBody),
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { client_key_id } = await response.json();

  return {
    clientKeyId: client_key_id as ClientKeyId,
  };
};

export interface GetKeyParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
  accessToken: ApiAccessToken;
}

const getKey = async ({
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

export interface ListKeysParams {
  formId: FormId;
  accessToken: ApiAccessToken;
}

export interface ListKeysResponse {
  clientKeyId: ClientKeyId;
  encryptedComment: EncryptedKeyComment;
  role: AccessRole;
  accessedAt: Date | undefined;
}

const listKeys = async ({
  formId,
  accessToken,
}: ListKeysParams): Promise<Array<ListKeysResponse>> => {
  const response = await fetch(`${API_URL}/keys/${formId}`, {
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    throw new ApiError(response);
  }

  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  const keys: Array<any> = await response.json();

  return keys.map(({ client_key_id, encrypted_comment, role, accessed_at }) => ({
    clientKeyId: client_key_id,
    encryptedComment: decodeBase64(encrypted_comment) as EncryptedKeyComment,
    role: role,
    accessedAt: accessed_at ? new Date(accessed_at) : undefined,
  }));
};

export interface DeleteKeyParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
  accessToken: ApiAccessToken;
}

const deleteKey = async ({ formId, clientKeyId, accessToken }: DeleteKeyParams) => {
  const response = await fetch(`${API_URL}/keys/${formId}/${clientKeyId}`, {
    method: "DELETE",
    headers: {
      Authorization: `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    throw new ApiError(response);
  }
};

export interface PostAccessTokenParams {
  signature: ApiChallengeSignature;
  challenge: ApiChallengeToken;
}

const postAccessToken = async ({ signature, challenge }: PostAccessTokenParams) => {
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

const postSubmission = async ({ formId, encryptedBody }: PostSubmissionParams) => {
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

const getSubmissions = async ({
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

export interface GetPasswordParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
}

export interface GetPasswordResponse {
  salt: SecretLinkPasswordSalt;
  nonce: SecretLinkPasswordNonce;
}

const getPassword = async ({ formId, clientKeyId }: GetPasswordParams) => {
  const response = await fetch(`${API_URL}/passwords/${formId}/${clientKeyId}`);

  if (!response.ok) {
    throw new ApiError(response);
  }

  const { salt, nonce } = await response.json();

  return {
    salt: decodeBase64(salt) as SecretLinkPasswordSalt,
    nonce: decodeBase64(nonce) as SecretLinkPasswordNonce,
  };
};

export interface PostPasswordParams {
  formId: FormId;
  clientKeyId: ClientKeyId;
  salt: SecretLinkPasswordSalt;
  nonce: SecretLinkPasswordNonce;
  accessToken: ApiAccessToken;
}

const postPassword = async ({
  formId,
  clientKeyId,
  salt,
  nonce,
  accessToken,
}: PostPasswordParams) => {
  const requestBody = {
    salt: encodeBase64(salt),
    nonce: encodeBase64(nonce),
  };

  const response = await fetch(`${API_URL}/passwords/${formId}/${clientKeyId}`, {
    method: "POST",
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

export default {
  getForm,
  postForm,
  deleteForm,
  patchForm,
  getChallengeToken,
  getKey,
  listKeys,
  patchKey,
  postKey,
  deleteKey,
  postAccessToken,
  postSubmission,
  getSubmissions,
  getPassword,
};
