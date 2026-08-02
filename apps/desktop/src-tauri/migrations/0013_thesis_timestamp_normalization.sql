-- Normalize legacy SQLite UTC timestamps and keep future trigger writes RFC3339-compatible.
UPDATE investment_theses
SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', created_at)
WHERE created_at GLOB '????-??-?? ??:??:??*';

UPDATE investment_theses
SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', updated_at)
WHERE updated_at GLOB '????-??-?? ??:??:??*';

UPDATE investment_theses
SET validation_date = strftime('%Y-%m-%dT%H:%M:%fZ', validation_date)
WHERE validation_date GLOB '????-??-?? ??:??:??*';

UPDATE thesis_evidence
SET created_at = strftime('%Y-%m-%dT%H:%M:%fZ', created_at)
WHERE created_at GLOB '????-??-?? ??:??:??*';

UPDATE thesis_confidence_history
SET recorded_at = strftime('%Y-%m-%dT%H:%M:%fZ', recorded_at)
WHERE recorded_at GLOB '????-??-?? ??:??:??*';

DROP TRIGGER IF EXISTS update_thesis_updated_at;
CREATE TRIGGER update_thesis_updated_at
AFTER UPDATE ON investment_theses
FOR EACH ROW
BEGIN
    UPDATE investment_theses
    SET updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
    WHERE id = OLD.id;
END;
