const hues = [
  "bg-sky-600",
  "bg-emerald-600",
  "bg-amber-600",
  "bg-rose-600",
  "bg-cyan-700",
  "bg-orange-600",
  "bg-lime-700",
  "bg-pink-600",
];

export function initialsOf(name: string): string {
  const words = name.trim().split(/\s+/).filter(Boolean);
  const letters = words.length > 1 ? [words[0]![0], words[1]![0]] : [name.trim().slice(0, 2)];
  return letters.join("").toUpperCase() || "?";
}

function hueOf(name: string): string {
  let hash = 0;
  for (const char of name) {
    hash = (hash * 31 + char.charCodeAt(0)) >>> 0;
  }
  return hues[hash % hues.length]!;
}

const sizes = {
  sm: "h-6 w-6 text-[10px]",
  md: "h-9 w-9 text-xs",
  lg: "h-10 w-10 text-sm",
};

interface AvatarProps {
  name: string;
  size?: keyof typeof sizes;
  square?: boolean;
}

export function Avatar({ name, size = "md", square = false }: AvatarProps) {
  return (
    <span
      aria-hidden="true"
      className={`inline-flex shrink-0 select-none items-center justify-center font-semibold text-white ${hueOf(name)} ${sizes[size]} ${square ? "rounded-lg" : "rounded-full"}`}
    >
      {initialsOf(name)}
    </span>
  );
}
