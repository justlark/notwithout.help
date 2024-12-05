import { jwtDecode } from "jwt-decode";
import {
  signApiChallengeNonce,
  type ApiAccessToken,
  type ApiChallengeNonce,
  type ApiChallengeToken,
  type PrivateSigningKey,
} from "./crypto";
import type { ClientKeyId, FormId } from "./types";
import api from "./api";
import { decodeBase64 } from "./encoding";

const extractNonce = (challengeToken: ApiChallengeToken): ApiChallengeNonce => {
  const { nonce } = jwtDecode<{ nonce: string }>(challengeToken);
  return decodeBase64(nonce) as ApiChallengeNonce;
};

export const getAccessToken = async (
  formId: FormId,
  clientKeyId: ClientKeyId,
  privateSigningKey: PrivateSigningKey,
): Promise<ApiAccessToken> => {
  const challenge = await api.getChallengeToken(formId, clientKeyId);
  const nonce = extractNonce(challenge);
  const signature = await signApiChallengeNonce(nonce, privateSigningKey);
  return await api.postAccessToken({
    challenge,
    signature,
  });
};
