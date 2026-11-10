import type { Metadata } from "next";
import { Geist } from "next/font/google";
import "./globals.css";
import { ThemeWrapper } from "@/components/theme-wrapper";
import { SessionProvider } from "next-auth/react";
import { Toaster } from "sonner";
import ProgressLoader from "@/components/progress-loader";
import { cookies } from "next/headers";
import type { Language } from "@/components/language-provider";

const geistSans = Geist({
    variable: "--font-geist-sans",
    subsets: ["latin"],
});

export const metadata: Metadata = {
    title: "ProjectX",
    description: "ProjectX is a platform that allows you to manage your projects and tasks in a simple and efficient way.",
    icons: {
        icon: "/x.webp",
    },
};

export default async function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    const languageCookie = (await cookies()).get("projectx-language")?.value;
    const language: Language = languageCookie === "en" ? "en" : "bn";

    return (
        <html lang={language} dir="ltr" suppressHydrationWarning>
            <body className={`${geistSans.className} h-full antialiased text-sm`}>
                <ProgressLoader />
                <ThemeWrapper initialLanguage={language}>
                    <SessionProvider>
                        {children}
                    </SessionProvider>
                </ThemeWrapper>
                <Toaster richColors closeButton theme="system" position="top-center" visibleToasts={1} duration={3000} />
            </body>
        </html >
    );
}
