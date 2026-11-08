import { NextRequest, NextResponse } from "next/server";

export function GET(req: NextRequest) {
    const host = `${req.nextUrl.protocol}//${req.headers.get("host")}`;

    const response = NextResponse.redirect(new URL("/signin", host));

    const authCookieName =
        process.env.NODE_ENV === "production"
            ? "__Secure-authjs.session-token"
            : "authjs.session-token";

    response.cookies.set(authCookieName, "", {
        expires: new Date(0),
        maxAge: 0,
        path: "/",
        httpOnly: true,
        secure: process.env.NODE_ENV === "production",
        sameSite: "lax",
    });

    return response;
}
