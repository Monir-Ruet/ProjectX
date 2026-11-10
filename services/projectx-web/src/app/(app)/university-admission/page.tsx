import Link from "next/link";
import { ArrowLeft, BookOpen, GraduationCap, Layers3, Play, School } from "lucide-react";

const examTypes = [
    { title: "BKRUET ভর্তি পরীক্ষা", description: "BUET, KUET, RUET ও CUET-এর প্রকৌশল ভর্তি প্রস্তুতি", subjects: "পদার্থবিজ্ঞান · রসায়ন · গণিত", tests: "২৪টি মডেল টেস্ট", tone: "bg-(--app-gold-soft) text-(--app-gold-text)" },
    { title: "সকল বিশ্ববিদ্যালয়", description: "দেশের বিভিন্ন পাবলিক বিশ্ববিদ্যালয়ের সাধারণ ভর্তি প্রস্তুতি", subjects: "বাংলা · ইংরেজি · সাধারণ জ্ঞান", tests: "৩০টি মডেল টেস্ট", tone: "bg-(--app-green-soft) text-(--app-primary)" },
    { title: "গুচ্ছ বিশ্ববিদ্যালয়", description: "GST গুচ্ছ ভর্তি পরীক্ষার জন্য বিষয়ভিত্তিক অনুশীলন", subjects: "পদার্থ · রসায়ন · জীববিজ্ঞান", tests: "১৮টি মডেল টেস্ট", tone: "bg-(--app-blue-soft) text-(--app-blue)" },
];

export default function UniversityAdmissionPage() {
    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="flex flex-col justify-between gap-5 rounded-3xl bg-(--app-hero) px-6 py-7 text-white sm:flex-row sm:items-end sm:px-10">
                <div>
                    <p className="text-sm text-(--app-hero-muted)">বিশ্ববিদ্যালয় ভর্তি প্রস্তুতি</p>
                    <h1 className="mt-2 text-3xl font-semibold tracking-tight">আপনার ভর্তি পরীক্ষা বেছে নিন</h1>
                    <p className="mt-2 max-w-xl text-sm leading-6 text-(--app-hero-text)">লক্ষ্য অনুযায়ী প্রস্তুতি নিন, বিষয়ভিত্তিক পরীক্ষা দিন এবং ভর্তি পরীক্ষার জন্য নিজেকে যাচাই করুন।</p>
                </div>
                <div className="flex items-center gap-2 rounded-xl bg-(--app-card)/10 px-3 py-2 text-sm text-(--app-green-pale)"><GraduationCap className="size-4 text-(--app-gold)" /> ৩টি ক্যাটাগরি</div>
            </section>

            <section className="grid gap-4 md:grid-cols-3">
                {examTypes.map((exam) => <article key={exam.title} className="rounded-2xl border border-(--app-border) bg-(--app-card) p-5 shadow-sm"><div className={`flex size-12 items-center justify-center rounded-2xl ${exam.tone}`}><School className="size-6" /></div><h2 className="mt-5 text-lg font-semibold text-(--app-heading)">{exam.title}</h2><p className="mt-2 min-h-12 text-sm leading-6 text-(--app-muted)">{exam.description}</p><div className="mt-5 space-y-2 border-t border-(--app-border-soft) pt-4 text-xs text-(--app-muted-strong)"><p className="flex items-center gap-2"><BookOpen className="size-4" /> {exam.subjects}</p><p className="flex items-center gap-2"><Layers3 className="size-4" /> {exam.tests}</p></div><button className="mt-5 flex w-full items-center justify-center gap-2 rounded-xl bg-(--app-primary) px-4 py-2.5 text-sm font-medium text-white hover:bg-(--app-primary-hover)"><Play className="size-4 fill-current" /> প্রস্তুতি শুরু করুন</button></article>)}
            </section>

            <section className="rounded-2xl border border-(--app-border) bg-(--app-warm) p-5 sm:p-6"><div className="flex items-center justify-between"><div><h2 className="text-lg font-semibold text-(--app-heading)">আপনার ভর্তি প্রস্তুতির অগ্রগতি</h2><p className="mt-1 text-sm text-(--app-muted)">একটি ক্যাটাগরি বেছে নিয়ে আপনার প্রস্তুতি শুরু করুন</p></div><GraduationCap className="size-5 text-(--app-primary)" /></div><div className="mt-5 grid gap-3 sm:grid-cols-3">{["BKRUET", "সকল বিশ্ববিদ্যালয়", "গুচ্ছ বিশ্ববিদ্যালয়"].map((label, index) => <div key={label} className="rounded-xl border border-(--app-border-warm) bg-(--app-card) p-4"><div className="flex justify-between text-sm"><span className="font-medium text-(--app-text)">{label}</span><span className="text-(--app-primary)">{[32, 18, 8][index]}%</span></div><div className="mt-3 h-2 rounded-full bg-(--app-track)"><div className="h-full rounded-full bg-(--app-primary)" style={{ width: `${[32, 18, 8][index]}%` }} /></div></div>)}</div></section>

            <Link href="/" className="inline-flex items-center gap-1 text-sm font-medium text-(--app-primary)"><ArrowLeft className="size-4" /> ড্যাশবোর্ডে ফিরে যান</Link>
        </main>
    );
}
