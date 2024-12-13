import { toRef, toValue, type MaybeRefOrGetter, type Ref } from "vue";

export type Newtype<T, S> = T & {
  readonly __tag: S;
};

export type Loadable<T = unknown, E = never> =
  | { state: "loading" }
  | { state: "done"; value: T }
  | { state: "error"; error: E };

export const loadableRef = <T, E>(loadable: Ref<Loadable<T, E>>): Ref<T | undefined> => {
  return toRef(() => (loadable.value.state === "done" ? loadable.value.value : undefined));
};

export const isDone = <T>(
  loadable: MaybeRefOrGetter<Loadable<T, unknown>>,
): loadable is MaybeRefOrGetter<{ state: "done"; value: T }> => toValue(loadable).state === "done";

export const allDone = (
  ...loadables: Array<MaybeRefOrGetter<Loadable<unknown, unknown>>>
): boolean => loadables.every(isDone);

export const returnsError = <E>(
  kind: E | Array<E>,
  ...loadables: Array<MaybeRefOrGetter<Loadable<unknown, E>>>
): boolean => {
  const kinds = Array.isArray(kind) ? kind : [kind];
  return loadables.some((loadable) => {
    const value = toValue(loadable);
    return value.state === "error" && kinds.includes(value.error);
  });
};
