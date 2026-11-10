"use client";

import Link from "next/link";
import { ArrowLeft, CheckCircle2, Clock3, FileQuestion, Flag, RotateCcw } from "lucide-react";
import { useEffect, useState } from "react";

const questions = [
    {
        question: "বাংলাদেশের জাতীয় ফুল কোনটি?",
        options: ["শাপলা", "গোলাপ", "জবা", "বেলি"],
        answer: "শাপলা",
    },
    {
        question: "‘আমার সোনার বাংলা’ গানের রচয়িতা কে?",
        options: ["কাজী নজরুল ইসলাম", "রবীন্দ্রনাথ ঠাকুর", "জসীমউদ্দীন", "মাইকেল মধুসূদন দত্ত"],
        answer: "রবীন্দ্রনাথ ঠাকুর",
    },
    {
        question: "2x + 6 = 14 হলে x-এর মান কত?",
        options: ["২", "৩", "৪", "৫"],
        answer: "৪",
    },
    {
        question: "বাংলাদেশের মুক্তিযুদ্ধ শুরু হয় কোন সালে?",
        options: ["১৯৫২", "১৯৬৯", "১৯৭১", "১৯৭৫"],
        answer: "১৯৭১",
    },
    {
        question: "‘Quickly’ শব্দটি কোন ধরনের শব্দ?",
        options: ["Noun", "Adjective", "Verb", "Adverb"],
        answer: "Adverb",
    },
];

export default function MockTestPage() {
    const [current, setCurrent] = useState(0);
    const [answers, setAnswers] = useState<Record<number, string>>({});
    const [secondsLeft, setSecondsLeft] = useState(15 * 60);
    const [submitted, setSubmitted] = useState(false);

    useEffect(() => {
        if (submitted || secondsLeft <= 0) return;
        const timer = window.setInterval(() => setSecondsLeft((seconds) => Math.max(0, seconds - 1)), 1000);
        return () => window.clearInterval(timer);
    }, [secondsLeft, submitted]);

    const question = questions[current];
    const answeredCount = Object.keys(answers).length;
    const score = questions.reduce((total, item, index) => total + (answers[index] === item.answer ? 1 : 0), 0);
    const minutes = Math.floor(secondsLeft / 60).toString().padStart(2, "0");
    const seconds = (secondsLeft % 60).toString().padStart(2, "0");

    const resetTest = () => {
        setCurrent(0);
        setAnswers({});
        setSecondsLeft(15 * 60);
        setSubmitted(false);
    };

    return (
        <main className="mx-auto w-full max-w-7xl space-y-6 pb-8">
            <section className="flex flex-col justify-between gap-5 rounded-3xl bg-(--app-hero) px-6 py-7 text-white sm:flex-row sm:items-end sm:px-10">
                <div>
                    <p className="text-sm text-(--app-hero-muted)">মক টেস্ট</p>
                    <h1 className="mt-2 text-3xl font-semibold tracking-tight">বাংলা ও সাধারণ জ্ঞান</h1>
                    <p className="mt-2 text-sm leading-6 text-(--app-hero-text)">সঠিক উত্তরটি নির্বাচন করুন।</p>
                </div>
                <div className={`flex items-center gap-2 rounded-xl px-4 py-3 text-lg font-semibold ${secondsLeft < 60 ? "bg-(--app-coral-soft) text-(--app-coral)" : "bg-(--app-card)/10 text-(--app-gold-pale)"}`}>
                    <Clock3 className="size-4" /> {minutes}:{seconds}
                </div>
            </section>

            {submitted ? (
                <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-6 text-center shadow-sm sm:p-8">
                    <div className="mx-auto flex size-12 items-center justify-center rounded-full bg-(--app-green-soft) text-(--app-primary)"><CheckCircle2 className="size-6" /></div>
                    <p className="mt-3 text-xs text-(--app-muted)">মক টেস্ট সম্পন্ন হয়েছে</p>
                    <h2 className="mt-1 text-3xl font-semibold text-(--app-heading)">{score} / {questions.length}</h2>
                    <p className="mt-1 text-xs text-(--app-muted-strong)">{Math.round((score / questions.length) * 100)}% সঠিক · {answeredCount}টি উত্তর</p>
                    <div className="mx-auto mt-5 flex max-w-sm gap-2"><button onClick={resetTest} className="inline-flex flex-1 items-center justify-center gap-1 rounded-lg border border-(--app-border-strong) px-3 py-2 text-xs font-medium text-(--app-text)"><RotateCcw className="size-3.5" /> আবার দিন</button><Link href="/job-preparation" className="flex flex-1 items-center justify-center rounded-lg bg-(--app-primary) px-3 py-2 text-xs font-medium text-white">মূল্যায়ন দেখুন</Link></div>
                </section>
            ) : (
                <div className="grid gap-3 lg:grid-cols-[1fr_220px]">
                    <section className="rounded-2xl border border-(--app-border) bg-(--app-card) p-4 shadow-sm sm:p-5">
                        <div className="flex items-center justify-between border-b border-(--app-border-soft) pb-3">
                            <div><p className="text-xs text-(--app-muted)">প্রশ্ন {current + 1} / {questions.length}</p><div className="mt-2 h-1.5 w-32 overflow-hidden rounded-full bg-(--app-track) sm:w-52"><div className="h-full rounded-full bg-(--app-primary) transition-all" style={{ width: `${((current + 1) / questions.length) * 100}%` }} /></div></div>
                            <span className="rounded-full bg-(--app-green-soft) px-2 py-1 text-[10px] font-medium text-(--app-primary)">{answeredCount}টি উত্তর</span>
                        </div>
                        <div className="py-5"><h2 className="text-lg font-semibold leading-7 text-(--app-heading)">{question.question}</h2><div className="mt-4 grid gap-2">{question.options.map((option, index) => { const selected = answers[current] === option; return <button key={option} onClick={() => setAnswers((previous) => ({ ...previous, [current]: option }))} className={`flex items-center gap-2 rounded-lg border p-2.5 text-right text-xs transition ${selected ? "border-(--app-primary) bg-(--app-green-soft) text-(--app-primary)" : "border-(--app-border) text-(--app-text) hover:border-(--app-primary) hover:bg-(--app-soft)"}`}><span className={`flex size-6 shrink-0 items-center justify-center rounded-full text-[10px] font-semibold ${selected ? "bg-(--app-primary) text-white" : "bg-(--app-soft) text-(--app-muted-strong)"}`}>{String.fromCharCode(65 + index)}</span>{option}</button>; })}</div></div>
                        <div className="flex items-center justify-between border-t border-(--app-border-soft) pt-3"><button disabled={current === 0} onClick={() => setCurrent((value) => value - 1)} className="rounded-lg border border-(--app-border-strong) px-3 py-2 text-xs font-medium text-(--app-text) disabled:cursor-not-allowed disabled:opacity-40">আগের</button>{current === questions.length - 1 ? <button onClick={() => setSubmitted(true)} className="rounded-lg bg-(--app-primary) px-4 py-2 text-xs font-medium text-white hover:bg-(--app-primary-hover)">জমা দিন</button> : <button onClick={() => setCurrent((value) => value + 1)} className="rounded-lg bg-(--app-primary) px-4 py-2 text-xs font-medium text-white hover:bg-(--app-primary-hover)">পরের</button>}</div>
                    </section>

                    <aside className="rounded-2xl border border-(--app-border) bg-(--app-warm) p-4 shadow-sm"><div className="flex items-center gap-2"><FileQuestion className="size-4 text-(--app-primary)" /><h2 className="text-sm font-semibold text-(--app-heading)">প্রশ্ন তালিকা</h2></div><div className="mt-3 grid grid-cols-5 gap-1.5 lg:grid-cols-4">{questions.map((_, index) => <button key={index} onClick={() => setCurrent(index)} className={`flex size-8 items-center justify-center rounded-md text-xs font-medium ${current === index ? "bg-(--app-primary) text-white" : answers[index] ? "bg-(--app-green-soft) text-(--app-primary)" : "bg-(--app-card) text-(--app-muted-strong) ring-1 ring-(--app-border)"}`}>{index + 1}</button>)}</div><div className="mt-4 space-y-2 border-t border-(--app-border-warm) pt-3 text-[10px] text-(--app-muted-strong)"><p className="flex items-center gap-2"><span className="size-2.5 rounded bg-(--app-primary)" /> বর্তমান</p><p className="flex items-center gap-2"><span className="size-2.5 rounded bg-(--app-green-soft)" /> উত্তর দেওয়া</p><p className="flex items-center gap-2"><span className="size-2.5 rounded bg-(--app-card) ring-1 ring-(--app-border)" /> বাকি</p></div><button className="mt-4 flex w-full items-center justify-center gap-1 rounded-lg border border-(--app-border-strong) px-2 py-2 text-[10px] font-medium text-(--app-text)"><Flag className="size-3" /> রিপোর্ট</button></aside>
                </div>
            )}

            <Link href="/" className="inline-flex items-center gap-1 text-sm font-medium text-(--app-primary)"><ArrowLeft className="size-4" /> ড্যাশবোর্ডে ফিরে যান</Link>
        </main>
    );
}
