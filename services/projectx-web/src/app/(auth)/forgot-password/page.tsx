"use client";

import Link from "next/link";
import { ArrowLeft, KeyRound, MailCheck } from "lucide-react";
import { useState } from "react";
import { BrandIcon } from "@/components/brand-icon";

export default function ForgotPasswordPage() {
    const [email, setEmail] = useState("");
    const [sent, setSent] = useState(false);
    const [resetToken, setResetToken] = useState("");

    const sendResetLink = (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setResetToken(btoa(JSON.stringify({ exp: Math.floor(Date.now() / 1000) + 3600 })));
        setSent(true);
    };

    return (
        <main className="relative min-h-svh items-center justify-center bg-(--app-bg) p-4 flex flex-col gap-5">
            <BrandIcon />
            <section className="w-full max-w-md rounded-3xl border border-(--app-border) bg-(--app-card) p-6 shadow-xl sm:p-8">
                <Link href="/signin" className="inline-flex items-center gap-1 text-xs font-medium text-(--app-primary)"><ArrowLeft className="size-3.5" /> সাইন ইন-এ ফিরে যান</Link>
                <div className="mt-4 flex size-11 items-center justify-center rounded-xl bg-(--app-gold-soft) text-(--app-gold-text)"><KeyRound className="size-5" /></div>
                <h1 className="mt-4 text-2xl font-semibold text-(--app-heading)">পাসওয়ার্ড ভুলে গেছেন?</h1>
                <p className="mt-1 text-sm leading-6 text-(--app-muted-strong)">আপনার ইমেইল লিখুন। আমরা রিসেট করার নির্দেশনা পাঠাব।</p>
                {sent ? <div className="mt-6 rounded-xl bg-(--app-green-soft) p-4 text-sm text-(--app-primary)"><MailCheck className="mb-2 size-5" />ইমেইলটি পাওয়া গেলে নির্দেশনা পাঠানো হবে। আপনার ইনবক্স দেখুন।<Link href={`/reset-password?token=${encodeURIComponent(resetToken)}`} className="mt-3 block font-semibold underline">নতুন পাসওয়ার্ড দিন</Link></div> : <form onSubmit={sendResetLink} className="mt-6 space-y-4"><input required type="email" value={email} onChange={(event) => setEmail(event.target.value)} className="w-full rounded-xl border border-(--app-border-strong) bg-transparent px-3 py-3 text-sm outline-none focus:border-(--app-primary)" placeholder="you@example.com" /><button className="w-full rounded-xl bg-(--app-primary) px-4 py-3 text-sm font-semibold text-white hover:bg-(--app-primary-hover)">রিসেট লিংক পাঠান</button></form>}
            </section>
        </main>
    );
}
