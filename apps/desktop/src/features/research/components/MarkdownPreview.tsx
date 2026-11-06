// Safe Markdown preview.
//
// Raw HTML is skipped and sanitized; links render as text so user-authored
// Markdown cannot navigate the privileged main window.

import ReactMarkdown, { type Components } from "react-markdown";
import rehypeSanitize from "rehype-sanitize";
import remarkGfm from "remark-gfm";

const components: Components = {
  h1: ({ children }) => <h1 className="mb-2 mt-4 text-lg font-semibold">{children}</h1>,
  h2: ({ children }) => <h2 className="mb-2 mt-3 text-base font-semibold">{children}</h2>,
  h3: ({ children }) => <h3 className="mb-2 mt-3 text-sm font-semibold">{children}</h3>,
  p: ({ children }) => <p className="mb-2 text-sm leading-6">{children}</p>,
  ul: ({ children }) => <ul className="mb-2 list-disc space-y-1 pl-5 text-sm">{children}</ul>,
  ol: ({ children }) => <ol className="mb-2 list-decimal space-y-1 pl-5 text-sm">{children}</ol>,
  li: ({ children }) => <li>{children}</li>,
  code: ({ children }) => <code className="rounded bg-muted px-1 py-0.5 text-xs">{children}</code>,
  pre: ({ children }) => (
    <pre className="mb-2 overflow-x-auto rounded-md bg-muted p-3 text-xs">{children}</pre>
  ),
  a: ({ children, href }) => (
    <span className="text-primary underline" title={href}>
      {children}
    </span>
  ),
  table: ({ children }) => (
    <div className="mb-2 overflow-x-auto">
      <table className="w-full border-collapse text-sm">{children}</table>
    </div>
  ),
  th: ({ children }) => (
    <th className="border-b border-border p-2 text-left font-medium">{children}</th>
  ),
  td: ({ children }) => <td className="border-b border-border/60 p-2">{children}</td>,
  blockquote: ({ children }) => (
    <blockquote className="mb-2 border-l-2 border-border pl-3 text-sm text-muted-foreground">
      {children}
    </blockquote>
  ),
};

export function MarkdownPreview({ content }: { content: string }) {
  if (!content.trim()) {
    return <p className="text-sm text-muted-foreground">Nothing to preview yet.</p>;
  }

  return (
    <div className="rounded-md border border-border bg-background p-3">
      <ReactMarkdown
        remarkPlugins={[remarkGfm]}
        rehypePlugins={[rehypeSanitize]}
        skipHtml
        components={components}
      >
        {content}
      </ReactMarkdown>
    </div>
  );
}
