import type { FormResolverOptions } from "@primevue/forms";
import { zodResolver } from "@primevue/forms/resolvers/zod";
import { Schema, z } from "zod";

export const persistingResolver = <T extends Schema<any, any>>(
  key: string,
  schema: T,
  transform: (input: z.infer<T>) => Record<string, any> = (input) => input,
) => {
  return async ({ values, names }: FormResolverOptions) => {
    const transformed = transform(values);
    localStorage.setItem(key, JSON.stringify(transformed));

    const resolver = zodResolver(schema);
    return await resolver({ values, names });
  };
};

export const loadPersisted = <T>(key: string, transform: (value: Record<string, any>) => T): T => {
  const persisted = localStorage.getItem(key);

  if (persisted) {
    return transform(JSON.parse(persisted));
  }

  return transform({});
};
