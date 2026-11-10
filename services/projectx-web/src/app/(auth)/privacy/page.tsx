import Link from "next/link";
import { ArrowLeft } from "lucide-react";
import { BrandIcon } from "@/components/brand-icon";

const sections = [
    ["আমরা যে তথ্য নিই", "অ্যাকাউন্ট তৈরি করতে আপনার নাম, ইমেইল এবং পাসওয়ার্ডের মতো তথ্য প্রয়োজন হতে পারে। আপনার শেখার অগ্রগতি ও পরীক্ষার ফলাফলও সেবার অংশ হিসেবে সংরক্ষিত হতে পারে।"],
    ["তথ্যের ব্যবহার", "আমরা আপনার অ্যাকাউন্ট পরিচালনা, শেখার অগ্রগতি দেখানো, মক টেস্টের ফলাফল তৈরি এবং সেবার মান উন্নত করতে তথ্য ব্যবহার করি।"],
    ["তথ্য সুরক্ষা", "আপনার পাসওয়ার্ড নিরাপদভাবে সংরক্ষণ করার জন্য সুরক্ষিত পদ্ধতি ব্যবহার করা হয়। তবে কোনো অনলাইন সেবাই সম্পূর্ণ ঝুঁকিমুক্ত নয়।"],
    ["আপনার অধিকার", "আপনার অ্যাকাউন্টের তথ্য দেখতে, সংশোধন করতে বা মুছে ফেলার অনুরোধ করতে পারেন। এ ধরনের অনুরোধের জন্য সহায়তা টিমের সঙ্গে যোগাযোগ করুন।"],
];

export default function PrivacyPage() {
    return (
        <main className="min-h-svh bg-(--app-bg) px-4 py-8 sm:px-6">
            <article className="mx-auto max-w-2xl rounded-3xl border border-(--app-border) bg-(--app-card) p-6 shadow-sm sm:p-10">
                <BrandIcon />
                <Link href="/signin" className="inline-flex items-center gap-1 text-xs font-medium text-(--app-primary)"><ArrowLeft className="size-3.5" /> ফিরে যান</Link>
                <p className="mt-8 text-xs font-medium text-(--app-muted)">সর্বশেষ আপডেট: ১১ নভেম্বর, ২০২৬</p>
                <h1 className="mt-2 text-3xl font-semibold tracking-tight text-(--app-heading)">গোপনীয়তা নীতি</h1>
                <p className="mt-3 text-sm leading-6 text-(--app-muted-strong)">আপনার তথ্য কীভাবে সংগ্রহ, ব্যবহার ও সুরক্ষিত রাখা হয়—এই নীতিতে তা সংক্ষেপে বলা হয়েছে।</p>
                <div className="mt-8 space-y-6">
                    {sections.map(([title, description]) => <section key={title}><h2 className="text-base font-semibold text-(--app-heading)">{title}</h2><p className="mt-2 text-sm leading-6 text-(--app-text-soft)">{description}</p></section>)}
                </div>
                <div className="mt-8 border-t border-(--app-border-soft) pt-5 text-xs leading-5 text-(--app-muted)">নীতি পরিবর্তন হলে এই পৃষ্ঠায় সর্বশেষ সংস্করণ প্রকাশ করা হবে।</div>
            </article>
        </main >
    );
}
