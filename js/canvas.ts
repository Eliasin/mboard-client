import { Canvas, RasterChunk, CanvasView, CanvasRect, Pixel } from '../pkg/mboard_client';
import { DragState } from './interactions';

export function renderChunk(chunk: RasterChunk) {
    const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

    const canvasContext = canvasElement.getContext("2d");
    if (canvasContext !== null) {
        const canvasImageData = canvasContext.createImageData(chunk.width(), chunk.height());

        canvasImageData.data.set(chunk.imageData());

        canvasContext.putImageData(canvasImageData, 0, 0);
    } else {
        console.error("Could not get canvas context");
    }

}

export function renderCanvas(canvas: Canvas, view: CanvasView) {
    const raster = canvas.render(view);

    renderChunk(raster);
}

export function updateCanvas(canvas: Canvas, canvasRect: CanvasRect, canvasView: CanvasView) {
    const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

    const canvasContext = canvasElement.getContext("2d");
    if (canvasContext !== null) {
        const dirtyRect = canvasRect.toViewRect(canvasView);

        if (dirtyRect !== undefined) {
            const canvasImageData = canvasContext.createImageData(dirtyRect.width(), dirtyRect.height());

            const chunk = canvas.rasterizeCanvasRect(canvasRect);
            canvasImageData.data.set(chunk.imageData());

            canvasContext.putImageData(canvasImageData, Number(dirtyRect.topLeft().x), Number(dirtyRect.topLeft().y));
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
    kind: 'brush';
    radius: number;
    color: Pixel;
};

export type Eraser = {
    kind: 'eraser';
    radius: number;
};

export type DragRectangle = {
    kind: 'drag-rectangle';
    color: Pixel;
};

export type DragOval = {
    kind: 'drag-oval';
    color: Pixel;
};

export type Pan = {
    kind: 'pan';
};

export type DrawTool = Pan | Brush | Eraser | DragRectangle | DragOval;

export function startDragForTool(drawTool: DrawTool, mouseEvent: MouseEvent): DragState {
    if (drawTool.kind === 'drag-rectangle' || drawTool.kind === 'drag-oval') {
        return { kind: 'dragging', dragStart: [mouseEvent.offsetX, mouseEvent.offsetY] };
    } else if (drawTool.kind === 'pan' || drawTool.kind === 'brush' || drawTool.kind === 'eraser') {
        return { kind: 'continuous', lastPoint: [mouseEvent.offsetX, mouseEvent.offsetY] };
    } else {
        return { kind: 'idle' };
    }
}
