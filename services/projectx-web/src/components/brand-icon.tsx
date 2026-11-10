import Link from "next/link";
import { GraduationCap } from "lucide-react";

export function BrandIcon() {
    return (
        <Link href="/" className="flex flex-row justify-center items-center text-center gap-2 text-sm font-bold text-(--app-heading)">
            <span className="flex size-8 items-center justify-center rounded-lg bg-(--app-hero) text-(--app-gold)">
                <GraduationCap className="size-4" />
            </span>পাঠশালা
        </Link>
    );
}
