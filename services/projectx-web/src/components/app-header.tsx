"use client";

import { CalendarDays, GraduationCap } from "lucide-react";
import { useSession } from "next-auth/react";
import { MenuActions } from "@/components/sidebar/nav-actions";
import Link from "next/link";
import { Skeleton } from "@/components/ui/skeleton";
import { useLanguage } from "@/components/language-provider";

export function AppHeader() {
    const { data: session, status } = useSession();
    const { language, t } = useLanguage();
    const localizedToday = new Intl.DateTimeFormat(language === "bn" ? "bn-BD" : "en-US", {
        weekday: "long",
        day: "numeric",
        month: "long",
    }).format(new Date());

    return (
        <header className="sticky top-0 z-30 border-b border-(--app-border)/70 bg-(--app-card)/75 backdrop-blur-md">
            <div className="mx-auto flex h-18 w-full items-center justify-between px-4 sm:px-6 lg:w-3/5 lg:px-0">
                <div className="flex items-center gap-3">
                    <Link href="/" className="flex items-center gap-2">
                        <div className="flex size-10 items-center justify-center rounded-xl bg-(--app-hero) text-(--app-gold)">
                            <GraduationCap className="size-5" />
                        </div>
                        <div>
                            <p className="text-lg font-bold leading-tight tracking-tight text-(--app-heading)">পাঠশালা</p>
                            <p className="mt-0.5 text-[11px] text-(--app-muted)">{language === "bn" ? "আপনার শেখার সঙ্গী" : "Your learning companion"}</p>
                        </div>
                    </Link>
                </div>

                <div className="flex items-center gap-3">
                    <div className="hidden items-center gap-2 text-left sm:flex">
                        <CalendarDays className="size-4 text-(--app-primary)" />
                        <span className="text-xs font-medium text-(--app-muted-strong)">{localizedToday}</span>
                    </div>
                    <div className="h-6 w-px bg-(--app-border) sm:block" />
                    {
                        session?.user?.name && (
                            <div className="flex items-center gap-2">
                                <div className="hidden text-right sm:block">
                                    <p className="text-xs font-semibold text-(--app-text)">{session?.user?.name || "শিক্ষার্থী"}</p>
                                    <p className="text-[11px] text-(--app-muted)">{language === "bn" ? "শিক্ষার্থী" : "Student"}</p>
                                </div>

                            </div>
                        )
                    }
                    <MenuActions />
                    {status === "loading" ? (
                        <Skeleton className="h-8 w-16 rounded-lg" aria-hidden="true" />
                    ) : !session?.user && (
                        <Link
                            href="/signin"
                            className="rounded-lg bg-(--app-primary) px-3 py-2 text-xs font-semibold text-white transition hover:bg-(--app-primary-hover)"
                        >
                            {t("signIn")}
                        </Link>
                    )}
                </div>
            </div>
        </header>
    );
}
