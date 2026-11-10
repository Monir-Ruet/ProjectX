import Link from "next/link";
import { ArrowLeft, BookOpen, CheckCircle2, ChevronLeft, Clock3, Play, Search } from "lucide-react";

const subjects = [
    { title: "বাংলা", description: "সাহিত্য, ব্যাকরণ ও সৃজনশীল লেখা", lessons: "১২ / ১৬ অধ্যায়", progress: 75, tone: "bg-(--app-green-soft) text-(--app-primary)", bar: "bg-(--app-primary)" },
    { title: "সাধারণ গণিত", description: "বীজগণিত, জ্যামিতি ও সংখ্যাতত্ত্ব", lessons: "৮ / ১৪ অধ্যায়", progress: 57, tone: "bg-(--app-gold-soft) text-(--app-gold-text)", bar: "bg-(--app-gold)" },
    { title: "ইংরেজি", description: "ব্যাকরণ, শব্দভাণ্ডার ও রচনা", lessons: "১০ / ১৮ অধ্যায়", progress: 56, tone: "bg-(--app-blue-soft) text-(--app-blue)", bar: "bg-(--app-blue)" },
    { title: "বিজ্ঞান", description: "পদার্থ, রসায়ন ও জীববিজ্ঞানের ভিত্তি", lessons: "৬ / ১৫ অধ্যায়", progress: 40, tone: "bg-(--app-coral-soft) text-(--app-coral)", bar: "bg-(--app-coral)" },
];

export default function SubjectsPage() {
    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="flex flex-col justify-between gap-5 rounded-3xl bg-(--app-hero) px-6 py-7 text-white sm:flex-row sm:items-end sm:px-10">
                <div>
                    <p className="text-sm text-(--app-hero-muted)">আপনার পাঠ্যক্রম</p>
                    <h1 className="mt-2 text-3xl font-semibold tracking-tight">বিষয়সমূহ</h1>
                    <p className="mt-2 text-sm text-(--app-hero-text)">আপনার শেখার সব বিষয় এক জায়গায়।</p>
                </div>
                <div className="flex items-center gap-2 rounded-xl bg-(--app-card)/10 px-3 py-2 text-sm text-(--app-green-pale)">
                    <BookOpen className="size-4 text-(--app-gold)" /> ৪টি বিষয়
                </div>
            </section>

            <div className="flex flex-col gap-3 sm:flex-row">
                <div className="flex flex-1 items-center gap-3 rounded-xl border border-(--app-border) bg-(--app-card) px-4 py-3 text-sm text-(--app-muted)">
                    <Search className="size-4" />
                    <span>বিষয় বা অধ্যায় খুঁজুন</span>
                </div>
                <button className="rounded-xl border border-(--app-border-strong) bg-(--app-card) px-4 py-3 text-sm font-medium text-(--app-text)">সব বিষয় <ChevronLeft className="mr-1 inline size-4" /></button>
            </div>

            <section className="grid gap-4 md:grid-cols-2">
                {subjects.map((subject) => (
                    <article key={subject.title} className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm transition hover:-translate-y-0.5 hover:shadow-md">
                        <div className="flex items-start gap-4">
                            <div className={`flex size-12 shrink-0 items-center justify-center rounded-2xl ${subject.tone}`}><BookOpen className="size-5" /></div>
                            <div className="min-w-0 flex-1">
                                <div className="flex items-start justify-between gap-3">
                                    <div><h2 className="font-semibold text-(--app-heading)">{subject.title}</h2><p className="mt-1 text-xs text-(--app-muted)">{subject.description}</p></div>
                                    <span className="text-sm font-semibold text-(--app-text)">{subject.progress}%</span>
                                </div>
                                <div className="mt-5 h-2 overflow-hidden rounded-full bg-(--app-track)"><div className={`h-full rounded-full ${subject.bar}`} style={{ width: `${subject.progress}%` }} /></div>
                                <div className="mt-3 flex items-center justify-between text-xs text-(--app-muted)"><span>{subject.lessons}</span><Link href="/routine" className="font-medium text-(--app-primary)">চালিয়ে যান <ArrowLeft className="mr-1 inline size-3" /></Link></div>
                            </div>
                        </div>
                    </article>
                ))}
            </section>

            <section className="rounded-2xl border border-(--app-border) bg-(--app-warm) p-5 sm:p-6">
                <div className="flex items-center justify-between"><div><h2 className="font-semibold text-(--app-heading)">সাম্প্রতিক অধ্যায়</h2><p className="mt-1 text-sm text-(--app-muted)">যেখান থেকে শেষ করেছিলেন সেখান থেকেই শুরু করুন</p></div><CheckCircle2 className="size-5 text-(--app-primary)" /></div>
                <div className="mt-5 grid gap-3 sm:grid-cols-2">
                    {[["বাংলা সাহিত্য", "রবীন্দ্রনাথ ঠাকুর — ছোটগল্প"], ["সাধারণ গণিত", "বীজগণিতের মৌলিক ধারণা"]].map(([title, subtitle]) => (
                        <div key={title} className="flex items-center gap-3 rounded-xl border border-(--app-border-warm) bg-(--app-card) p-3.5"><div className="flex size-9 items-center justify-center rounded-full bg-(--app-green-soft) text-(--app-primary)"><Play className="size-4 fill-current" /></div><div className="flex-1"><p className="text-sm font-medium text-(--app-text)">{title}</p><p className="mt-1 text-xs text-(--app-muted)">{subtitle}</p></div><Clock3 className="size-4 text-(--app-muted-light)" /></div>
                    ))}
                </div>
            </section>
        </main>
    );
}
