import Link from "next/link";
import { ArrowLeft, GraduationCap } from "lucide-react";

const sections = [
    ["সেবা ব্যবহার", "পাঠশালা আপনার পড়াশোনা, মক টেস্ট এবং প্রস্তুতি পরিচালনার জন্য তৈরি। অ্যাকাউন্টের তথ্য সঠিক রাখা এবং নিজের অ্যাকাউন্ট নিরাপদ রাখা আপনার দায়িত্ব।"],
    ["শিক্ষামূলক কনটেন্ট", "এখানের কনটেন্ট অনুশীলন ও প্রস্তুতির সহায়ক হিসেবে দেওয়া হয়। পরীক্ষার ফলাফল বা ভর্তি নিশ্চিত করার কোনো নিশ্চয়তা এটি দেয় না।"],
    ["গ্রহণযোগ্য ব্যবহার", "সেবা নষ্ট করা, অন্যের অ্যাকাউন্টে প্রবেশের চেষ্টা করা, কনটেন্ট অনুমতি ছাড়া কপি বা বিতরণ করা যাবে না।"],
    ["অ্যাকাউন্ট বন্ধ", "নিয়ম ভঙ্গ হলে আমরা সাময়িকভাবে বা স্থায়ীভাবে অ্যাকাউন্ট সীমিত করতে পারি। আপনি চাইলে অ্যাকাউন্ট ব্যবহারের সমাপ্তি জানাতে পারেন।"],
];

export default function TermsPage() {
    return (
        <main className="min-h-svh bg-(--app-bg) px-4 py-8 sm:px-6">
            <article className="mx-auto max-w-2xl rounded-3xl border border-(--app-border) bg-(--app-card) p-6 shadow-sm sm:p-10">
                <Link href="/signin" className="inline-flex items-center gap-1 text-xs font-medium text-(--app-primary)"><ArrowLeft className="size-3.5" /> ফিরে যান</Link>
                <div className="mt-8 flex items-center gap-3"><span className="flex size-10 items-center justify-center rounded-xl bg-(--app-hero) text-(--app-gold)"><GraduationCap className="size-5" /></span><span className="font-bold text-(--app-heading)">পাঠশালা</span></div>
                <p className="mt-8 text-xs font-medium text-(--app-muted)">সর্বশেষ আপডেট: ১১ নভেম্বর, ২০২৬</p>
                <h1 className="mt-2 text-3xl font-semibold tracking-tight text-(--app-heading)">শর্তাবলি</h1>
                <p className="mt-3 text-sm leading-6 text-(--app-muted-strong)">পাঠশালা ব্যবহার করার আগে এই শর্তগুলো পড়ে নিন। সেবা ব্যবহার করলে আপনি এগুলোতে সম্মত হচ্ছেন।</p>
                <div className="mt-8 space-y-6">
                    {sections.map(([title, description]) => <section key={title}><h2 className="text-base font-semibold text-(--app-heading)">{title}</h2><p className="mt-2 text-sm leading-6 text-(--app-text-soft)">{description}</p></section>)}
                </div>
                <div className="mt-8 border-t border-(--app-border-soft) pt-5 text-xs leading-5 text-(--app-muted)">এই শর্তাবলি সম্পর্কে প্রশ্ন থাকলে আমাদের সহায়তা টিমের সঙ্গে যোগাযোগ করুন।</div>
            </article>
        </main>
    );
}
