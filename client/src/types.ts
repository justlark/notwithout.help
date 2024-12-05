export type Newtype<T> = T & { readonly __tag: unique symbol };

export type FormId = Newtype<string>;
export type SecretLinkKey = Newtype<string>;
export type ClientKeyId = Newtype<string>;

export type AtLeastOne<T> = [T, ...Array<T>];
