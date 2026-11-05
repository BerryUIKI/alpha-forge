// Comparison table renderer.

import type { ArtifactCellValue, ComparisonTableData } from "../parseArtifactData";

function formatCell(value: ArtifactCellValue | undefined): string {
  if (value === undefined || value === null) {
    return "—";
  }
  return String(value);
}

export function ComparisonTable({ data }: { data: ComparisonTableData }) {
  return (
    <div className="overflow-x-auto">
      <table className="w-full border-collapse text-sm">
        <thead>
          <tr className="border-b border-border text-left">
            <th scope="col" className="p-2 font-medium text-muted-foreground">
              Item
            </th>
            {data.columns.map((column) => (
              <th key={column.key} scope="col" className="p-2 font-medium">
                {column.label}
              </th>
            ))}
          </tr>
        </thead>
        <tbody>
          {data.rows.map((row, rowIndex) => (
            <tr
              key={`${row.label}-${rowIndex}`}
              className="border-b border-border/60 last:border-0"
            >
              <th scope="row" className="p-2 text-left font-medium">
                {row.label}
              </th>
              {data.columns.map((column) => (
                <td key={column.key} className="p-2">
                  {formatCell(row.values[column.key])}
                </td>
              ))}
            </tr>
          ))}
        </tbody>
      </table>
    </div>
  );
}
