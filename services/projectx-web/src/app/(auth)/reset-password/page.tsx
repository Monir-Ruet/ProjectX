"use client";

import Link from "next/link";
import { ArrowLeft, CheckCircle2, LockKeyhole } from "lucide-react";
import { useState } from "react";
import { BrandIcon } from "@/components/brand-icon";
import { useSearchParams } from "next/navigation";

export default function ResetPasswordPage() {
    const searchParams = useSearchParams();
    const token = searchParams.get("token");
    const [password, setPassword] = useState("");
    const [confirm, setConfirm] = useState("");
    const [saved, setSaved] = useState(false);
    const mismatch = confirm.length > 0 && password !== confirm;
    let tokenExpired = true;
    if (token) {
        try {
            const payload = JSON.parse(atob(token.split(".")[1] || token)) as { exp?: number };
            tokenExpired = !payload.exp || payload.exp <= Math.floor(Date.now() / 1000);
        } catch {
            tokenExpired = true;
        }
    }

    return (
        <main className="relative min-h-svh items-center justify-center bg-(--app-bg) p-4 flex flex-col gap-5">
            <BrandIcon />
            <section className="w-full max-w-md rounded-3xl border border-(--app-border) bg-(--app-card) p-6 shadow-xl sm:p-8">
                <Link href="/signin" className="inline-flex items-center gap-1 text-xs font-medium text-(--app-primary)"><ArrowLeft className="size-3.5" /> সাইন ইন-এ ফিরে যান</Link>
                <div className="mt-8 flex size-11 items-center justify-center rounded-xl bg-(--app-blue-soft) text-(--app-blue)"><LockKeyhole className="size-5" /></div>
                <h1 className="mt-4 text-2xl font-semibold text-(--app-heading)">নতুন পাসওয়ার্ড</h1>
                <p className="mt-1 text-sm text-(--app-muted-strong)">আপনার অ্যাকাউন্টের জন্য একটি শক্তিশালী পাসওয়ার্ড তৈরি করুন।</p>
                {
                    tokenExpired ?
                        <div className="mt-6 rounded-xl bg-(--app-coral-soft) p-4 text-sm text-(--app-coral)">
                            এই রিসেট লিংকটি অনুপস্থিত বা মেয়াদ শেষ হয়ে গেছে।
                            <Link href="/forgot-password" className="mt-2 block font-semibold underline">নতুন লিংক নিন</Link>
                        </div> :
                        saved ?
                            <div className="mt-6 rounded-xl bg-(--app-green-soft) p-4 text-sm text-(--app-primary)">
                                <CheckCircle2 className="mb-2 size-5" />পাসওয়ার্ড পরিবর্তনের অনুরোধ প্রস্তুত হয়েছে। এখন সাইন ইন করতে পারেন।</div>
                            : <form onSubmit={
                                (event) => {
                                    event.preventDefault(); if (!mismatch) setSaved(true);

                                }
                            } className="mt-6 space-y-4">
                                <input required minLength={6} type="password" value={password} onChange={
                                    (event) => setPassword(event.target.value)
                                } className="w-full rounded-xl border border-(--app-border-strong) bg-transparent px-3 py-3 text-sm outline-none focus:border-(--app-primary)" placeholder="নতুন পাসওয়ার্ড" />
                                <input required minLength={6} type="password" value={confirm} onChange={(event) => setConfirm(event.target.value)} className="w-full rounded-xl border border-(--app-border-strong) bg-transparent px-3 py-3 text-sm outline-none focus:border-(--app-primary)" placeholder="পাসওয়ার্ড নিশ্চিত করুন" />
                                {
                                    mismatch &&
                                    <p className="text-xs text-(--app-coral)">পাসওয়ার্ড দুটি মিলছে না।</p>
                                }
                                <button disabled={mismatch} className="w-full rounded-xl bg-(--app-primary) px-4 py-3 text-sm font-semibold text-white hover:bg-(--app-primary-hover) disabled:opacity-50">পাসওয়ার্ড পরিবর্তন করুন</button>
                            </form>}
            </section>
        </main>
    );
}
