import { useState } from "react";
import type { KnowledgeEntityType } from "@/lib/desktop-api/knowledge-graph";
import { useCreateKnowledgeEntity, useCreateKnowledgeRelationship, useKnowledgeEntities, useKnowledgeRelationships } from "../hooks/useKnowledgeGraph";
import { useLocale } from "@/lib/i18n/useLocale";

export function KnowledgeGraphPanel({ workspaceId }: { workspaceId: string }) {
  const { t } = useLocale();
  const entities = useKnowledgeEntities(workspaceId);
  const relationships = useKnowledgeRelationships(workspaceId);
  const createEntity = useCreateKnowledgeEntity();
  const createRelationship = useCreateKnowledgeRelationship();
  const [name, setName] = useState("");
  const [entityType, setEntityType] = useState<KnowledgeEntityType>("company");
  const [sourceId, setSourceId] = useState("");
  const [targetId, setTargetId] = useState("");
  const [relationshipType, setRelationshipType] = useState("enables");
  const [error, setError] = useState("");

  async function addEntity(event: React.FormEvent) {
    event.preventDefault();
    if (!name.trim()) return setError(t("entityNameRequired"));
    try {
      await createEntity.mutateAsync({ workspaceId, entityType, name: name.trim() });
      setName("");
      setError("");
    } catch {
      setError(t("unableToCreateEntity"));
    }
  }

  async function addRelationship(event: React.FormEvent) {
    event.preventDefault();
    if (!sourceId || !targetId || !relationshipType.trim()) return setError(t("relationshipRequired"));
    try {
      await createRelationship.mutateAsync({ sourceEntityId: sourceId, targetEntityId: targetId, relationshipType: relationshipType.trim() });
      setError("");
    } catch {
      setError(t("unableToCreateRelationship"));
    }
  }

  const entityName = (id: string) => entities.data?.find((entity) => entity.id === id)?.name ?? id;

  return (
    <section className="rounded-lg border border-border bg-card p-5">
      <h2 className="text-lg font-semibold">{t("knowledgeGraph")}</h2>
      <p className="mt-1 text-sm text-muted-foreground">{t("knowledgeGraphDescription")}</p>

      <div className="mt-4 grid gap-4 lg:grid-cols-2">
        <form onSubmit={addEntity} className="space-y-2">
          <div className="flex gap-2">
            <select value={entityType} onChange={(event) => setEntityType(event.target.value as KnowledgeEntityType)} className="rounded-md border border-input bg-background px-2 text-sm">
              <option value="company">{t("entityTypeCompany")}</option>
              <option value="industry">{t("entityTypeIndustry")}</option>
              <option value="technology">{t("entityTypeTechnology")}</option>
              <option value="macro_theme">{t("entityTypeMacroTheme")}</option>
            </select>
            <input value={name} onChange={(event) => setName(event.target.value)} placeholder={t("entityName")} className="min-w-0 flex-1 rounded-md border border-input bg-background px-3 py-2 text-sm" />
          </div>
          <button className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent" disabled={createEntity.isPending}>
            {t("addEntity")}
          </button>
        </form>

        <form onSubmit={addRelationship} className="space-y-2">
          <div className="grid grid-cols-3 gap-2">
            <select value={sourceId} onChange={(event) => setSourceId(event.target.value)} className="rounded-md border border-input bg-background p-2 text-sm">
              <option value="">{t("source")}</option>
              {entities.data?.map((entity) => <option key={entity.id} value={entity.id}>{entity.name}</option>)}
            </select>
            <input value={relationshipType} onChange={(event) => setRelationshipType(event.target.value)} className="min-w-0 rounded-md border border-input bg-background px-2 text-sm" />
            <select value={targetId} onChange={(event) => setTargetId(event.target.value)} className="rounded-md border border-input bg-background p-2 text-sm">
              <option value="">{t("target")}</option>
              {entities.data?.map((entity) => <option key={entity.id} value={entity.id}>{entity.name}</option>)}
            </select>
          </div>
          <button className="rounded-md border border-input px-3 py-2 text-sm hover:bg-accent" disabled={createRelationship.isPending}>
            {t("addRelationship")}
          </button>
        </form>
      </div>

      {error && <p className="mt-2 text-sm text-destructive">{error}</p>}

      <div className="mt-4 grid gap-4 lg:grid-cols-2">
        <ul className="space-y-1 text-sm">
          {entities.data?.map((entity) => (
            <li key={entity.id} className="rounded bg-muted px-2 py-1">
              <span className="text-muted-foreground">{entity.entity_type}: </span>
              {entity.name}
            </li>
          ))}
        </ul>
        <ul className="space-y-1 text-sm">
          {relationships.data?.map((relationship) => (
            <li key={relationship.id} className="rounded bg-muted px-2 py-1">
              {entityName(relationship.source_entity_id)} <span className="text-muted-foreground">{relationship.relationship_type}</span> {entityName(relationship.target_entity_id)}
            </li>
          ))}
        </ul>
      </div>
    </section>
  );
}