import type { OrgRole } from "./api";

const randomDigits = (length: number): string =>
  Array.from({ length }, () => Math.floor(Math.random() * 10)).join("");

const nameToSlug = (name: string): string => {
  return name.toLowerCase().replace(/\s+/g, "-") + randomDigits(4);
};

export const parseRolesFile = (rolesFile: string): Array<OrgRole> => {
  const lines = rolesFile.split(/\r?\n/);
  lines.reverse();

  const roles: Array<OrgRole> = [];
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
      role = {
        id: nameToSlug(line),
        name: line,
        details: [],
      };

      continue;
    }

    role.details.push(line);
  }

  return roles;
};
