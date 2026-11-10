"use client";

import { createContext, useCallback, useContext, useEffect, useMemo, useState } from "react";

export type Language = "bn" | "en";

type TranslationKey =
    | "home"
    | "subjects"
    | "routine"
    | "mockTest"
    | "progress"
    | "profile"
    | "account"
    | "settings"
    | "billing"
    | "theme"
    | "chooseTheme"
    | "system"
    | "dark"
    | "light"
    | "signIn"
    | "signOut"
    | "dashboard"
    | "language"
    | "chooseLanguage"
    | "saved"
    | "save";

const translations: Record<TranslationKey, Record<Language, string>> = {
    home: { bn: "হোম", en: "Home" },
    subjects: { bn: "বিষয়সমূহ", en: "Subjects" },
    routine: { bn: "রুটিন", en: "Routine" },
    mockTest: { bn: "মক টেস্ট", en: "Mock test" },
    progress: { bn: "অগ্রগতি", en: "Progress" },
    profile: { bn: "প্রোফাইল", en: "Profile" },
    account: { bn: "আমার অ্যাকাউন্ট", en: "My account" },
    settings: { bn: "সেটিংস", en: "Settings" },
    billing: { bn: "বিলিং", en: "Billing" },
    theme: { bn: "থিম", en: "Theme" },
    chooseTheme: { bn: "থিম বেছে নিন", en: "Choose a theme" },
    system: { bn: "সিস্টেম", en: "System" },
    dark: { bn: "ডার্ক", en: "Dark" },
    light: { bn: "লাইট", en: "Light" },
    signIn: { bn: "সাইন ইন", en: "Sign in" },
    signOut: { bn: "সাইন আউট", en: "Sign out" },
    dashboard: { bn: "ড্যাশবোর্ডে ফিরে যান", en: "Back to dashboard" },
    language: { bn: "ভাষা", en: "Language" },
    chooseLanguage: { bn: "অ্যাপের ভাষা নির্বাচন করুন", en: "Choose the app language" },
    saved: { bn: "সংরক্ষিত হয়েছে", en: "Saved" },
    save: { bn: "সংরক্ষণ করুন", en: "Save" },
};

type LanguageContextValue = {
    language: Language;
    setLanguage: (language: Language) => void;
    t: (key: TranslationKey) => string;
};

const LANGUAGE_STORAGE_KEY = "projectx-language";

const LanguageContext = createContext<LanguageContextValue | null>(null);

export function LanguageProvider({
    children,
    initialLanguage = "bn",
}: {
    children: React.ReactNode;
    initialLanguage?: Language;
}) {
    const [language, setLanguageState] = useState<Language>(initialLanguage);

    useEffect(() => {
        document.documentElement.lang = language;
    }, [language]);

    const setLanguage = useCallback((nextLanguage: Language) => {
        setLanguageState(nextLanguage);
        window.localStorage.setItem(LANGUAGE_STORAGE_KEY, nextLanguage);
        document.cookie = `${LANGUAGE_STORAGE_KEY}=${nextLanguage};path=/;max-age=31536000;samesite=lax`;
        document.documentElement.lang = nextLanguage;
    }, []);

    const value = useMemo<LanguageContextValue>(() => ({
        language,
        setLanguage,
        t: (key) => translations[key][language],
    }), [language, setLanguage]);

    return <LanguageContext.Provider value={value}>{children}</LanguageContext.Provider>;
}

export function useLanguage() {
    const context = useContext(LanguageContext);
    if (!context) {
        throw new Error("useLanguage must be used within LanguageProvider");
    }
    return context;
}
