/**
 * useResize Hook
 *
 * Handles drag-to-resize and keyboard resizing for sidebars.
 *
 * Keyboard contract (on the focused `separator` handle):
 * - ArrowLeft / ArrowRight adjust width in the direction of the pane edge.
 * - Home resets to `minWidth`, End expands to `maxWidth`.
 * - Shift increases the step (16px → 48px).
 *
 * @module hooks/layout
 */

import { useState, useCallback, useEffect, useRef } from "react";

const KEYBOARD_STEP_PX = 16;
const KEYBOARD_LARGE_STEP_PX = 48;

interface UseResizeConfig {
  /** Initial width */
  initialWidth: number;
  /** Minimum width constraint */
  minWidth: number;
  /** Maximum width constraint */
  maxWidth: number;
  /** Callback when width changes */
  onWidthChange?: (width: number) => void;
  /** Resize direction (left or right edge) */
  direction?: "left" | "right";
}

interface UseResizeReturn {
  /** Current width */
  width: number;
  /** Whether currently resizing */
  isResizing: boolean;
  /** Ref to attach to the resize handle */
  resizeHandleRef: React.RefObject<HTMLDivElement>;
  /** Start resize operation */
  startResize: (event: React.MouseEvent) => void;
  /** Stop resize operation */
  stopResize: () => void;
  /** Keyboard handler for the resize separator */
  handleKeyDown: (event: React.KeyboardEvent) => void;
}

/**
 * Hook for handling drag-to-resize and keyboard resizing.
 */
export function useResize({
  initialWidth,
  minWidth,
  maxWidth,
  onWidthChange,
  direction = "right",
}: UseResizeConfig): UseResizeReturn {
  const [width, setWidth] = useState(initialWidth);
  const [isResizing, setIsResizing] = useState(false);
  const resizeHandleRef = useRef<HTMLDivElement>(null);
  const startXRef = useRef(0);
  const startWidthRef = useRef(0);

  const applyWidth = useCallback(
    (nextWidth: number) => {
      const constrainedWidth = Math.max(minWidth, Math.min(maxWidth, nextWidth));
      setWidth(constrainedWidth);
      onWidthChange?.(constrainedWidth);
    },
    [minWidth, maxWidth, onWidthChange],
  );

  // Start resize operation
  const startResize = useCallback(
    (event: React.MouseEvent) => {
      event.preventDefault();
      setIsResizing(true);
      startXRef.current = event.clientX;
      startWidthRef.current = width;
      document.body.style.cursor = "col-resize";
      document.body.style.userSelect = "none";
    },
    [width],
  );

  // Stop resize operation
  const stopResize = useCallback(() => {
    setIsResizing(false);
    document.body.style.cursor = "";
    document.body.style.userSelect = "";
  }, []);

  // Handle keyboard resizing on the separator.
  const handleKeyDown = useCallback(
    (event: React.KeyboardEvent) => {
      const step = event.shiftKey ? KEYBOARD_LARGE_STEP_PX : KEYBOARD_STEP_PX;

      switch (event.key) {
        case "ArrowLeft":
        case "ArrowRight": {
          event.preventDefault();
          // A handle on the right edge grows with ArrowRight; a handle on the
          // left edge grows in the opposite direction.
          const towardsRight = event.key === "ArrowRight";
          const grows = direction === "right" ? towardsRight : !towardsRight;
          applyWidth(width + (grows ? step : -step));
          break;
        }
        case "Home":
          event.preventDefault();
          applyWidth(minWidth);
          break;
        case "End":
          event.preventDefault();
          applyWidth(maxWidth);
          break;
        default:
          break;
      }
    },
    [applyWidth, direction, minWidth, maxWidth, width],
  );

  // Handle mouse move during resize
  useEffect(() => {
    if (!isResizing) return;

    const handleMouseMove = (event: MouseEvent) => {
      const deltaX = event.clientX - startXRef.current;
      const delta = direction === "right" ? deltaX : -deltaX;
      const newWidth = startWidthRef.current + delta;
      const constrainedWidth = Math.max(minWidth, Math.min(maxWidth, newWidth));

      setWidth(constrainedWidth);
      onWidthChange?.(constrainedWidth);
    };

    const handleMouseUp = () => {
      stopResize();
    };

    document.addEventListener("mousemove", handleMouseMove);
    document.addEventListener("mouseup", handleMouseUp);

    return () => {
      document.removeEventListener("mousemove", handleMouseMove);
      document.removeEventListener("mouseup", handleMouseUp);
    };
  }, [isResizing, minWidth, maxWidth, direction, onWidthChange, stopResize]);

  // Update width when initialWidth changes
  useEffect(() => {
    setWidth(initialWidth);
  }, [initialWidth]);

  return {
    width,
    isResizing,
    resizeHandleRef: resizeHandleRef as React.RefObject<HTMLDivElement>,
    startResize,
    stopResize,
    handleKeyDown,
  };
}
