import {
  CanvasRect,
  CanvasView,
  RasterLayerAction,
} from "../pkg/mboard_client";
import { Brush } from "./canvas";

export type Dragging = {
  kind: "dragging";
  dragStart: [number, number];
};

export type Continuous = {
  kind: "continuous";
  lastPoint: [number, number];
};

export type NoDrag = {
  kind: "idle";
};

export type DragState = Dragging | NoDrag | Continuous;

export function createBrushAction(
  e: MouseEvent,
  canvasView: CanvasView,
  brush: Brush
): RasterLayerAction {
  const canvasPosition = canvasView.transformViewToCanvas(e.offsetX, e.offsetY);
  const radius = brush.radius;

  const topLeft = [canvasPosition.x - radius, canvasPosition.y - radius];
  const brushRect = new CanvasRect(
    topLeft[0],
    topLeft[1],
    radius * 2,
    radius * 2
  );

  return RasterLayerAction.fillOval(brushRect, brush.color);
}

export function distanceTraversed(
  continuousDrag: Continuous,
  mouseEvent: MouseEvent
): number {
  let [x0, y0] = continuousDrag.lastPoint;
  let [x1, y1] = [mouseEvent.offsetX, mouseEvent.offsetY];

  return Math.sqrt(Math.pow(x1 - x0, 2) + Math.pow(y1 - y0, 2));
}
