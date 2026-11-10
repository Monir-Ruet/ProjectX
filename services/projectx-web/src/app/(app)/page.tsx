import { auth } from "@/auth";
import Link from "next/link";
import {
    ArrowLeft,
    ArrowUpRight,
    BookOpen,
    BriefcaseBusiness,
    CalendarDays,
    Check,
    Clock3,
    Flame,
    GraduationCap,
    Play,
    Sparkles,
    Target,
} from "lucide-react";

export default async function Home() {
    const session = await auth();
    const firstName = session?.user?.name?.split(" ")[0] || "শিক্ষার্থী";

    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="relative overflow-hidden rounded-3xl bg-(--app-hero) px-6 py-7 text-white shadow-sm sm:px-10 sm:py-9">
                <div className="absolute -right-20 -top-24 h-72 w-72 rounded-full bg-(--app-hero-glow)/50 blur-2xl" />
                <div className="absolute -bottom-22.5 right-1/4 h-52 w-52 rounded-full bg-(--app-gold)/20 blur-3xl" />
                <div className="relative max-w-2xl">
                    <div className="mb-5 inline-flex items-center gap-2 rounded-full bg-(--app-)/10 px-3 py-1.5 text-xs font-medium text-(--app-green) dark:text-(--app-text)">
                        <Sparkles className="size-3.5 text-(--app-gold)" />
                        আপনার শেখার যাত্রা অব্যাহত রাখুন
                    </div>
                    <h1 className="max-w-xl text-3xl font-semibold leading-tight tracking-tight sm:text-4xl">
                        শুভ সকাল, {firstName}! আজ একটু এগিয়ে যান।
                    </h1>
                    <p className="mt-3 max-w-lg text-sm leading-6 text-(--app-hero-text) sm:text-base">
                        প্রতিদিনের ছোট ছোট অগ্রগতিই আপনাকে আপনার লক্ষ্যের কাছে নিয়ে যাবে।
                    </p>
                    <button className="mt-7 inline-flex items-center gap-2 rounded-xl bg-(--app-gold) px-4 py-2.5 text-sm font-semibold text-(--app-hero) transition hover:bg-(--app-gold-hover)">
                        পড়াশোনা শুরু করুন
                        <ArrowUpRight className="size-4" />
                    </button>
                </div>
            </section>

            <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                {[
                    { label: "সাপ্তাহিক অগ্রগতি", value: "৭২%", detail: "+১২% গত সপ্তাহ থেকে", icon: Target, color: "text-(--app-primary)", bg: "bg-(--app-green-soft)" },
                    { label: "সম্পন্ন অধ্যায়", value: "২৪", detail: "মোট ৩৬টি অধ্যায়", icon: BookOpen, color: "text-(--app-blue)", bg: "bg-(--app-blue-soft)" },
                    { label: "পড়ার সময়", value: "১৮.৫ ঘন্টা", detail: "এই সপ্তাহে", icon: Clock3, color: "text-(--app-gold-text)", bg: "bg-(--app-gold-soft)" },
                    { label: "অবিচ্ছিন্ন দিন", value: "১২ দিন", detail: "দারুণ ধারাবাহিকতা!", icon: Flame, color: "text-(--app-coral)", bg: "bg-(--app-coral-soft)" },
                ].map((stat) => {
                    const Icon = stat.icon;
                    return (
                        <div key={stat.label} className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm">
                            <div className="flex items-start justify-between">
                                <span className="text-sm text-(--app-muted-strong)">{stat.label}</span>
                                <span className={`rounded-xl p-2 ${stat.bg} ${stat.color}`}><Icon className="size-4" /></span>
                            </div>
                            <p className="mt-5 text-2xl font-semibold tracking-tight text-(--app-heading)">{stat.value}</p>
                            <p className="mt-1 text-xs text-(--app-muted)">{stat.detail}</p>
                        </div>
                    );
                })}
            </section>

            <div className="grid gap-6 lg:grid-cols-[1.35fr_1fr]">
                <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm sm:p-6">
                    <div className="flex items-center justify-between">
                        <div>
                            <h2 className="text-lg font-semibold text-(--app-heading)">আজকের পড়াশোনা</h2>
                            <p className="mt-1 text-sm text-(--app-muted-strong)">আপনার নির্ধারিত বিষয়গুলো শেষ করুন</p>
                        </div>
                        <button className="inline-flex items-center gap-1 text-sm font-medium text-(--app-primary) hover:text-(--app-hero)">
                            সব দেখুন <ArrowLeft className="size-4" />
                        </button>
                    </div>
                    <div className="mt-6 space-y-3">
                        {[
                            { title: "বাংলা সাহিত্য", subtitle: "রবীন্দ্রনাথ ঠাকুর — ছোটগল্প", time: "৪৫ মিনিট", done: true, tone: "bg-(--app-green-soft) text-(--app-primary)" },
                            { title: "সাধারণ গণিত", subtitle: "বীজগণিতের মৌলিক ধারণা", time: "৬০ মিনিট", done: false, tone: "bg-(--app-gold-soft) text-(--app-gold-text)" },
                            { title: "ইংরেজি ব্যাকরণ", subtitle: "কাল ও বাক্য গঠন", time: "৩০ মিনিট", done: false, tone: "bg-(--app-blue-soft) text-(--app-blue)" },
                        ].map((lesson) => (
                            <div key={lesson.title} className="flex items-center gap-3 rounded-xl border border-(--app-border-soft) p-3.5">
                                <div className={`flex size-9 shrink-0 items-center justify-center rounded-full ${lesson.done ? "bg-(--app-primary) text-white" : "border-2 border-(--app-border-strong) text-transparent"}`}>
                                    <Check className="size-4" />
                                </div>
                                <div className="min-w-0 flex-1">
                                    <p className={`font-medium ${lesson.done ? "text-(--app-muted) line-through" : "text-(--app-text)"}`}>{lesson.title}</p>
                                    <p className="mt-0.5 truncate text-xs text-(--app-muted)">{lesson.subtitle}</p>
                                </div>
                                <span className={`hidden rounded-lg px-2.5 py-1.5 text-xs font-medium sm:block ${lesson.tone}`}>{lesson.time}</span>
                                {!lesson.done && <button className="rounded-lg p-2 text-(--app-primary) hover:bg-(--app-green-soft)"><Play className="size-4 fill-current" /></button>}
                            </div>
                        ))}
                    </div>
                </section>

                <section className="rounded-2xl border border-(--app-border) bg-(--app-warm) p-5 shadow-sm sm:p-6">
                    <div className="flex items-start justify-between">
                        <div>
                            <h2 className="text-lg font-semibold text-(--app-heading)">আপনার লক্ষ্য</h2>
                            <p className="mt-1 text-sm text-(--app-muted-strong)">এই সপ্তাহের অগ্রগতি</p>
                        </div>
                        <CalendarDays className="size-5 text-(--app-gold-text)" />
                    </div>
                    <div className="mt-7 flex items-center gap-5">
                        <div className="relative flex size-28 shrink-0 items-center justify-center rounded-full" style={{ background: "conic-gradient(var(--app-primary) 0 72%, var(--app-track) 72% 100%)" }}>
                            <div className="flex size-20 flex-col items-center justify-center rounded-full bg-(--app-warm)">
                                <span className="text-2xl font-semibold text-(--app-heading)">৭২%</span>
                                <span className="text-[10px] text-(--app-muted)">সম্পন্ন</span>
                            </div>
                        </div>
                        <div className="space-y-3 text-sm">
                            <div><p className="font-medium text-(--app-text)">লক্ষ্য: ২৫ ঘণ্টা</p><p className="text-xs text-(--app-muted)">আরও ৬.৫ ঘণ্টা বাকি</p></div>
                            <div><p className="font-medium text-(--app-text)">সময়সীমা: রবিবার</p><p className="text-xs text-(--app-muted)">আরও ৩ দিন বাকি</p></div>
                        </div>
                    </div>
                    <div className="mt-7 rounded-xl bg-(--app-soft) p-3.5 text-sm text-(--app-text-soft)">
                        <span className="font-semibold text-(--app-primary)">চমৎকার!</span> আপনি গত সপ্তাহের চেয়ে ১২% বেশি এগিয়েছেন।
                    </div>
                </section>
            </div>

            <section className="space-y-4">
                <div>
                    <h2 className="text-xl font-semibold text-(--app-heading)">আপনার প্রস্তুতি</h2>
                    <p className="mt-1 text-sm text-(--app-muted-strong)">আপনার ভবিষ্যৎ লক্ষ্য অনুযায়ী প্রস্তুতি নিন</p>
                </div>
                <div className="grid gap-4 md:grid-cols-2">
                    <article className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm">
                        <div className="flex items-start gap-4">
                            <div className="flex size-11 shrink-0 items-center justify-center rounded-xl bg-(--app-gold-soft) text-(--app-gold-text)">
                                <BriefcaseBusiness className="size-5" />
                            </div>
                            <div className="min-w-0 flex-1">
                                <h3 className="font-semibold text-(--app-heading)">চাকরির প্রস্তুতি</h3>
                                <p className="mt-1 text-sm leading-6 text-(--app-muted)">চাকরির পরীক্ষার জন্য দক্ষতা, সাধারণ জ্ঞান ও মডেল টেস্ট অনুশীলন করুন।</p>
                            </div>
                        </div>
                        <div className="mt-5 flex items-center justify-between">
                            <span className="text-xs font-medium text-(--app-muted-strong)">শুরু করার জন্য প্রস্তুত</span>
                            <Link href="/job-preparation" className="inline-flex items-center gap-1 text-sm font-medium text-(--app-gold-text) hover:text-(--app-gold-hover)">
                                দেখুন <ArrowLeft className="size-4" />
                            </Link>
                        </div>
                    </article>
                    <article className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm">
                        <div className="flex items-start gap-4">
                            <div className="flex size-11 shrink-0 items-center justify-center rounded-xl bg-(--app-blue-soft) text-(--app-blue)">
                                <GraduationCap className="size-5" />
                            </div>
                            <div className="min-w-0 flex-1">
                                <h3 className="font-semibold text-(--app-heading)">বিশ্ববিদ্যালয় প্রস্তুতি</h3>
                                <p className="mt-1 text-sm leading-6 text-(--app-muted)">ভর্তি পরীক্ষার বিষয়ভিত্তিক প্রস্তুতি, প্রশ্নব্যাংক ও নিয়মিত মক টেস্টে এগিয়ে থাকুন।</p>
                            </div>
                        </div>
                        <div className="mt-5 flex items-center justify-between">
                            <span className="text-xs font-medium text-(--app-muted-strong)">আপনার লক্ষ্য বেছে নিন</span>
                            <Link href="/university-admission" className="inline-flex items-center gap-1 text-sm font-medium text-(--app-blue) hover:text-(--app-primary)">
                                দেখুন <ArrowLeft className="size-4" />
                            </Link>
                        </div>
                    </article>
                </div>
            </section>
        </main>
    );
}
