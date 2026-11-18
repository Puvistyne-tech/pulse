import type { CampusId, CampusSummary } from "@pulse/protocol";

interface CampusPickerProps {
  campuses: CampusSummary[];
  selectCampus(campusId: CampusId): Promise<void>;
}

export function CampusPicker({ campuses, selectCampus }: CampusPickerProps) {
  return (
    <nav aria-label="Campuses">
      <ul className="space-y-1">
        {campuses.map((campus) => (
          <li key={campus.campus_id}>
            <button
              className="text-left underline"
              type="button"
              onClick={() => void selectCampus(campus.campus_id)}
            >
              {campus.display_name}
            </button>
          </li>
        ))}
      </ul>
    </nav>
  );
}
