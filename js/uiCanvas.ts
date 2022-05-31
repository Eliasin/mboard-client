import { Canvas, CanvasView } from "../pkg/mboard_client";

enum Layers {
  DRAG_HINT = 1,
  SELECTION = 2,
}

export class UiCanvas {
  canvas: Canvas;

  constructor() {
    this.canvas = new Canvas();

    // DragHint
    this.canvas.addRasterLayer();
    // Selection
    this.canvas.addRasterLayer();
  }

  public clearCanvas() {}
}
