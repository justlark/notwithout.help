import type { OrgRole } from "./api";
import type { Loadable } from "./types";

const randomDigits = (length: number): string =>
  Array.from({ length }, () => Math.floor(Math.random() * 10)).join("");

const nameToSlug = (name: string): string => {
  return name.toLowerCase().replace(/\s+/g, "-") + randomDigits(4);
};

export type RolesParseError = "empty" | "duplicates";

export const parseRolesFile = (rolesFile: string): Loadable<Array<OrgRole>, RolesParseError> => {
  if (rolesFile.trim().length === 0) {
    return { state: "error", error: "empty" };
  }

  const lines = rolesFile.split(/\r?\n/);

  const roles: Array<OrgRole> = [];
  const roleNames = new Set();
  let role: OrgRole | undefined;

  for (const line of lines) {
    if (line === undefined) {
      break;
    }

    if (line.trim().length === 0) {
      if (role !== undefined) {
        roles.push(role);
      }

      role = undefined;

      continue;
    }

    if (role === undefined) {
      if (roleNames.has(line)) {
        return { state: "error", error: "duplicates" };
      }

      role = {
        id: nameToSlug(line),
        name: line,
        details: [],
      };

      roleNames.add(role.name);

      continue;
    }

    role.details.push(line);
  }

  return { state: "done", value: roles };
};
