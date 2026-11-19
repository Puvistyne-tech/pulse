import type { CampusId, CampusSummary } from "@pulse/protocol";
import { initialsOf } from "../ui/Avatar";

interface CampusPickerProps {
  campuses: CampusSummary[];
  activeCampusId?: CampusId | null;
  selectCampus(campusId: CampusId): Promise<void>;
}

export function CampusPicker({ campuses, activeCampusId = null, selectCampus }: CampusPickerProps) {
  return (
    <nav aria-label="Campuses">
      <ul className="flex flex-col items-center gap-2">
        {campuses.map((campus) => {
          const active = campus.campus_id === activeCampusId;
          return (
            <li key={campus.campus_id} className="group relative">
              <button
                className={`flex h-10 w-10 items-center justify-center rounded-xl text-sm font-semibold transition-all ${
                  active
                    ? "bg-accent text-on-accent ring-2 ring-accent/40 ring-offset-2 ring-offset-rail"
                    : "bg-white/10 text-side-ink hover:rounded-lg hover:bg-white/20"
                }`}
                type="button"
                aria-current={active ? "true" : undefined}
                onClick={() => void selectCampus(campus.campus_id)}
              >
                <span aria-hidden="true">{initialsOf(campus.display_name)}</span>
                <span className="pointer-events-none absolute left-full z-30 ml-3 whitespace-nowrap rounded-md bg-ink px-2 py-1 text-xs font-medium text-canvas opacity-0 shadow-panel transition-opacity group-hover:opacity-100">
                  {campus.display_name}
                </span>
              </button>
            </li>
          );
        })}
      </ul>
    </nav>
  );
}
