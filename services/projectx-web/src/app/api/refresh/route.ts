import { NextRequest, NextResponse } from "next/server";

export function GET(req: NextRequest) {
    const returnUrl = req.nextUrl.searchParams.get("returnUrl") || "/";
    let host = `https://${req.headers.get('host')}`;
    process.env.NODE_ENV === "development" && (host = req.nextUrl.origin);
    const redirectUrl = new URL(returnUrl, host);
    const res = NextResponse.redirect(redirectUrl);

    return res;
}
