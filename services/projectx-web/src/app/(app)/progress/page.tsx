import Link from "next/link";
import { ArrowLeft, BarChart3, BookOpen, Clock3, Flame, Trophy } from "lucide-react";

const stats = [
    { label: "সাপ্তাহিক অগ্রগতি", value: "৭২%", detail: "+১২% গত সপ্তাহ থেকে", icon: BarChart3, tone: "bg-(--app-green-soft) text-(--app-primary)" },
    { label: "পড়ার সময়", value: "১৮.৫", detail: "ঘণ্টা এই সপ্তাহে", icon: Clock3, tone: "bg-(--app-gold-soft) text-(--app-gold-text)" },
    { label: "অবিচ্ছিন্ন দিন", value: "১২ দিন", detail: "আপনার সেরা: ১৮ দিন", icon: Flame, tone: "bg-(--app-coral-soft) text-(--app-coral)" },
    { label: "সম্পন্ন অধ্যায়", value: "২৪", detail: "মোট ৩৬টি অধ্যায়", icon: BookOpen, tone: "bg-(--app-blue-soft) text-(--app-blue)" },
];

const weekly = [
    { day: "শনি", value: 55 }, { day: "রবি", value: 72 }, { day: "সোম", value: 42 },
    { day: "মঙ্গল", value: 88 }, { day: "বুধ", value: 64 }, { day: "বৃহস্পতি", value: 78 }, { day: "শুক্র", value: 35 },
];

export default function ProgressPage() {
    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="flex flex-col justify-between gap-5 rounded-3xl bg-(--app-hero) px-6 py-7 text-white sm:flex-row sm:items-end sm:px-10">
                <div><p className="text-sm text-(--app-hero-muted)">আপনার শেখার যাত্রা</p><h1 className="mt-2 text-3xl font-semibold tracking-tight">অগ্রগতি</h1><p className="mt-2 text-sm text-(--app-hero-text)">আপনার ধারাবাহিকতা এবং অর্জনগুলো এক নজরে দেখুন।</p></div>
                <div className="flex items-center gap-2 rounded-xl bg-(--app-card)/10 px-3 py-2 text-sm text-(--app-hero-muted)"><BarChart3 className="size-4 text-(--app-gold)" /> এই সপ্তাহ</div>
            </section>
            <section className="grid gap-4 sm:grid-cols-2 lg:grid-cols-4">
                {stats.map((stat) => { const Icon = stat.icon; return <div key={stat.label} className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm"><div className="flex items-start justify-between"><span className="text-sm text-(--app-muted-strong)">{stat.label}</span><span className={`rounded-xl p-2 ${stat.tone}`}><Icon className="size-4" /></span></div><p className="mt-5 text-2xl font-semibold tracking-tight text-(--app-heading)">{stat.value}</p><p className="mt-1 text-xs text-(--app-muted)">{stat.detail}</p></div>; })}
            </section>
            <div className="grid gap-6 lg:grid-cols-[1.4fr_1fr]">
                <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm sm:p-6"><div className="flex items-center justify-between"><div><h2 className="text-lg font-semibold text-(--app-heading)">সাপ্তাহিক পড়াশোনা</h2><p className="mt-1 text-sm text-(--app-muted)">প্রতিদিনের লক্ষ্যের তুলনায়</p></div><span className="rounded-lg bg-(--app-soft) px-3 py-1.5 text-xs font-medium text-(--app-primary)">এই সপ্তাহ</span></div><div className="mt-8 flex h-56 items-end justify-between gap-2 border-b border-(--app-border-soft) px-1">{weekly.map((item, index) => <div key={item.day} className="flex h-full flex-1 flex-col items-center justify-end gap-2"><div className={`w-full max-w-10 rounded-t-lg ${index === 3 ? "bg-(--app-primary)" : "bg-(--app-green-mid)"}`} style={{ height: `${item.value}%` }} /><span className="text-[11px] text-(--app-muted)">{item.day}</span></div>)}</div><div className="mt-5 flex items-center justify-between text-xs text-(--app-muted)"><span>০ ঘণ্টা</span><span>দৈনিক লক্ষ্য: ৩ ঘণ্টা</span><span>৩ ঘণ্টা+</span></div></section>
                <section className="rounded-2xl border border-(--app-border) bg-(--app-warm) p-5 shadow-sm sm:p-6"><div className="flex items-center gap-3"><div className="flex size-11 items-center justify-center rounded-2xl bg-(--app-gold-pale) text-(--app-gold-text)"><Trophy className="size-5" /></div><div><h2 className="font-semibold text-(--app-heading)">আপনার অর্জন</h2><p className="mt-1 text-xs text-(--app-muted)">এগিয়ে যাওয়ার ছোট স্বীকৃতি</p></div></div><div className="mt-6 space-y-3">{[["প্রথম পদক্ষেপ", "প্রথম অধ্যায় শেষ করেছেন"], ["ধারাবাহিক শিক্ষার্থী", "টানা ৭ দিন পড়াশোনা"], ["দ্রুত অগ্রগতি", "এক সপ্তাহে ৫টি অধ্যায়"]].map(([title, detail]) => <div key={title} className="flex items-center gap-3 rounded-xl border border-(--app-border-warm) bg-(--app-card) p-3"><span className="flex size-9 items-center justify-center rounded-full bg-(--app-gold-soft) text-lg">★</span><div><p className="text-sm font-medium text-(--app-text)">{title}</p><p className="mt-0.5 text-xs text-(--app-muted)">{detail}</p></div></div>)}</div><Link href="/subjects" className="mt-5 inline-flex items-center gap-1 text-sm font-medium text-(--app-primary)">সব বিষয় দেখুন <ArrowLeft className="size-4" /></Link></section>
            </div>
            <section className="rounded-2xl bg-(--app-hero) p-6 text-white sm:flex sm:items-center sm:justify-between"><div><p className="text-sm text-(--app-hero-muted)">এই সপ্তাহের পরামর্শ</p><h2 className="mt-2 text-xl font-semibold">আরও একটু নিয়মিত হলেই লক্ষ্য পূরণ হবে!</h2><p className="mt-2 text-sm text-(--app-hero-text)">গত সপ্তাহের চেয়ে আপনি ইতিমধ্যে ১২% এগিয়ে আছেন।</p></div><Link href="/routine" className="mt-5 inline-flex items-center gap-2 rounded-xl bg-(--app-gold) px-4 py-2.5 text-sm font-semibold text-(--app-hero) sm:mt-0">রুটিন দেখুন <ArrowLeft className="size-4" /></Link></section>
        </main>
    );
}
