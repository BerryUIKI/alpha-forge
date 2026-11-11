import "@testing-library/jest-dom";
import { describe, expect, it, vi } from "vitest";
import { fireEvent, render, screen } from "@testing-library/react";
import { ResearchProjectsSection } from "./ResearchProjectsSection";

vi.mock("@/lib/i18n/useLocale", () => ({
  useLocale: () => ({ t: (key: string) => key }),
}));

function renderSection(
  overrides: {
    projectTitle?: string;
    createPending?: boolean;
    onCreateProject?: () => void;
    onProjectTitleChange?: (value: string) => void;
  } = {},
) {
  const onProjectTitleChange = overrides.onProjectTitleChange ?? vi.fn();
  const onCreateProject = overrides.onCreateProject ?? vi.fn();
  render(
    <ResearchProjectsSection
      projects={[]}
      projectTitle={overrides.projectTitle ?? "Draft project"}
      onProjectTitleChange={onProjectTitleChange}
      createPending={overrides.createPending ?? false}
      onCreateProject={onCreateProject}
      onSelectProject={vi.fn()}
      archiveProject={vi.fn()}
      completeProject={vi.fn()}
      deleteProject={vi.fn()}
    />,
  );
  return { onProjectTitleChange, onCreateProject };
}

describe("ResearchProjectsSection", () => {
  it("clears the draft when Escape is pressed in the creation field", () => {
    const { onProjectTitleChange } = renderSection();

    fireEvent.keyDown(screen.getByLabelText("projectTitle"), { key: "Escape" });

    expect(onProjectTitleChange).toHaveBeenCalledWith("");
  });

  it("does not submit while creation is pending", () => {
    const { onCreateProject } = renderSection({ createPending: true });

    fireEvent.submit(screen.getByLabelText("projectTitle").closest("form")!);

    expect(onCreateProject).not.toHaveBeenCalled();
  });
});
