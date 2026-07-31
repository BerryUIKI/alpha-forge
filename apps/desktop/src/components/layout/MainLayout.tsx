import { Outlet } from "react-router-dom";
import { Sidebar } from "@/components/navigation/Sidebar";

export function MainLayout() {
  return (
    <div className="flex h-screen bg-background">
      <Sidebar />
      <main className="flex-1 overflow-auto">
        <Outlet />
      </main>
    </div>
  );
}
