import { jwtDecode } from "jwt-decode";
import {
  deriveKeys,
  signApiChallengeNonce,
  unwrapPrivatePrimaryKey,
  type ApiAccessToken,
  type ApiChallengeNonce,
  type ApiChallengeToken,
  type PrivatePrimaryKey,
  type PrivateSigningKey,
  type PublicPrimaryKey,
  type SecretLinkKey,
} from "./crypto";
import type { ClientKeyId, FormId } from "./types";
import api from "./api";
import { decodeBase64, decodeBase64Url } from "./encoding";
import { ref, watchEffect, type Ref } from "vue";
import { useRoute } from "vue-router";
import type { ContactMethodCode } from "./vars";

const extractNonce = (challengeToken: ApiChallengeToken): ApiChallengeNonce => {
  const { nonce } = jwtDecode<{ nonce: string }>(challengeToken);
  return decodeBase64(nonce) as ApiChallengeNonce;
};

export const getAccessToken = async (
  formId: FormId,
  clientKeyId: ClientKeyId,
  privateSigningKey: PrivateSigningKey,
): Promise<ApiAccessToken> => {
  const challenge = await api.getChallengeToken({ formId, clientKeyId });
  const nonce = extractNonce(challenge);
  const signature = await signApiChallengeNonce(nonce, privateSigningKey);
  return await api.postAccessToken({
    challenge,
    signature,
  });
};

export const useLink = () => {
  const formId = ref<FormId>();

  const route = useRoute();

  const [, formIdSegment] = route.hash.split("/");

  formId.value = formIdSegment as FormId;

  return { formId };
};

export const useSecretLink = () => {
  const formId = ref<FormId>();
  const clientKeyId = ref<ClientKeyId>();
  const secretLinkKey = ref<SecretLinkKey>();

  const route = useRoute();

  const [, formIdSegment, clientKeyIdSegment, secretLinkKeySegment] = route.hash.split("/");

  formId.value = formIdSegment as FormId;
  clientKeyId.value = clientKeyIdSegment as ClientKeyId;
  secretLinkKey.value = decodeBase64Url(secretLinkKeySegment) as SecretLinkKey;

  return { formId, clientKeyId, secretLinkKey };
};

export const useAccessToken = () => {
  const accessToken = ref<ApiAccessToken>();

  const { formId, clientKeyId, secretLinkKey } = useSecretLink();

  watchEffect(async () => {
    if (
      formId.value === undefined ||
      clientKeyId.value === undefined ||
      secretLinkKey.value === undefined
    ) {
      return;
    }

    const { privateSigningKey } = await deriveKeys(secretLinkKey.value);

    accessToken.value = await getAccessToken(formId.value, clientKeyId.value, privateSigningKey);
  });

  return { accessToken };
};

export const usePrivatePrimaryKey = (accessToken: Ref<ApiAccessToken | undefined>) => {
  const privatePrimaryKey = ref<PrivatePrimaryKey>();

  const { formId, clientKeyId, secretLinkKey } = useSecretLink();

  watchEffect(async () => {
    if (
      formId.value === undefined ||
      clientKeyId.value === undefined ||
      secretLinkKey.value === undefined ||
      accessToken.value === undefined
    ) {
      return;
    }

    const { secretWrappingKey } = await deriveKeys(secretLinkKey.value);

    const wrappedPrivatePrimaryKey = await api.getKey({
      formId: formId.value,
      clientKeyId: clientKeyId.value,
      accessToken: accessToken.value,
    });

    privatePrimaryKey.value = unwrapPrivatePrimaryKey(wrappedPrivatePrimaryKey, secretWrappingKey);
  });

  return { privatePrimaryKey };
};

export const useForm = () => {
  const orgName = ref<string>();
  const description = ref<string>();
  const contactMethods = ref<Array<ContactMethodCode>>();
  const publicPrimaryKey = ref<PublicPrimaryKey>();

  const { formId } = useLink();

  watchEffect(async () => {
    if (formId.value === undefined) {
      return;
    }

    const response = await api.getForm({ formId: formId.value });

    orgName.value = response.orgName;
    description.value = response.description;
    contactMethods.value = response.contactMethods;
    publicPrimaryKey.value = response.publicPrimaryKey;
  });

  return { orgName, description, contactMethods, publicPrimaryKey };
};
