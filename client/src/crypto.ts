import _sodium from "libsodium-wrappers";
import * as ed from "@noble/ed25519";
import type { Newtype } from "./types";

await _sodium.ready;
export const sodium = _sodium;

export const encodeBase64 = (bytes: Uint8Array): string =>
  btoa(String.fromCharCode.apply(null, [...bytes]));

export const encodeBase64Url = (bytes: Uint8Array): string =>
  encodeBase64(bytes).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");

export const decodeBase64 = (base64: string): Uint8Array =>
  Uint8Array.from(atob(base64), (c) => c.charCodeAt(0));

export type PrivatePrimaryKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type PublicPrimaryKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type SecretLinkKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type SecretWrappingKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type WrappedPrivatePrimaryKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type PrivateSigningKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type PublicSigningKey = Newtype<Uint8Array, { readonly __tag: unique symbol }>;

export type EncryptedSubmissionBody = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type EncryptedKeyComment = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type ApiChallengeNonce = Newtype<Uint8Array, { readonly __tag: unique symbol }>;
export type ApiChallengeSignature = Newtype<Uint8Array, { readonly __tag: unique symbol }>;

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

const SECRET_WRAPPING_KEY_PARAMS: DerivedKeyParams = {
  index: 1,
  context: "nwh-wrap",
  len: sodium["crypto_secretbox_KEYBYTES"],
};

const PRIVATE_SIGNING_KEY_PARAMS: DerivedKeyParams = {
  index: 2,
  context: "nwh-sign",
  len: 32,
};

export const generateSecretLinkKey = (): SecretLinkKey =>
  sodium.crypto_kdf_keygen() as SecretLinkKey;

export const generatePrimaryKeypair = (): PrimaryKeypair => {
  const { privateKey, publicKey } = sodium.crypto_box_keypair();

  return {
    private: privateKey as PrivatePrimaryKey,
    public: publicKey as PublicPrimaryKey,
  };
};

export const deriveKeys = async (secretLinkKey: SecretLinkKey): Promise<DerivedKeys> => {
  const secretWrappingKey = sodium.crypto_kdf_derive_from_key(
    SECRET_WRAPPING_KEY_PARAMS.len,
    SECRET_WRAPPING_KEY_PARAMS.index,
    SECRET_WRAPPING_KEY_PARAMS.context,
    secretLinkKey,
  ) as SecretWrappingKey;

  const privateSigningKey = sodium.crypto_kdf_derive_from_key(
    PRIVATE_SIGNING_KEY_PARAMS.len,
    PRIVATE_SIGNING_KEY_PARAMS.index,
    PRIVATE_SIGNING_KEY_PARAMS.context,
    secretLinkKey,
  ) as PrivateSigningKey;

  const publicSigningKey = (await ed.getPublicKeyAsync(privateSigningKey)) as PublicSigningKey;

  return {
    secretWrappingKey,
    privateSigningKey,
    publicSigningKey,
  };
};

const encryptSecretbox = (
  plaintext: Uint8Array,
  secretWrappingKey: SecretWrappingKey,
): Uint8Array => {
  const nonce = sodium.randombytes_buf(sodium["crypto_secretbox_NONCEBYTES"]);
  const ciphertext = sodium.crypto_secretbox_easy(plaintext, nonce, secretWrappingKey);
  return new Uint8Array([...nonce, ...ciphertext]);
};

export const decryptSecretbox = (
  ciphertext: Uint8Array,
  secretWrappingKey: SecretWrappingKey,
): Uint8Array => {
  const nonce = ciphertext.slice(0, sodium["crypto_secretbox_NONCEBYTES"]);
  const message = ciphertext.slice(sodium["crypto_secretbox_NONCEBYTES"]);

  return sodium.crypto_secretbox_open_easy(message, nonce, secretWrappingKey);
};

const sealBox = (message: Uint8Array, publicPrimaryKey: PublicPrimaryKey): Uint8Array =>
  sodium.crypto_box_seal(message, publicPrimaryKey);

const unsealBox = (
  ciphertext: Uint8Array,
  publicPrimaryKey: PublicPrimaryKey,
  privatePrimaryKey: PrivatePrimaryKey,
): Uint8Array => sodium.crypto_box_seal_open(ciphertext, publicPrimaryKey, privatePrimaryKey);

const sign = async (
  message: Uint8Array,
  privateSigningKey: PrivateSigningKey,
): Promise<Uint8Array> => ed.signAsync(message, privateSigningKey);

export const wrapPrivatePrimaryKey = (
  privatePrimaryKey: PrivatePrimaryKey,
  secretWrappingKey: SecretWrappingKey,
): WrappedPrivatePrimaryKey =>
  encryptSecretbox(privatePrimaryKey, secretWrappingKey) as WrappedPrivatePrimaryKey;

export const unwrapPrivatePrimaryKey = (
  wrappedPrivatePrimaryKey: WrappedPrivatePrimaryKey,
  secretWrappingKey: SecretWrappingKey,
): PrivatePrimaryKey =>
  decryptSecretbox(wrappedPrivatePrimaryKey, secretWrappingKey) as PrivatePrimaryKey;

export const encryptKeyComment = (
  comment: Uint8Array,
  secretWrappingKey: SecretWrappingKey,
): EncryptedKeyComment => encryptSecretbox(comment, secretWrappingKey) as EncryptedKeyComment;

export const decryptKeyComment = (
  comment: EncryptedKeyComment,
  secretWrappingKey: SecretWrappingKey,
): Uint8Array => decryptSecretbox(comment, secretWrappingKey);

export const sealSubmissionBody = (
  body: Uint8Array,
  publicPrimaryKey: PublicPrimaryKey,
): EncryptedSubmissionBody => sealBox(body, publicPrimaryKey) as EncryptedSubmissionBody;

export const unsealSubmissionBody = (
  body: EncryptedSubmissionBody,
  publicPrimaryKey: PublicPrimaryKey,
  privatePrimaryKey: PrivatePrimaryKey,
): Uint8Array => unsealBox(body, publicPrimaryKey, privatePrimaryKey);

export const signApiChallengeNonce = (
  nonce: ApiChallengeNonce,
  privateSigningKey: PrivateSigningKey,
): ApiChallengeSignature => sign(nonce, privateSigningKey) as ApiChallengeSignature;
