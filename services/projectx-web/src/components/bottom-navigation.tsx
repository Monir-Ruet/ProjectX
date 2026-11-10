"use client";

import { BarChart3, BookOpen, ClipboardCheck, ClipboardList, LayoutDashboard, UserRound } from "lucide-react";
import Link from "next/link";
import { usePathname } from "next/navigation";
import { useLanguage } from "@/components/language-provider";

const navigation = [
    { key: "home", href: "/", icon: LayoutDashboard },
    { key: "subjects", href: "/subjects", icon: BookOpen },
    { key: "routine", href: "/routine", icon: ClipboardCheck },
    { key: "mockTest", href: "/mock-test", icon: ClipboardList },
    { key: "progress", href: "/progress", icon: BarChart3 },
    { key: "profile", href: "/profile", icon: UserRound },
] as const;

export function BottomNavigation() {
    const pathname = usePathname();
    const { t } = useLanguage();

    return (
        <nav className="fixed inset-x-0 bottom-0 z-40 border-t border-(--app-border) bg-(--app-card) pb-[env(safe-area-inset-bottom)] shadow-[0_-8px_24px_rgba(27,59,48,0.06)] backdrop-blur-md">
            <div className="mx-auto flex h-16 w-full items-stretch justify-around px-1 lg:w-3/5">
                {navigation.map((item) => {
                    const Icon = item.icon;
                    const active = pathname === item.href;

                    return (
                        <Link
                            key={item.href}
                            href={item.href}
                            className={`group flex min-w-0 flex-1 flex-col items-center justify-center gap-0.5 text-[10px] font-medium transition-colors ${active ? "text-(--app-primary)" : "text-(--app-muted) hover:text-(--app-primary)"}`}
                        >
                            <span className={`flex size-7 items-center justify-center rounded-lg transition-colors ${active ? "bg-(--app-green-soft)" : "group-hover:bg-(--app-soft)"}`}>
                                <Icon className={`size-4 ${active ? "stroke-[2.4]" : "stroke-[1.8]"}`} />
                            </span>
                            {t(item.key)}
                        </Link>
                    );
                })}
            </div>
        </nav>
    );
}
