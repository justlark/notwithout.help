import _sodium from "libsodium-wrappers";
import * as ed from "@noble/ed25519";
import type { Newtype } from "./types";

let isSodiumReady = false;

const getSodium = async () => {
  if (!isSodiumReady) {
    await _sodium.ready;
    isSodiumReady = true;
  }

  return _sodium;
};

export type FormId = Newtype<string, { readonly __tag: unique symbol }>;
export type ClientKeyId = Newtype<string, { readonly __tag: unique symbol }>;

export type ApiAccessToken = Newtype<string, { readonly __tag: unique symbol }>;
export type ApiChallengeToken = Newtype<string, { readonly __tag: unique symbol }>;
export type ApiChallengeNonce = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type ApiChallengeSignature = Newtype<Uint8Array, { readonly __tag: unique symbol }>;

export type PrivatePrimaryKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type PublicPrimaryKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type SecretLinkKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type SecretWrappingKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type WrappedPrivatePrimaryKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type PrivateSigningKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type PublicSigningKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;

export type EncryptedSubmissionBody = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type EncryptedKeyComment = Newtype<Uint8Array, { readonly __tag: unique symbol }>;

export type PrimaryKeypair = {
  private: PrivatePrimaryKey;
  public: PublicPrimaryKey;
};

export type DerivedKeys = {
  secretWrappingKey: SecretWrappingKey;
  privateSigningKey: PrivateSigningKey;
  publicSigningKey: PublicSigningKey;
};

type DerivedKeyParams = {
  index: number;
  context: string;
  len: number;
};

const secretWrappingKeyParams = async (): Promise<DerivedKeyParams> => {
  const sodium = await getSodium();

  return {
    index: 1,
    context: "nwh-wrap",
    len: sodium["crypto_secretbox_KEYBYTES"],
  };
};

const privateSigningKeyParams = (): DerivedKeyParams => ({
  index: 2,
  context: "nwh-sign",
  len: 32,
});

export const generateSecretLinkKey = async (): Promise<SecretLinkKey> => {
  const sodium = await getSodium();
  return sodium.crypto_kdf_keygen() as SecretLinkKey;
};

export const generatePrimaryKeypair = async (): Promise<PrimaryKeypair> => {
  const sodium = await getSodium();

  const { privateKey, publicKey } = sodium.crypto_box_keypair();

  return {
    private: privateKey as PrivatePrimaryKey,
    public: publicKey as PublicPrimaryKey,
  };
};

export const deriveKeys = async (secretLinkKey: SecretLinkKey): Promise<DerivedKeys> => {
  const sodium = await getSodium();

  const wrappingkeyParams = await secretWrappingKeyParams();
  const signingKeyParams = privateSigningKeyParams();

  const secretWrappingKey = sodium.crypto_kdf_derive_from_key(
    wrappingkeyParams.len,
    wrappingkeyParams.index,
    wrappingkeyParams.context,
    secretLinkKey,
  ) as SecretWrappingKey;

  const privateSigningKey = sodium.crypto_kdf_derive_from_key(
    signingKeyParams.len,
    signingKeyParams.index,
    signingKeyParams.context,
    secretLinkKey,
  ) as PrivateSigningKey;

  const publicSigningKey = (await ed.getPublicKeyAsync(privateSigningKey)) as PublicSigningKey;

  return {
    secretWrappingKey,
    privateSigningKey,
    publicSigningKey,
  };
};

const encryptSecretbox = async (
  plaintext: Uint8Array,
  secretWrappingKey: SecretWrappingKey,
): Promise<Uint8Array> => {
  const sodium = await getSodium();

  const nonce = sodium.randombytes_buf(sodium["crypto_secretbox_NONCEBYTES"]);
  const ciphertext = sodium.crypto_secretbox_easy(plaintext, nonce, secretWrappingKey);
  return new Uint8Array([...nonce, ...ciphertext]);
};

export const decryptSecretbox = async (
  ciphertext: Uint8Array,
  secretWrappingKey: SecretWrappingKey,
): Promise<Uint8Array> => {
  const sodium = await getSodium();

  const nonce = ciphertext.slice(0, sodium["crypto_secretbox_NONCEBYTES"]);
  const message = ciphertext.slice(sodium["crypto_secretbox_NONCEBYTES"]);

  return sodium.crypto_secretbox_open_easy(message, nonce, secretWrappingKey);
};

const sealBox = async (
  message: Uint8Array,
  publicPrimaryKey: PublicPrimaryKey,
): Promise<Uint8Array> => {
  const sodium = await getSodium();
  return sodium.crypto_box_seal(message, publicPrimaryKey);
};

const unsealBox = async (
  ciphertext: Uint8Array,
  publicPrimaryKey: PublicPrimaryKey,
  privatePrimaryKey: PrivatePrimaryKey,
): Promise<Uint8Array> => {
  const sodium = await getSodium();
  return sodium.crypto_box_seal_open(ciphertext, publicPrimaryKey, privatePrimaryKey);
};

const sign = async (
  message: Uint8Array,
  privateSigningKey: PrivateSigningKey,
): Promise<Uint8Array> => ed.signAsync(message, privateSigningKey);

export const wrapPrivatePrimaryKey = async (
  privatePrimaryKey: PrivatePrimaryKey,
  secretWrappingKey: SecretWrappingKey,
): Promise<WrappedPrivatePrimaryKey> =>
  (await encryptSecretbox(privatePrimaryKey, secretWrappingKey)) as WrappedPrivatePrimaryKey;

export const unwrapPrivatePrimaryKey = async (
  wrappedPrivatePrimaryKey: WrappedPrivatePrimaryKey,
  secretWrappingKey: SecretWrappingKey,
): Promise<PrivatePrimaryKey> =>
  (await decryptSecretbox(wrappedPrivatePrimaryKey, secretWrappingKey)) as PrivatePrimaryKey;

export const sealKeyComment = async (
  comment: Uint8Array,
  publicPrimaryKey: PublicPrimaryKey,
): Promise<EncryptedKeyComment> =>
  (await sealBox(comment, publicPrimaryKey)) as EncryptedKeyComment;

export const unsealKeyComment = async (
  comment: EncryptedKeyComment,
  publicPrimaryKey: PublicPrimaryKey,
  privatePrimaryKey: PrivatePrimaryKey,
): Promise<Uint8Array> =>
  (await unsealBox(comment, publicPrimaryKey, privatePrimaryKey)) as Uint8Array;

export const sealSubmissionBody = async (
  body: Uint8Array,
  publicPrimaryKey: PublicPrimaryKey,
): Promise<EncryptedSubmissionBody> =>
  (await sealBox(body, publicPrimaryKey)) as EncryptedSubmissionBody;

export const unsealSubmissionBody = async (
  body: EncryptedSubmissionBody,
  publicPrimaryKey: PublicPrimaryKey,
  privatePrimaryKey: PrivatePrimaryKey,
): Promise<Uint8Array> => await unsealBox(body, publicPrimaryKey, privatePrimaryKey);

export const signApiChallengeNonce = async (
  nonce: ApiChallengeNonce,
  privateSigningKey: PrivateSigningKey,
): Promise<ApiChallengeSignature> =>
  (await sign(nonce, privateSigningKey)) as ApiChallengeSignature;
