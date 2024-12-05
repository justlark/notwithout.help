export type Newtype<T, S> = T & {
  readonly __tag: S;
};

export type FormId = Newtype<string, { readonly __tag: unique symbol }>;
export type SecretLinkKey = Newtype<string, { readonly __tag: unique symbol }>;
export type ClientKeyId = Newtype<string, { readonly __tag: unique symbol }>;

export type AtLeastOne<T> = [T, ...Array<T>];
