"use client"

import {
    DollarSign,
    DoorOpenIcon,
    EllipsisVertical,
    MoonIcon,
    SettingsIcon,
    UserIcon,
} from "lucide-react"
import {
    DropdownMenu, DropdownMenuCheckboxItem, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel,
    DropdownMenuSeparator, DropdownMenuSub, DropdownMenuSubContent, DropdownMenuSubTrigger, DropdownMenuTrigger
} from "@/components/ui/dropdown-menu"
import { useTheme } from "@/components/theme-provider"
import { useState } from "react"
import { Button } from "@/components/ui/button"
import { Avatar, AvatarBadge, AvatarFallback, AvatarImage } from "@/components/ui/avatar"
import { signOut, useSession } from "next-auth/react"
import { useRouter } from "next/navigation";
import Link from "next/link"
import { useLanguage } from "@/components/language-provider"

export function MenuActions() {
    const router = useRouter();
    const [isOpen, setIsOpen] = useState(false);
    const { data: session } = useSession();
    const { theme, setTheme } = useTheme();
    const { t } = useLanguage();

    return (
        <div className="flex items-center gap-2">
            <DropdownMenu open={isOpen} onOpenChange={setIsOpen}>
                <DropdownMenuTrigger render={
                    <Button
                        variant="ghost"
                        size="icon"
                        className="h-7 w-7 data-[state=open]:bg-accent"
                    >
                        {session?.user ? (
                            <Avatar>
                                <AvatarImage
                                    src={
                                        session?.user?.image || ""
                                    }
                                />
                                <AvatarFallback>
                                    {session?.user?.name
                                        ?.split(" ")
                                        .map((n) => n[0])
                                        .join("")}
                                </AvatarFallback>
                                <AvatarBadge className="bg-green-600 dark:bg-green-800" />
                            </Avatar>
                        ) : (
                            <EllipsisVertical className="black" />
                        )}
                    </Button>
                }>
                </DropdownMenuTrigger>

                <DropdownMenuContent className="w-64">
                    {
                        session && (
                            <>
                                <DropdownMenuGroup>
                                    <DropdownMenuLabel>{t("account")}</DropdownMenuLabel>
                                    <DropdownMenuItem>
                                        <Link href="/profile" className="flex items-center gap-2 w-full">
                                            <UserIcon />
                                            {t("profile")}
                                        </Link>
                                    </DropdownMenuItem>
                                    <DropdownMenuItem onClick={() => router.push("/billing")}>
                                        <DollarSign />
                                        {t("billing")}
                                    </DropdownMenuItem>
                                </DropdownMenuGroup>
                                <DropdownMenuSeparator />
                            </>
                        )
                    }

                    <DropdownMenuGroup>
                        <DropdownMenuLabel>{t("settings")}</DropdownMenuLabel>
                        <DropdownMenuItem onClick={() => router.push("/settings")}>
                            <SettingsIcon />
                            {t("settings")}
                        </DropdownMenuItem>
                        <DropdownMenuSub>
                            <DropdownMenuSubTrigger>
                                <MoonIcon />
                                {t("theme")}: {theme === "dark" ? t("dark") : theme === "light" ? t("light") : t("system")}
                            </DropdownMenuSubTrigger>
                            <DropdownMenuSubContent>
                                <DropdownMenuLabel>{t("chooseTheme")}</DropdownMenuLabel>
                                {
                                    (["system", "dark", "light"] as const).map((option, index) => (
                                        <DropdownMenuCheckboxItem
                                            key={index}
                                            checked={option == theme}
                                            onCheckedChange={() => {
                                                setTheme(option)
                                            }}
                                        >
                                            {option === "dark" ? t("dark") : option === "light" ? t("light") : t("system")}
                                        </DropdownMenuCheckboxItem>
                                    ))
                                }
                            </DropdownMenuSubContent>
                        </DropdownMenuSub>
                    </DropdownMenuGroup>

                    {
                        session && (
                            <>
                                <DropdownMenuSeparator />
                                <DropdownMenuGroup>
                                    <DropdownMenuItem onClick={async () => {
                                        await signOut({
                                            redirectTo: `/api/refresh`,
                                        });
                                    }}>
                                        <DoorOpenIcon />
                                        {t("signOut")}
                                    </DropdownMenuItem>
                                </DropdownMenuGroup>
                            </>
                        )
                    }

                </DropdownMenuContent>
            </DropdownMenu>
        </div >
    )
}
