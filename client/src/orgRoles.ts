import type { OrgRole } from "./api";
import type { Loadable } from "./types";

const slugifyName = (name: string): string => {
  return name.toLowerCase().replace(/\s+/g, "-");
};

export type RolesParseError = "empty" | "duplicates";

export const parseRolesFile = (rolesFile: string): Loadable<Array<OrgRole>, RolesParseError> => {
  if (rolesFile.trim().length === 0) {
    return { state: "error", error: "empty" };
  }

  const lines = rolesFile.split(/\r?\n/);

  const roles: Array<OrgRole> = [];
  const roleSlugs = new Set();
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
      if (roleSlugs.has(slugifyName(line))) {
        return { state: "error", error: "duplicates" };
      }

      role = {
        id: slugifyName(line),
        name: line,
        details: [],
      };

      roleSlugs.add(slugifyName(role.name));

      continue;
    }

    role.details.push(line);
  }

  return { state: "done", value: roles };
};
