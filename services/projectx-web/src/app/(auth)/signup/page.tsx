"use client";

import Link from "next/link";
import { GraduationCap, Sparkles, UserPlus } from "lucide-react";
import { useState } from "react";
import { signIn } from "next-auth/react";
import { useRouter } from "next/navigation";
import { reqwest } from "@/lib/fetch";
import { REGISTER_ENDPOINT } from "@/constants/endpoints";
import { Field, FieldDescription, FieldGroup, FieldLabel } from "@/components/ui/field";
import { Input } from "@/components/ui/input";

export default function SignupPage() {
    const router = useRouter();
    const [form, setForm] = useState({ name: "", email: "", password: "" });
    const [message, setMessage] = useState("");
    const [loading, setLoading] = useState(false);

    const submit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setLoading(true);
        setMessage("");
        try {
            const response = await reqwest(REGISTER_ENDPOINT, { method: "POST", body: JSON.stringify(form) });
            if (!response.ok) throw new Error("রেজিস্ট্রেশন করা যায়নি। তথ্যগুলো যাচাই করুন।");
            const result = await signIn("credentials", { ...form, redirect: false });
            if (result?.error) throw new Error("অ্যাকাউন্ট তৈরি হয়েছে। এখন সাইন ইন করুন।");
            router.push("/");
        } catch (error) {
            setMessage(error instanceof Error ? error.message : "দুঃখিত, আবার চেষ্টা করুন।");
        } finally {
            setLoading(false);
        }
    };

    return (
        <main className="flex min-h-svh items-center justify-center bg-(--app-bg) p-4 sm:p-8">
            <div className="grid w-full max-w-5xl overflow-hidden rounded-3xl border border-(--app-border) bg-(--app-card) shadow-2xl lg:grid-cols-[0.85fr_1.15fr]">
                <section className="relative hidden overflow-hidden bg-(--app-hero) p-10 text-white lg:flex lg:flex-col lg:justify-between">
                    <div className="absolute -bottom-24 -left-20 size-72 rounded-full bg-(--app-hero-glow)/40 blur-3xl" />
                    <div className="relative">
                        <Link href="/" className="inline-flex items-center gap-3">
                            <span className="flex size-11 items-center justify-center rounded-xl bg-(--app-gold) text-(--app-hero)">
                                <GraduationCap className="size-6" />
                            </span>
                            <span className="text-lg font-bold">পাঠশালা</span>
                        </Link>
                        <div className="mt-24 max-w-xs">
                            <Sparkles className="size-6 text-(--app-gold)" />
                            <h1 className="mt-5 text-4xl font-semibold leading-tight">আপনার শেখার নতুন অধ্যায় শুরু হোক।</h1>
                            <p className="mt-5 text-sm leading-6 text-(--app-hero-text)">একটি অ্যাকাউন্ট তৈরি করে আপনার পড়াশোনার লক্ষ্যগুলো আরও সহজে গুছিয়ে নিন।</p>
                        </div>
                    </div>
                    <p className="relative text-xs text-(--app-hero-muted)">স্মার্টভাবে শিখুন • নিয়মিত এগিয়ে যান</p>
                </section>
                <section className="p-6 sm:p-10">
                    <div className="mb-8 flex items-center gap-3 lg:hidden">
                        <span className="flex size-10 items-center justify-center rounded-xl bg-(--app-hero) text-(--app-gold)">
                            <GraduationCap className="size-5" />
                        </span>
                        <span className="font-bold text-(--app-heading)">পাঠশালা</span>
                    </div>
                    <div className="mb-8">
                        <p className="text-xs font-medium uppercase tracking-[0.18em] text-(--app-muted)">শুরু করুন</p>
                        <h1 className="mt-2 text-3xl font-semibold tracking-tight text-(--app-heading)">অ্যাকাউন্ট তৈরি করুন</h1>
                        <p className="mt-2 text-sm text-(--app-muted-strong)">আপনার শেখার সঙ্গী হিসেবে আজই যোগ দিন।</p>
                    </div>
                    {message && <p className="mb-5 rounded-lg bg-(--app-coral-soft) px-3 py-2 text-xs text-(--app-coral)">{message}</p>}
                    <form onSubmit={submit}>
                        <FieldGroup>
                            <Field>
                                <FieldLabel htmlFor="name">পুরো নাম</FieldLabel>
                                <Input
                                    id="name"
                                    required
                                    minLength={3}
                                    autoComplete="name"
                                    value={form.name}
                                    onChange={(event) => setForm({ ...form, name: event.target.value })}
                                    className="rounded-sm p-5 text-(--app-text)"
                                    placeholder="আপনার নাম"
                                />
                            </Field>
                            <Field>
                                <FieldLabel htmlFor="email">ইমেইল</FieldLabel>
                                <Input
                                    id="email"
                                    required
                                    type="email"
                                    autoComplete="email"
                                    value={form.email}
                                    onChange={(event) => setForm({ ...form, email: event.target.value })}
                                    className="rounded-sm p-5 text-(--app-text)"
                                    placeholder="you@example.com"
                                />
                            </Field>
                            <Field>
                                <FieldLabel htmlFor="password">পাসওয়ার্ড</FieldLabel>
                                <Input
                                    id="password"
                                    required
                                    minLength={6}
                                    type="password"
                                    autoComplete="new-password"
                                    value={form.password}
                                    onChange={(event) => setForm({ ...form, password: event.target.value })}
                                    className="rounded-sm p-5 text-(--app-text)"
                                    placeholder="কমপক্ষে ৬ অক্ষর"
                                />
                            </Field>
                            <Field>
                                <button disabled={loading} className="flex h-11 w-full items-center justify-center gap-2 rounded-xl bg-(--app-primary) px-4 text-sm font-semibold text-white transition hover:bg-(--app-primary-hover) disabled:cursor-not-allowed disabled:opacity-60">
                                    <UserPlus className="size-4" />
                                    {loading ? "তৈরি হচ্ছে..." : "অ্যাকাউন্ট তৈরি করুন"}
                                </button>
                                <FieldDescription className="text-center">
                                    আগেই অ্যাকাউন্ট আছে? <Link href="/signin" className="font-medium text-(--app-primary) hover:underline">সাইন ইন করুন</Link>
                                </FieldDescription>
                            </Field>
                        </FieldGroup>
                    </form>
                    <div className="mt-8 flex items-center justify-center gap-2 text-xs text-(--app-muted)">
                        <span className="h-px flex-1 bg-(--app-border)" />
                        <span>নিরাপদ ও সহজ</span>
                        <span className="h-px flex-1 bg-(--app-border)" />
                    </div>
                </section>
            </div>
        </main>
    );
}
