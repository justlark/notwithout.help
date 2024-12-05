import type { FormResolverOptions } from "@primevue/forms";
import { zodResolver } from "@primevue/forms/resolvers/zod";
import { Schema } from "zod";

export const persistingZodResolver = <T extends Schema<any, any>>(key: string, schema: T) => {
  return async ({ values, names }: FormResolverOptions) => {
    const innerResolver = zodResolver(schema);
    const result = await innerResolver({ values, names });
    localStorage.setItem(key, JSON.stringify(result.values));
    return result;
  };
};

export const loadPersisted = <T>(key: string, map: (value: Record<string, any>) => T): T => {
  const persisted = localStorage.getItem(key);

  if (persisted) {
    return map(JSON.parse(persisted));
  }

  return map({});
};
