// Timeline renderer.

import type { TimelineData } from "../parseArtifactData";

export function Timeline({ data }: { data: TimelineData }) {
  return (
    <ol className="relative space-y-4 border-l border-border pl-4">
      {data.events.map((event, index) => (
        <li key={`${event.date}-${index}`}>
          <p className="text-xs font-medium uppercase tracking-wide text-muted-foreground">
            {event.date}
          </p>
          <p className="font-medium">{event.title}</p>
          {event.description && (
            <p className="mt-0.5 text-sm text-muted-foreground">{event.description}</p>
          )}
        </li>
      ))}
    </ol>
  );
}
