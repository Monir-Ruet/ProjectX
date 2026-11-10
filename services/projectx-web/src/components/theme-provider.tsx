"use client";

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";

type Theme = "light" | "dark" | "system";

type ThemeContextValue = {
    theme: Theme;
    resolvedTheme: "light" | "dark";
    setTheme: (theme: Theme) => void;
};

const ThemeContext = createContext<ThemeContextValue | null>(null);

function getSystemTheme(): "light" | "dark" {
    return window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
}

export function ThemeProvider({ children }: { children: React.ReactNode }) {
    const [theme, setThemeState] = useState<Theme>("system");
    const [systemTheme, setSystemTheme] = useState<"light" | "dark">("light");

    useEffect(() => {
        const storedTheme = window.localStorage.getItem("theme");
        if (storedTheme === "light" || storedTheme === "dark" || storedTheme === "system") {
            setThemeState(storedTheme);
        }

        const mediaQuery = window.matchMedia("(prefers-color-scheme: dark)");
        const updateSystemTheme = () => setSystemTheme(getSystemTheme());
        updateSystemTheme();
        mediaQuery.addEventListener("change", updateSystemTheme);
        return () => mediaQuery.removeEventListener("change", updateSystemTheme);
    }, []);

    const resolvedTheme = theme === "system" ? systemTheme : theme;

    useEffect(() => {
        document.documentElement.classList.toggle("dark", resolvedTheme === "dark");
        document.documentElement.style.colorScheme = resolvedTheme;
    }, [resolvedTheme]);

    const setTheme = useCallback((nextTheme: Theme) => {
        setThemeState(nextTheme);
        window.localStorage.setItem("theme", nextTheme);
    }, []);

    const value = useMemo(() => ({ theme, resolvedTheme, setTheme }), [theme, resolvedTheme, setTheme]);

    return <ThemeContext.Provider value={value}>{children}</ThemeContext.Provider>;
}

export function useTheme() {
    const context = useContext(ThemeContext);
    if (!context) {
        throw new Error("useTheme must be used within ThemeProvider");
    }
    return context;
}
