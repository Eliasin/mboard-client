import { CanvasRect, CanvasView, RasterLayerAction } from "../pkg/mboard_client";
import { Brush } from "./canvas";

export type Dragging = {
    kind: 'dragging';
    dragStart: [number, number];
};

export type NoDrag = {
    kind: 'idle';
}

export type DragState = Dragging | NoDrag;

export function createBrushAction(e: MouseEvent, canvasView: CanvasView, brush: Brush): RasterLayerAction {
    const canvasPosition = canvasView.transformViewToCanvas(e.offsetX, e.offsetY);
    const radius = brush.radius;

    const topLeft = [Number(canvasPosition.x) - radius, Number(canvasPosition.y) - radius];
    const brushRect = new CanvasRect(BigInt(topLeft[0]), BigInt(topLeft[1]), radius * 2, radius * 2);

    return RasterLayerAction.fillOval(brushRect, brush.color);
}
