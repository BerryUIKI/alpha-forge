import { ThesisDashboard } from "@/features/thesis";

export function JournalPage() {
  return (
    <div className="p-6">
      <h1 className="text-2xl font-bold">Journal</h1>
      <div className="mt-6">
        <ThesisDashboard />
      </div>
    </div>
  );
}
