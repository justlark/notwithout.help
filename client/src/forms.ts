import type { FormResolverOptions } from "@primevue/forms";
import { zodResolver } from "@primevue/forms/resolvers/zod";
import { Schema } from "zod";

export const persistingZodResolver = <T extends Schema<any, any>>(key: string, schema: T) => {
  return ({ values, names }: FormResolverOptions) => {
    localStorage.setItem(key, JSON.stringify(values));
    const innerResolver = zodResolver(schema);
    return innerResolver({ values, names });
  };
};

export const loadPersisted = <T>(key: string, defaults: T): T => {
  const persisted = localStorage.getItem(key);

  if (persisted) {
    return JSON.parse(persisted);
  }

  return defaults;
};
