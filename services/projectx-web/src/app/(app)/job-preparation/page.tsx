import Link from "next/link";
import { ArrowLeft, BarChart3, BriefcaseBusiness, CheckCircle2, Clock3, Play, Target } from "lucide-react";

const tests = [
    { title: "বাংলা ও সাধারণ জ্ঞান", questions: "৫০টি প্রশ্ন", duration: "৪৫ মিনিট", level: "মাঝারি", tone: "bg-(--app-green-soft) text-(--app-primary)" },
    { title: "গণিত ও মানসিক দক্ষতা", questions: "৪০টি প্রশ্ন", duration: "৪০ মিনিট", level: "কঠিন", tone: "bg-(--app-gold-soft) text-(--app-gold-text)" },
    { title: "ইংরেজি ও কম্পিউটার", questions: "৫০টি প্রশ্ন", duration: "৪৫ মিনিট", level: "মাঝারি", tone: "bg-(--app-blue-soft) text-(--app-blue)" },
];

const evaluations = [
    { title: "সাপ্তাহিক মডেল টেস্ট", date: "১০ নভেম্বর, ২০২৬", score: "৭৮%", correct: "৩৯ / ৫০", tone: "text-(--app-primary)" },
    { title: "সাধারণ জ্ঞান অনুশীলন", date: "৮ নভেম্বর, ২০২৬", score: "৮৬%", correct: "৪৩ / ৫০", tone: "text-(--app-gold-text)" },
    { title: "ইংরেজি গ্রামার টেস্ট", date: "৫ নভেম্বর, ২০২৬", score: "৬৮%", correct: "৩৪ / ৫০", tone: "text-(--app-blue)" },
];

export default function JobPreparationPage() {
    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="flex flex-col justify-between gap-5 rounded-3xl bg-(--app-hero) px-6 py-7 text-white sm:flex-row sm:items-end sm:px-10">
                <div>
                    <p className="text-sm text-(--app-hero-muted)">চাকরির প্রস্তুতি</p>
                    <h1 className="mt-2 text-3xl font-semibold tracking-tight">MCQ পরীক্ষা ও মূল্যায়ন</h1>
                    <p className="mt-2 max-w-xl text-sm leading-6 text-(--app-hero-text)">নিয়মিত মডেল টেস্ট দিন, ফলাফল দেখুন এবং আপনার দুর্বল বিষয়গুলোতে আরও অনুশীলন করুন।</p>
                </div>
                <div className="flex items-center gap-2 rounded-xl bg-(--app-card)/10 px-3 py-2 text-sm text-(--app-green-pale)">
                    <BriefcaseBusiness className="size-4 text-(--app-gold)" /> ১২টি পরীক্ষা
                </div>
            </section>

            <section className="grid gap-4 sm:grid-cols-3">
                {[
                    { label: "গড় স্কোর", value: "৭৬%", icon: Target, tone: "bg-(--app-green-soft) text-(--app-primary)" },
                    { label: "দেওয়া পরীক্ষা", value: "১৮টি", icon: CheckCircle2, tone: "bg-(--app-gold-soft) text-(--app-gold-text)" },
                    { label: "মোট সময়", value: "১২.৫ ঘণ্টা", icon: Clock3, tone: "bg-(--app-blue-soft) text-(--app-blue)" },
                ].map((stat) => {
                    const Icon = stat.icon;
                    return <div key={stat.label} className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm"><div className="flex items-center justify-between"><span className="text-sm text-(--app-muted-strong)">{stat.label}</span><span className={`rounded-xl p-2 ${stat.tone}`}><Icon className="size-4" /></span></div><p className="mt-4 text-2xl font-semibold text-(--app-heading)">{stat.value}</p></div>;
                })}
            </section>

            <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm sm:p-6">
                <div className="flex items-center justify-between"><div><h2 className="text-lg font-semibold text-(--app-heading)">MCQ পরীক্ষা</h2><p className="mt-1 text-sm text-(--app-muted)">আপনার পছন্দের বিষয় থেকে একটি পরীক্ষা শুরু করুন</p></div><Play className="size-5 text-(--app-primary)" /></div>
                <div className="mt-5 grid gap-3 md:grid-cols-3">
                    {tests.map((test) => <article key={test.title} className="rounded-xl border border-(--app-border-soft) p-4"><div className={`mb-4 flex size-10 items-center justify-center rounded-xl ${test.tone}`}><BarChart3 className="size-5" /></div><h3 className="font-semibold text-(--app-heading)">{test.title}</h3><div className="mt-2 flex gap-3 text-xs text-(--app-muted)"><span>{test.questions}</span><span>{test.duration}</span></div><div className="mt-4 flex items-center justify-between"><span className="rounded-full bg-(--app-soft) px-2.5 py-1 text-xs text-(--app-muted-strong)">{test.level}</span><Link href="/mock-test" className="rounded-lg bg-(--app-primary) px-3 py-2 text-xs font-medium text-white hover:bg-(--app-primary-hover)">পরীক্ষা দিন</Link></div></article>)}
                </div>
            </section>

            <section className="rounded-2xl border border-(--app-border) bg-(--app-warm) p-5 shadow-sm sm:p-6">
                <div className="flex items-center justify-between"><div><h2 className="text-lg font-semibold text-(--app-heading)">পরীক্ষার মূল্যায়ন</h2><p className="mt-1 text-sm text-(--app-muted)">আপনার সাম্প্রতিক ফলাফল ও অগ্রগতি</p></div><BarChart3 className="size-5 text-(--app-primary)" /></div>
                <div className="mt-5 space-y-3">{evaluations.map((evaluation) => <div key={evaluation.title} className="flex flex-wrap items-center gap-3 rounded-xl border border-(--app-border-warm) bg-(--app-card) p-4"><div className="flex size-9 items-center justify-center rounded-full bg-(--app-green-soft) text-(--app-primary)"><CheckCircle2 className="size-4" /></div><div className="min-w-40 flex-1"><p className="text-sm font-medium text-(--app-text)">{evaluation.title}</p><p className="mt-1 text-xs text-(--app-muted)">{evaluation.date} · {evaluation.correct} সঠিক</p></div><span className={`text-lg font-semibold ${evaluation.tone}`}>{evaluation.score}</span><button className="rounded-lg border border-(--app-border-strong) px-3 py-2 text-xs font-medium text-(--app-text)">বিস্তারিত</button></div>)}</div>
            </section>

            <Link href="/" className="inline-flex items-center gap-1 text-sm font-medium text-(--app-primary)"><ArrowLeft className="size-4" /> ড্যাশবোর্ডে ফিরে যান</Link>
        </main>
    );
}
