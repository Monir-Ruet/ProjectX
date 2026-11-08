import { NextRequest, NextResponse } from "next/server";

export function GET(req: NextRequest) {
    const returnUrl = req.nextUrl.searchParams.get("returnUrl") || "/";

    let host = `${req.nextUrl.protocol}//${req.headers.get('host')}`;
    const redirectUrl = new URL(returnUrl, host);
    const res = NextResponse.redirect(redirectUrl);

    return res;
}
