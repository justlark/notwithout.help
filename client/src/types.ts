export type Newtype<T, S> = T & {
  readonly __tag: S;
};

export type FormId = Newtype<string, { readonly __tag: unique symbol }>;
export type ClientKeyId = Newtype<string, { readonly __tag: unique symbol }>;

export type AtLeastOne<T> = [T, ...Array<T>];

export type Loadable<T extends Record<string, any> = {}, E extends Error = Error> =
  | { state: "loading" }
  | { state: "done"; value: T }
  | { state: "error"; error: E };
