import type { NextConfig } from "next";

const nextConfig: NextConfig = {
    output: "standalone",
    images: {
        remotePatterns: [
            {
                protocol: "https",
                hostname: "lh3.googleusercontent.com",
            },
        ],
    },
    allowedDevOrigins: ["nirvanic-johanna-unrepresentatively.ngrok-free.dev", "grafting-aftermost-property.ngrok-free.dev"],
};

export default nextConfig;
