import "./style.css";

import "../pkg/mboard_client_bg.wasm";
import init, { Canvas, CanvasView, Pixel } from "../pkg/mboard_client";
import {
  DrawTool,
  renderCanvas,
  resizeCanvas,
  startDragForTool,
  updateCanvas,
} from "./canvas";
import { createBrushAction, DragState } from "./interactions";

await init();

const canvas = new Canvas();
const canvasView = new CanvasView(window.innerWidth, window.innerHeight);

let dragState: DragState = { kind: "idle" };
let drawTool: DrawTool = {
  kind: "brush",
  color: Pixel.newRgb(255, 0, 0),
  radius: 50,
};

resizeCanvas(canvasView);
//window.onresize = () => resizeCanvas(canvasView);

document.body.onkeydown = (e: KeyboardEvent) => {
  switch (e.code) {
    case "ArrowDown": {
      canvasView.pinScaleCanvas(0.9, 0.9);
      renderCanvas(canvas, canvasView);
      break;
    }
    case "ArrowUp": {
      canvasView.pinScaleCanvas(1.1, 1.1);
      renderCanvas(canvas, canvasView);
      break;
    }
    case "KeyP": {
      drawTool = { kind: "pan" };
      break;
    }
    case "KeyB": {
      drawTool = { kind: "brush", color: Pixel.newRgb(255, 0, 0), radius: 50 };
      break;
    }
  }
};

const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
canvasElement.onmousedown = (e: MouseEvent) => {
  dragState = startDragForTool(drawTool, e);
  console.log(dragState);
  if (drawTool.kind === "brush") {
    const brushAction = createBrushAction(e, canvasView, drawTool);
    const canvasRect = canvas.performRasterAction(0, brushAction);

    if (canvasRect !== undefined) {
      updateCanvas(canvas, canvasRect, canvasView);
    }
  } else if (drawTool.kind === "eraser") {
  }
};

canvasElement.onmousemove = (e: MouseEvent) => {
  if (dragState.kind === "continuous") {
    switch (drawTool.kind) {
      case "pan": {
        const delta = [
          e.offsetX - dragState.lastPoint[0],
          e.offsetY - dragState.lastPoint[1],
        ];

        console.log(e.offsetX, e.offsetY, dragState);
        canvasView.translate(BigInt(-delta[0]), BigInt(-delta[1]));

        dragState.lastPoint = [e.offsetX, e.offsetY];
        renderCanvas(canvas, canvasView);
        break;
      }
      case "brush": {
        if (e.buttons & 0b1) {
          const brushAction = createBrushAction(e, canvasView, drawTool);
          const canvasRect = canvas.performRasterAction(0, brushAction);

          if (canvasRect !== undefined) {
            updateCanvas(canvas, canvasRect, canvasView);
          }
        }
      }
    }
  }
};

canvasElement.onmouseup = (e: MouseEvent) => {
  dragState = { kind: "idle" };
};

canvas.addRasterLayer();
