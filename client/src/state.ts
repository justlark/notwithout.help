// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const persistState = <T extends Record<string, any>>(
  key: string,
  values: T,
  // eslint-disable-next-line @typescript-eslint/no-explicit-any
  transform: (input: T) => Record<string, any> = (values) => values,
) => {
  localStorage.setItem(key, JSON.stringify(transform(values)));
};

// eslint-disable-next-line @typescript-eslint/no-explicit-any
export const loadState = <T>(key: string, transform: (value: Record<string, any>) => T): T => {
  const persisted = localStorage.getItem(key);

  if (persisted) {
    return transform(JSON.parse(persisted));
  }

  return transform({});
};