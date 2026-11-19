import type { Role } from "@pulse/protocol";

export const roleLabels: Record<Role, string> = {
  owner: "Owner",
  admin: "Admin",
  member: "Member",
  guest: "Guest",
  steward: "Steward",
};
