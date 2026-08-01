// Artifact renderer types and registry.

import type { ReactNode } from "react";

/**
 * Props for artifact renderers.
 */
export interface ArtifactRendererProps {
  artifactId: string;
  data: unknown;
  onUpdate?: (data: unknown) => void;
}

/**
 * Metadata for an artifact renderer.
 */
export interface ArtifactRendererMeta {
  type: string;
  name: string;
  description: string;
  icon?: ReactNode;
}

/**
 * Registry of artifact renderers.
 */
class ArtifactRendererRegistry {
  private renderers: Map<string, React.ComponentType<ArtifactRendererProps>> = new Map();
  private metadata: Map<string, ArtifactRendererMeta> = new Map();

  /**
   * Register an artifact renderer.
   */
  register(
    type: string,
    renderer: React.ComponentType<ArtifactRendererProps>,
    meta: ArtifactRendererMeta
  ): void {
    this.renderers.set(type, renderer);
    this.metadata.set(type, meta);
  }

  /**
   * Get a renderer by artifact type.
   */
  getRenderer(type: string): React.ComponentType<ArtifactRendererProps> | undefined {
    return this.renderers.get(type);
  }

  /**
   * Get metadata for an artifact type.
   */
  getMetadata(type: string): ArtifactRendererMeta | undefined {
    return this.metadata.get(type);
  }

  /**
   * List all registered artifact types.
   */
  listTypes(): string[] {
    return Array.from(this.renderers.keys());
  }

  /**
   * Check if a type is registered.
   */
  has(type: string): boolean {
    return this.renderers.has(type);
  }
}

export const artifactRegistry = new ArtifactRendererRegistry();