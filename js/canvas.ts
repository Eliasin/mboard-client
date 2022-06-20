import {
  Canvas,
  CanvasView,
  CanvasRect,
  Pixel,
  ImageDataService,
} from "../pkg/mboard_client";
import { DragState } from "./interactions";

export function renderCanvas(
  imageDataService: ImageDataService,
  canvas: Canvas,
  view: CanvasView
) {
  const imageData = imageDataService.getImageDataFromCanvas(canvas, view);
  const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

  const canvasContext = canvasElement.getContext("2d");
  if (canvasContext !== null) {
    canvasContext.putImageData(imageData, 0, 0);
  } else {
    console.error("Could not get canvas context");
  }
}

export function updateCanvas(
  imageDataService: ImageDataService,
  canvas: Canvas,
  canvasRect: CanvasRect,
  canvasView: CanvasView
) {
  const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

  const canvasContext = canvasElement.getContext("2d");
  if (canvasContext !== null) {
    const subview = canvasView.canvasRectSubview(canvasRect);
    const dirtyRect = canvasRect.toViewRect(canvasView);

    if (dirtyRect !== undefined && subview !== undefined) {
      const imageData = imageDataService.getImageDataFromCanvas(
        canvas,
        subview
      );

      canvasContext.putImageData(
        imageData,
        Number(dirtyRect.topLeft().x),
        Number(dirtyRect.topLeft().y)
      );
    }
  } else {
    console.error("Could not get canvas context");
  }
}

export function resizeCanvas(canvasView: CanvasView) {
  const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

  canvasElement.width = window.innerWidth;
  canvasElement.height = window.innerHeight;

  canvasView.resizeView(window.innerWidth, window.innerHeight);
}

export type Brush = {
  kind: "brush";
  radius: number;
  color: Pixel;
};

export type Eraser = {
  kind: "eraser";
  radius: number;
};

export type DragRectangle = {
  kind: "drag-rectangle";
  color: Pixel;
};

export type DragOval = {
  kind: "drag-oval";
  color: Pixel;
};

export type Pan = {
  kind: "pan";
};

export type DrawTool = Pan | Brush | Eraser | DragRectangle | DragOval;

export function startDragForTool(
  drawTool: DrawTool,
  mouseEvent: MouseEvent
): DragState {
  if (drawTool.kind === "drag-rectangle" || drawTool.kind === "drag-oval") {
    return {
      kind: "dragging",
      dragStart: [mouseEvent.offsetX, mouseEvent.offsetY],
    };
  } else if (
    drawTool.kind === "pan" ||
    drawTool.kind === "brush" ||
    drawTool.kind === "eraser"
  ) {
    return {
      kind: "continuous",
      lastPoint: [mouseEvent.offsetX, mouseEvent.offsetY],
    };
  } else {
    return { kind: "idle" };
  }
}
