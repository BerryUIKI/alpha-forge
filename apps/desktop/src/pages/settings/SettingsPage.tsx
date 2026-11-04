import { ProviderSettings } from "@/features/settings";

export function SettingsPage() {
  return (
    <div className="p-6">
      <h1 className="mb-6 text-2xl font-bold">Settings</h1>
      <div className="max-w-3xl space-y-6">
        <ProviderSettings />
      </div>
    </div>
  );
}
