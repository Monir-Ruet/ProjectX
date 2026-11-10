'use client'

import { Button } from "@/components/ui/button"
import { Input } from "@/components/ui/input"
import Image from "next/image"
import { AlertTriangleIcon, FingerprintPattern, GraduationCap, Sparkles } from "lucide-react"
import { signIn } from "next-auth/react"
import { useState } from "react"
import { useRouter } from "next/navigation";
import { signInSchema, signInZodSchema } from "@/schemas/account"
import { useForm } from "react-hook-form"
import { zodResolver } from "@hookform/resolvers/zod"
import { Alert, AlertDescription } from "@/components/ui/alert"
import {
    Field,
    FieldDescription,
    FieldGroup,
    FieldLabel,
    FieldSeparator,
} from "@/components/ui/field"
import Link from "next/link"
import { Base64 } from "js-base64"
import { reqwest } from "@/lib/fetch"
import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupInput } from "@/components/ui/input-group"
import { Tooltip, TooltipContent, TooltipTrigger } from "@/components/ui/tooltip"


export default function SignIn() {
    const router = useRouter();
    const [message, setMessage] = useState<string | null>("");
    const [loading, setLoading] = useState(false);
    const signInForm = useForm<signInSchema>({
        resolver: zodResolver(signInZodSchema),
        defaultValues: {
            email: "",
            password: "",
        },
    });
    const email = signInForm.watch("email");

    const handleSignIn = async (data: signInSchema) => {
        setLoading(true);
        setMessage(null);
        const signInResult = await signIn("credentials", {
            email: data.email,
            password: data.password,
            redirect: false,
        });
        setLoading(false);
        if (signInResult.error) {
            setMessage("Invalid email or password");
        } else {
            router.push("/");
        }
    };

    const handlePassKeySignin = async () => {
        try {
            const res = await reqwest('/api/auth/webauthn/login/options?email=' + signInForm.getValues('email'), {
                method: 'POST',
            });

            if (!res.ok)
                throw new Error('Failed to fetch authentication options');

            const credentialRequestOptions = await res.json();

            const publicKey = credentialRequestOptions.options.publicKey;

            publicKey.challenge = Base64.toUint8Array(publicKey.challenge);

            publicKey.allowCredentials?.forEach(
                (credential: { id: string | Uint8Array }) => {
                    if (typeof credential.id === 'string') {
                        credential.id = Base64.toUint8Array(credential.id);
                    }
                }
            );

            const assertion = await navigator.credentials.get({
                publicKey,
                mediation: "required"
            });

            if (!assertion)
                throw new Error('Authentication failed');

            const pkc = assertion as PublicKeyCredential;
            const response = pkc.response as AuthenticatorAssertionResponse;

            const signInResult = await reqwest('/api/auth/webauthn/login/verify', {
                method: 'POST',
                body: JSON.stringify({
                    auth: {
                        id: pkc.id,
                        rawId: Base64.fromUint8Array(new Uint8Array(pkc.rawId), true),
                        type: pkc.type,
                        response: {
                            authenticatorData: Base64.fromUint8Array(new Uint8Array(response.authenticatorData), true),
                            clientDataJSON: Base64.fromUint8Array(new Uint8Array(response.clientDataJSON), true),
                            signature: Base64.fromUint8Array(new Uint8Array(response.signature), true),
                            userHandle: response.userHandle ? Base64.fromUint8Array(new Uint8Array(response.userHandle), true) : null
                        }
                    },
                    state_token: credentialRequestOptions.state_token,
                }),
            });

            if (!signInResult.ok) {
                throw new Error('Failed to verify authentication');
            }

            const { access_token, refresh_token } = await signInResult.json();
            await signIn("credentials", { access_token, refresh_token, redirect: false });

            router.push("/");
            return true;
        } catch {
            setMessage("Authentication failed. Please try again. Make sure you have registered a passkey for this email.");
            return false;
        }
    };

    return (
        <main className="flex min-h-svh items-center justify-center bg-(--app-bg) p-4 sm:p-8">
            <div className="grid w-full max-w-5xl overflow-hidden rounded-3xl border border-(--app-border) bg-(--app-card) shadow-2xl lg:grid-cols-[0.85fr_1.15fr]">
                <section className="relative hidden overflow-hidden bg-(--app-hero) p-10 text-white lg:flex lg:flex-col lg:justify-between">
                    <div className="absolute -bottom-24 -left-20 size-72 rounded-full bg-(--app-hero-glow)/40 blur-3xl" />
                    <div className="relative">
                        <Link href="/" className="inline-flex items-center gap-3">
                            <span className="flex size-11 items-center justify-center rounded-xl bg-(--app-gold) text-(--app-hero)"><GraduationCap className="size-6" /></span>
                            <span className="text-lg font-bold">পাঠশালা</span>
                        </Link>
                        <div className="mt-24 max-w-xs">
                            <Sparkles className="size-6 text-(--app-gold)" />
                            <h1 className="mt-5 text-4xl font-semibold leading-tight">আপনার শেখার যাত্রা এখান থেকেই শুরু।</h1>
                            <p className="mt-5 text-sm leading-6 text-(--app-hero-text)">প্রতিদিন একটু করে এগিয়ে যান, নিজের লক্ষ্যে পৌঁছে যান।</p>
                        </div>
                    </div>
                    <p className="relative text-xs text-(--app-hero-muted)">স্মার্টভাবে শিখুন • নিয়মিত এগিয়ে যান</p>
                </section>
                <section className="p-6 sm:p-10">
                    <div className="mb-8 flex items-center gap-3 lg:hidden">
                        <span className="flex size-10 items-center justify-center rounded-xl bg-(--app-hero) text-(--app-gold)"><GraduationCap className="size-5" /></span>
                        <span className="font-bold text-(--app-heading)">পাঠশালা</span>
                    </div>
                    <div className="mb-8">
                        <p className="text-xs font-medium uppercase tracking-[0.18em] text-(--app-muted)">স্বাগতম</p>
                        <h1 className="mt-2 text-3xl font-semibold tracking-tight text-(--app-heading)">আবার দেখা হচ্ছে</h1>
                        <p className="mt-2 text-sm text-(--app-muted-strong)">আপনার অ্যাকাউন্টে সাইন ইন করে শেখা চালিয়ে যান।</p>
                    </div>
                    <div className="flex flex-col">
                        <form onSubmit={signInForm.handleSubmit(handleSignIn)} >
                            <FieldGroup>
                                <Field>
                                    <div className="grid gap-3 sm:grid-cols-2">
                                        <Button variant="outline" className="h-11 border-(--app-border-strong)" type="button" onClick={() => signIn("google", { callbackUrl: "/" })}>
                                            <Image src="/google.svg" alt="Google" width={20} height={20} />
                                            Google
                                        </Button>
                                        <Button variant="outline" className="h-11 border-(--app-border-strong)" type="button" onClick={() => signIn("github", { callbackUrl: "/" })}>
                                            <Image src={"/github.svg"} width={20} height={20} alt="github" />
                                            Github
                                        </Button>
                                    </div>
                                </Field>

                                <FieldSeparator className="*:data-[slot=field-separator-content]:bg-(--app-card) text-(--app-muted)">
                                    অথবা ইমেইল দিয়ে
                                </FieldSeparator>

                                {
                                    message &&
                                    (
                                        <Alert className="border-red-200 bg-red-50 text-red-900 dark:border-red-900 dark:bg-red-950 dark:text-red-100">
                                            <AlertTriangleIcon />
                                            <AlertDescription>
                                                {message}
                                            </AlertDescription>
                                        </Alert>
                                    )
                                }

                                <Field>
                                    <FieldLabel htmlFor="email">ইমেইল</FieldLabel>
                                    <InputGroup className="rounded-sm py-5">
                                        <InputGroupInput
                                            {...signInForm.register("email")}
                                            id="email"
                                            type="text"
                                            required
                                            autoComplete="username webauthn" />

                                        <InputGroupAddon align="inline-end">
                                            <Tooltip >
                                                <TooltipTrigger render={
                                                    <InputGroupButton
                                                        variant="secondary"
                                                        type="button"
                                                        onClick={handlePassKeySignin}
                                                        disabled={!email.trim()}
                                                    >
                                                        <FingerprintPattern />
                                                    </InputGroupButton>} />
                                                <TooltipContent>
                                                    <p>Sign in with Passkey</p>
                                                </TooltipContent>
                                            </Tooltip>


                                        </InputGroupAddon>
                                    </InputGroup>
                                </Field>
                                <Field>
                                    <div className="flex items-center">
                                        <FieldLabel htmlFor="password">পাসওয়ার্ড</FieldLabel>
                                        <Link href="/forgot-password" className="ml-auto text-xs text-(--app-muted-strong) underline-offset-4 hover:text-(--app-primary) hover:underline">
                                            পাসওয়ার্ড ভুলে গেছেন?
                                        </Link>
                                    </div>
                                    <Input
                                        {...signInForm.register("password")}
                                        id="password"
                                        type="password"
                                        required
                                        className="rounded-sm p-5"
                                    />
                                </Field>
                                <Field>
                                    <Button type="submit" disabled={loading} className="h-11 w-full bg-(--app-primary) text-sm font-semibold text-white hover:bg-(--app-primary-hover)">
                                        {loading ? "সাইন ইন হচ্ছে..." : "সাইন ইন করুন"}
                                    </Button>
                                    <FieldDescription className="text-center">
                                        অ্যাকাউন্ট নেই? <Link href="/signup" className="font-medium text-(--app-primary) hover:underline">
                                            রেজিস্টার করুন
                                        </Link>
                                    </FieldDescription>
                                </Field>
                            </FieldGroup>
                        </form>

                        <FieldDescription className="px-6 text-center">
                            চালিয়ে গেলে আপনি আমাদের <Link href="/terms" className="text-(--app-primary) hover:underline">
                                শর্তাবলি
                            </Link>{" "}
                            and <Link href="/privacy" className="text-primary hover:underline">
                                গোপনীয়তা নীতি
                            </Link>.
                        </FieldDescription>
                    </div>
                </section>
            </div>
        </main>
    )
}
