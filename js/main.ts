import './style.css';

import '../pkg/mboard_client_bg.wasm';
import init, { Canvas, CanvasView, RasterLayerAction, Pixel, CanvasRect } from '../pkg/mboard_client';
import { DrawTool, renderCanvas, resizeCanvas, toolStartsDrag } from './canvas';
import { DragState } from './interactions';

await init();

const canvas = new Canvas();
const canvasView = new CanvasView(window.innerWidth, window.innerHeight);

let dragState: DragState = { kind: 'idle' };
let drawTool: DrawTool = { kind: 'brush', color: Pixel.newRgb(255, 0, 0), radius: 50 };

resizeCanvas(canvasView);
//window.onresize = () => resizeCanvas(canvasView);

document.body.onkeydown = (e: KeyboardEvent) => {
    switch (e.code) {
            case "ArrowDown": {
                canvasView.pinScaleCanvas(0.9, 0.9);
                break;
            }
            case "ArrowUp": {
                canvasView.pinScaleCanvas(1.1, 1.1);
                break;
            }
    }

    renderCanvas(canvas, canvasView);
};

const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;
canvasElement.onmousedown = (e: MouseEvent) => {
    if (toolStartsDrag(drawTool)) {
        dragState = { kind: 'dragging', dragStart: [e.offsetX, e.offsetY] };
    } else if (drawTool.kind === 'brush') {
        const canvasPosition = canvasView.transformViewToCanvas(e.offsetX, e.offsetY);
        const radius = drawTool.radius;

        const topLeft = [Number(canvasPosition.x) - radius, Number(canvasPosition.y) - radius];
        const brushRect = new CanvasRect(BigInt(topLeft[0]), BigInt(topLeft[1]), radius * 2, radius * 2);

        console.log(topLeft);
        console.log(brushRect.dbg());
        const brushFill = RasterLayerAction.fillOval(brushRect, drawTool.color);
        canvas.performRasterAction(0, brushFill);

        renderCanvas(canvas, canvasView);
    } else if (drawTool.kind === 'eraser') {

    }
};

canvasElement.onmouseup = (e: MouseEvent) => {
    if (toolStartsDrag(drawTool)) {
        if (dragState.kind === 'dragging') {
            const dragStart = dragState.dragStart;
            const dragEnd = [e.offsetX, e.offsetY];

            //const canvasDragStart = canvasView.transformViewToCanvas(dragStart[0], dragStart[1]);
            //const canvasDragEnd = canvasView.transformViewToCanvas(dragEnd[0], dragEnd[1]);
        }
    }
};

canvas.addRasterLayer();

const canvasRect = new CanvasRect(BigInt(5), BigInt(5), 500, 500);
const pixel = Pixel.newRgb(255, 0, 0);

const fillRedOval = RasterLayerAction.fillOval(canvasRect, pixel);

canvas.performRasterAction(0, fillRedOval);

renderCanvas(canvas, canvasView);
