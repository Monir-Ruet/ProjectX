"use client";

import Link from "next/link";
import { Bell, ChevronLeft, Globe2, LockKeyhole, Moon, Palette, Save, Settings2, Sun, UserRound } from "lucide-react";
import { useEffect, useState } from "react";
import { useTheme } from "@/components/theme-provider";
import Switch from "@/components/switch";
import { useLanguage } from "@/components/language-provider";

export default function SettingsPage() {
    const { theme, setTheme } = useTheme();
    const { language, setLanguage, t } = useLanguage();
    const [mounted, setMounted] = useState(false);
    const [notifications, setNotifications] = useState(true);
    const [dailyReminder, setDailyReminder] = useState(true);
    const [saved, setSaved] = useState(false);

    useEffect(() => setMounted(true), []);

    return (
        <main className="mx-auto w-full max-w-5xl space-y-4 pb-8">
            <section>
                <p className="text-xs text-(--app-muted-strong)">{language === "bn" ? "আপনার পছন্দ" : "Your preferences"}</p>
                <h1 className="mt-1 text-2xl font-semibold tracking-tight text-(--app-heading)">{t("settings")}</h1>
                <p className="mt-1 text-sm text-(--app-muted)">{language === "bn" ? "অ্যাপটি আপনার মতো করে সাজিয়ে নিন।" : "Personalize the app for yourself."}</p>
            </section>

            <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-4 shadow-sm sm:p-5">
                <div className="flex items-center gap-3">
                    <div className="flex size-9 items-center justify-center rounded-xl bg-(--app-green-soft) text-(--app-primary)"><Settings2 className="size-4" /></div>
                    <div><h2 className="text-sm font-semibold text-(--app-heading)">{language === "bn" ? "সাধারণ" : "General"}</h2><p className="mt-0.5 text-xs text-(--app-muted)">{language === "bn" ? "ভাষা ও চেহারা পরিবর্তন করুন" : "Change language and appearance"}</p></div>
                </div>
                <div className="mt-4 divide-y divide-(--app-border-soft)">
                    <div className="flex items-center justify-between gap-4 py-3 first:pt-0">
                        <div className="flex items-center gap-3"><Globe2 className="size-4 text-(--app-muted)" /><div><p className="text-sm font-medium text-(--app-text)">{t("language")}</p><p className="text-xs text-(--app-muted)">{t("chooseLanguage")}</p></div></div>
                        <select value={language} onChange={(event) => setLanguage(event.target.value as "bn" | "en")} className="rounded-lg border border-(--app-border-strong) bg-(--app-card) px-2 py-1.5 text-xs text-(--app-text) outline-none"><option value="bn">বাংলা</option><option value="en">English</option></select>
                    </div>
                    <div className="flex items-center justify-between gap-4 py-3">
                        <div className="flex items-center gap-3"><Palette className="size-4 text-(--app-muted)" /><div><p className="text-sm font-medium text-(--app-text)">{t("theme")}</p><p className="text-xs text-(--app-muted)">{language === "bn" ? "লাইট, ডার্ক অথবা সিস্টেম" : "Light, dark, or system"}</p></div></div>
                        <div className="flex gap-1 rounded-lg bg-(--app-soft) p-1">
                            <button onClick={() => setTheme("light")} aria-label={t("light")} className={`rounded-md p-1.5 ${mounted && theme === "light" ? "bg-(--app-card) text-(--app-primary) shadow-sm" : "text-(--app-muted)"}`}><Sun className="size-3.5" /></button>
                            <button onClick={() => setTheme("dark")} aria-label={t("dark")} className={`rounded-md p-1.5 ${mounted && theme === "dark" ? "bg-(--app-card) text-(--app-primary) shadow-sm" : "text-(--app-muted)"}`}><Moon className="size-3.5" /></button>
                            <button onClick={() => setTheme("system")} aria-label={t("system")} className={`rounded-md px-2 py-1 text-[10px] ${mounted && theme === "system" ? "bg-(--app-card) text-(--app-primary) shadow-sm" : "text-(--app-muted)"}`}>{t("system")}</button>
                        </div>
                    </div>
                </div>
            </section>

            <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-4 shadow-sm sm:p-5">
                <div className="flex items-center gap-3">
                    <div className="flex size-9 items-center justify-center rounded-xl bg-(--app-gold-soft) text-(--app-gold-text)"><Bell className="size-4" /></div>
                    <div><h2 className="text-sm font-semibold text-(--app-heading)">{language === "bn" ? "নোটিফিকেশন" : "Notifications"}</h2><p className="mt-0.5 text-xs text-(--app-muted)">{language === "bn" ? "আপনার শেখার আপডেট নিয়ন্ত্রণ করুন" : "Control your learning updates"}</p></div>
                </div>
                <div className="mt-4 divide-y divide-(--app-border-soft)">
                    <div className="flex items-center justify-between gap-4 py-3 first:pt-0"><div><p className="text-sm font-medium text-(--app-text)">{language === "bn" ? "নোটিফিকেশন" : "Notifications"}</p><p className="text-xs text-(--app-muted)">{language === "bn" ? "পরীক্ষা ও অগ্রগতির আপডেট পান" : "Receive test and progress updates"}</p></div><Switch checked={notifications} onChange={() => setNotifications((value) => !value)} /></div>
                    <div className="flex items-center justify-between gap-4 py-3 last:pb-0"><div><p className="text-sm font-medium text-(--app-text)">{language === "bn" ? "দৈনিক রিমাইন্ডার" : "Daily reminder"}</p><p className="text-xs text-(--app-muted)">{language === "bn" ? "আজকের পড়াশোনার কথা মনে করিয়ে দিন" : "Remind me about today's study"}</p></div><Switch checked={dailyReminder} onChange={() => setDailyReminder((value) => !value)} /></div>
                </div>
            </section>

            <section className="grid gap-3 sm:grid-cols-2">
                <Link href="/profile" className="flex items-center gap-3 rounded-2xl border border-(--app-border) bg-(--app-card) p-4 shadow-sm hover:bg-(--app-soft)"><UserRound className="size-4 text-(--app-primary)" /><span className="flex-1"><span className="block text-sm font-medium text-(--app-text)">{language === "bn" ? "প্রোফাইল সম্পাদনা" : "Edit profile"}</span><span className="mt-0.5 block text-xs text-(--app-muted)">{language === "bn" ? "ব্যক্তিগত তথ্য পরিবর্তন করুন" : "Change personal information"}</span></span><ChevronLeft className="size-4 text-(--app-muted)" /></Link>
                <button className="flex items-center gap-3 rounded-2xl border border-(--app-border) bg-(--app-card) p-4 text-right shadow-sm hover:bg-(--app-soft)"><LockKeyhole className="size-4 text-(--app-blue)" /><span className="flex-1"><span className="block text-sm font-medium text-(--app-text)">{language === "bn" ? "নিরাপত্তা" : "Security"}</span><span className="mt-0.5 block text-xs text-(--app-muted)">{language === "bn" ? "পাসওয়ার্ড ও নিরাপত্তা" : "Password and security"}</span></span><ChevronLeft className="size-4 text-(--app-muted)" /></button>
            </section>

            <div className="flex items-center justify-between">
                <Link href="/" className="text-xs font-medium text-(--app-primary)">{t("dashboard")}</Link>
                <button onClick={() => { setSaved(true); window.setTimeout(() => setSaved(false), 2000); }} className="inline-flex items-center gap-2 rounded-lg bg-(--app-primary) px-3 py-2 text-xs font-medium text-white hover:bg-(--app-primary-hover)"><Save className="size-3.5" />{saved ? t("saved") : t("save")}</button>
            </div>
        </main>
    );
}
