export default function Switch({ checked, onChange }: Readonly<{ checked: boolean; onChange: () => void }>) {
    return (
        <button
            type="button"
            role="switch"
            aria-checked={checked}
            aria-label={checked ? "বন্ধ করুন" : "চালু করুন"}
            onClick={onChange}
            className={`relative h-5 w-9 shrink-0 rounded-full transition-colors focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-(--app-primary) ${checked ? "bg-(--app-primary)" : "bg-(--app-border-strong)"}`}
        >
            <span className={`absolute left-0.5 top-0.5 size-4 rounded-full bg-white shadow-sm transition-transform ${checked ? "translate-x-4" : "translate-x-0"}`} />
        </button>
    );
}
