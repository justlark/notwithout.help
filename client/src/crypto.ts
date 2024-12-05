import _sodium from "libsodium-wrappers";
import * as ed from "@noble/ed25519";
import type { Newtype } from "./types";

await _sodium.ready;
export const sodium = _sodium;

export type PrivatePrimaryKey = Newtype<Uint8Array>;
export type PublicPrimaryKey = Newtype<Uint8Array>;
export type SecretLinkKey = Newtype<Uint8Array>;
export type SecretWrappingKey = Newtype<Uint8Array>;
export type PrivateSigningKey = Newtype<Uint8Array>;
export type PublicSigningKey = Newtype<Uint8Array>;

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
};

const SECRET_WRAPPING_KEY_PARAMS: DerivedKeyParams = { index: 1, context: "nwh-wrap" };
const SIGNING_KEYS_SEED_KEY_PARAMS: DerivedKeyParams = { index: 2, context: "nwh-sign" };

export const generateSecretLinkKey = (): SecretLinkKey =>
  sodium.crypto_kdf_keygen() as SecretLinkKey;

export const generatePrimaryKeypair = (): PrimaryKeypair => {
  const { privateKey, publicKey } = sodium.crypto_box_keypair();

  return {
    private: privateKey as PrivatePrimaryKey,
    public: publicKey as PublicPrimaryKey,
  };
};

export function deriveKeys(secretLinkKey: SecretLinkKey): DerivedKeys {
  const secretWrappingKey = sodium.crypto_kdf_derive_from_key(
    sodium["crypto_secretbox_KEYBYTES"],
    SECRET_WRAPPING_KEY_PARAMS.index,
    SECRET_WRAPPING_KEY_PARAMS.context,
    secretLinkKey,
  );

  const signingKeysSeedKey = sodium.crypto_kdf_derive_from_key(
    sodium["crypto_sign_SEEDBYTES"],
    SIGNING_KEYS_SEED_KEY_PARAMS.index,
    SIGNING_KEYS_SEED_KEY_PARAMS.context,
    secretLinkKey,
  );

  const { publicKey: publicSigningKey, privateKey: privateSigningKey } =
    sodium.crypto_sign_seed_keypair(signingKeysSeedKey);

  return {
    secretWrappingKey: secretWrappingKey as SecretWrappingKey,
    privateSigningKey: privateSigningKey as PrivateSigningKey,
    publicSigningKey: publicSigningKey as PublicSigningKey,
  };
}
