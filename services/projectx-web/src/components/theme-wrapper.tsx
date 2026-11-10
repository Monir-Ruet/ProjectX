"use client";

import { ThemeProvider } from "./theme-provider";
import { LanguageProvider } from "./language-provider";

import type { Language } from "./language-provider";

export function ThemeWrapper({ children, initialLanguage }: { children: React.ReactNode; initialLanguage: Language }) {
    return (
        <ThemeProvider>
            <LanguageProvider initialLanguage={initialLanguage}>{children}</LanguageProvider>
        </ThemeProvider>
    );
}