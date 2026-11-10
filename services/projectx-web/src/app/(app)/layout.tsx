import { AppHeader } from "@/components/app-header";
import { BottomNavigation } from "@/components/bottom-navigation";

export default function RootLayout({
    children,
}: Readonly<{
    children: React.ReactNode;
}>) {
    return (
        <div className="min-h-svh bg-(--app-bg)">
            <AppHeader />
            <div className="mx-auto w-full px-4 pb-24 pt-5 sm:px-6 lg:w-3/5 lg:px-0">
                {children}
            </div>
            <BottomNavigation />
        </div>
    );
}
