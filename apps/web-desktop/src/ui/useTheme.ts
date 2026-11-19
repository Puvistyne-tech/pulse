import { useEffect, useState } from "react";

export type ThemeChoice = "light" | "dark" | "system";

const storageKey = "pulse.theme";
const darkQuery = "(prefers-color-scheme: dark)";

function storedChoice(): ThemeChoice {
  try {
    const value = window.localStorage.getItem(storageKey);
    return value === "light" || value === "dark" ? value : "system";
  } catch {
    return "system";
  }
}

function rememberChoice(choice: ThemeChoice) {
  try {
    window.localStorage.setItem(storageKey, choice);
  } catch {
    document.documentElement.dataset.themeChoice = choice;
  }
}

function systemPrefersDark(): boolean {
  return typeof window.matchMedia === "function" && window.matchMedia(darkQuery).matches;
}

export function useTheme(): [ThemeChoice, (choice: ThemeChoice) => void] {
  const [choice, setChoice] = useState<ThemeChoice>(storedChoice);

  useEffect(() => {
    const apply = () => {
      const dark = choice === "dark" || (choice === "system" && systemPrefersDark());
      document.documentElement.dataset.theme = dark ? "dark" : "light";
    };
    apply();
    rememberChoice(choice);
    if (choice !== "system" || typeof window.matchMedia !== "function") {
      return;
    }
    const media = window.matchMedia(darkQuery);
    media.addEventListener("change", apply);
    return () => media.removeEventListener("change", apply);
  }, [choice]);

  return [choice, setChoice];
}
