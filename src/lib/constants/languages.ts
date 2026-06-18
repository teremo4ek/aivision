export interface Language {
  value: string;
  label: string;
}

export const LANGUAGES: Language[] = [
  { value: "auto", label: "Auto Detect" },
  { value: "en", label: "English" },
  { value: "ru", label: "Russian" },
];
