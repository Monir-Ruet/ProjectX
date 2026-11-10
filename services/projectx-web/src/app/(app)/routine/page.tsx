import { Check, ChevronLeft, Clock3, Plus, Target } from "lucide-react";

const days = ["শনি", "রবি", "সোম", "মঙ্গল", "বুধ", "বৃহস্পতি", "শুক্র"];
const lessons = [
    { subject: "বাংলা সাহিত্য", title: "রবীন্দ্রনাথ ঠাকুর — ছোটগল্প", time: "সকাল ৯:০০", duration: "৪৫ মিনিট", done: true, color: "bg-(--app-green-soft) text-(--app-primary)" },
    { subject: "সাধারণ গণিত", title: "বীজগণিতের মৌলিক ধারণা", time: "দুপুর ১২:৩০", duration: "৬০ মিনিট", done: false, color: "bg-(--app-gold-soft) text-(--app-gold-text)" },
    { subject: "ইংরেজি ব্যাকরণ", title: "কাল ও বাক্য গঠন", time: "সন্ধ্যা ৭:০০", duration: "৩০ মিনিট", done: false, color: "bg-(--app-blue-soft) text-(--app-blue)" },
];

export default function RoutinePage() {
    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="flex flex-col justify-between gap-4 rounded-3xl bg-(--app-hero) px-6 py-7 text-white sm:flex-row sm:items-end sm:px-10">
                <div><p className="text-sm text-(--app-hero-muted)">আপনার পরিকল্পনা</p><h1 className="mt-2 text-3xl font-semibold tracking-tight">পড়াশোনার রুটিন</h1><p className="mt-2 text-sm text-(--app-hero-text)">ধারাবাহিক থাকুন, নিজের লক্ষ্যে পৌঁছান।</p></div>
                <button className="inline-flex items-center justify-center gap-2 rounded-xl bg-(--app-gold) px-4 py-2.5 text-sm font-semibold text-(--app-hero) shadow-sm hover:bg-(--app-gold-hover)"><Plus className="size-4" /> রুটিন যোগ করুন</button>
            </section>
            <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-4 shadow-sm sm:p-5">
                <div className="mb-5 flex items-center justify-between"><button className="rounded-lg p-2 text-(--app-muted-strong) hover:bg-(--app-soft)"><ChevronLeft className="size-5" /></button><div className="text-center"><h2 className="font-semibold text-(--app-text)">এই সপ্তাহ</h2><p className="mt-1 text-xs text-(--app-muted)">১০–১৬ জুন, ২০২৪</p></div><button className="rounded-lg p-2 text-(--app-muted-strong) hover:bg-(--app-soft)"><ChevronLeft className="size-5 rotate-180" /></button></div>
                <div className="grid grid-cols-7 gap-1 sm:gap-3">{days.map((day, index) => <div key={day} className={`rounded-xl py-3 text-center ${index === 2 ? "bg-(--app-primary) text-white shadow-sm" : "text-(--app-muted)"}`}><p className="text-[11px]">{day}</p><p className={`mt-2 text-lg font-semibold ${index === 2 ? "text-white" : "text-(--app-text)"}`}>{13 + index}</p>{index === 2 && <span className="mx-auto mt-2 block size-1 rounded-full bg-(--app-gold)" />}</div>)}</div>
            </section>
            <div className="grid gap-6 lg:grid-cols-[1.35fr_1fr]">
                <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm sm:p-6"><div className="flex items-start justify-between"><div><h2 className="text-lg font-semibold text-(--app-heading)">আজকের পড়াশোনা</h2><p className="mt-1 text-sm text-(--app-muted)">বুধবার, ১২ জুন</p></div><span className="rounded-lg bg-(--app-green-soft) px-2.5 py-1.5 text-xs font-medium text-(--app-primary)">১ / ৩ সম্পন্ন</span></div><div className="mt-6 space-y-3">{lessons.map((lesson) => <div key={lesson.title} className="flex items-center gap-3 rounded-xl border border-(--app-border-soft) p-3.5"><div className={`flex size-10 shrink-0 items-center justify-center rounded-full ${lesson.done ? "bg-(--app-primary) text-white" : lesson.color}`}><Check className={`size-4 ${lesson.done ? "" : "opacity-30"}`} /></div><div className="min-w-0 flex-1"><p className={`text-sm font-medium ${lesson.done ? "text-(--app-muted) line-through" : "text-(--app-text)"}`}>{lesson.subject}</p><p className="mt-1 truncate text-xs text-(--app-muted)">{lesson.title}</p></div><div className="hidden text-left sm:block"><p className="text-xs font-medium text-(--app-muted-strong)">{lesson.time}</p><p className="mt-1 text-[11px] text-(--app-muted-light)">{lesson.duration}</p></div></div>)}</div></section>
                <section className="rounded-2xl bg-(--app-hero) p-6 text-white shadow-sm"><div className="flex size-11 items-center justify-center rounded-2xl bg-(--app-card)/10 text-(--app-gold)"><Target className="size-5" /></div><h2 className="mt-5 text-xl font-semibold">আজকের লক্ষ্য</h2><p className="mt-2 text-sm leading-6 text-(--app-hero-text)">আরও ২টি সেশন শেষ করলে আপনার দৈনিক লক্ষ্য পূরণ হবে।</p><div className="mt-6 h-2 rounded-full bg-(--app-card)/15"><div className="h-full w-1/3 rounded-full bg-(--app-gold)" /></div><div className="mt-3 flex justify-between text-xs text-(--app-hero-text)"><span>১ ঘণ্টা ১৫ মিনিট পড়া হয়েছে</span><span>৩ ঘণ্টা</span></div><button className="mt-7 inline-flex items-center gap-2 rounded-xl bg-(--app-gold) px-4 py-2.5 text-sm font-semibold text-(--app-hero)">পরবর্তী সেশন <Clock3 className="size-4" /></button></section>
            </div>
        </main>
    );
}
