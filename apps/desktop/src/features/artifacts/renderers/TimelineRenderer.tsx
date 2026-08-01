// Timeline artifact renderer.

import type { ArtifactRendererProps } from "./registry";

interface TimelineEvent {
  date: string;
  title: string;
  description?: string;
  type?: string;
}

interface TimelineData {
  events: TimelineEvent[];
}

/**
 * Timeline renderer for displaying chronological events.
 */
export function TimelineRenderer({ data }: ArtifactRendererProps) {
  const timelineData = data as TimelineData;

  if (!timelineData?.events || timelineData.events.length === 0) {
    return (
      <div className="p-4 text-center text-muted-foreground">
        No events to display
      </div>
    );
  }

  const { events } = timelineData;

  return (
    <div className="p-4">
      <div className="space-y-4">
        {events
          .sort((a, b) => new Date(b.date).getTime() - new Date(a.date).getTime())
          .map((event, index) => (
            <div key={index} className="flex gap-4">
              <div className="w-24 shrink-0 text-sm text-muted-foreground">
                {formatDate(event.date)}
              </div>
              <div className="flex-1 border-l-2 border-muted pl-4 pb-4">
                <div className="font-medium">{event.title}</div>
                {event.description && (
                  <div className="text-sm text-muted-foreground mt-1">
                    {event.description}
                  </div>
                )}
              </div>
            </div>
          ))}
      </div>
    </div>
  );
}

function formatDate(dateStr: string): string {
  const date = new Date(dateStr);
  return date.toLocaleDateString("en-US", {
    year: "numeric",
    month: "short",
    day: "numeric",
  });
}