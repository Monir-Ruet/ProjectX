"use client";

import { useTheme } from "@/components/theme-provider";
import { Button } from "@/components/ui/button";
import { Sun, Moon } from "lucide-react";
import { useEffect, useState } from "react";

export function ThemeToggle() {
    const { resolvedTheme, setTheme } = useTheme();
    const [mounted, setMounted] = useState(false);

    useEffect(() => setMounted(true), []);

    return (
        <Button
            onClick={() => setTheme(resolvedTheme === "dark" ? "light" : "dark")}
            variant="outline"
            size="icon"
            className="size-9 rounded-xl border-(--app-border) bg-(--app-soft) text-(--app-text) hover:bg-(--app-green-soft)"
            aria-label={mounted && resolvedTheme === "dark" ? "লাইট মোড চালু করুন" : "ডার্ক মোড চালু করুন"}
            title={mounted && resolvedTheme === "dark" ? "লাইট মোড" : "ডার্ক মোড"}
        >
            {mounted && resolvedTheme === "dark" ? <Sun className="size-4" /> : <Moon className="size-4" />}
            <span className="sr-only">থিম পরিবর্তন করুন</span>
        </Button>
    );
}
