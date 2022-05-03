import './style.css';

import '../pkg/mboard_client_bg.wasm';
import init, { Canvas, RasterChunk, CanvasView, RasterLayerAction, Pixel, CanvasRect } from '../pkg/mboard_client';

function renderChunk(chunk: RasterChunk) {
    const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

    const canvasContext = canvasElement.getContext("2d");
    if (canvasContext !== null) {
        canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

        const canvasImageData = canvasContext.createImageData(chunk.width(), chunk.height());

        canvasImageData.data.set(chunk.imageData());

        // Clear the canvas
        canvasContext.clearRect(0, 0, canvasElement.width, canvasElement.height);

        // Place the new generated checkerboard onto the canvas
        canvasContext.putImageData(canvasImageData, 0, 0);
    } else {
        console.error("Could not get canvas context");
    }

}

function renderCanvas(canvas: Canvas, view: CanvasView) {
    const raster = canvas.render(view);

    renderChunk(raster);
}

await init();


const canvas = new Canvas();
const canvasView = new CanvasView(window.innerWidth, window.innerHeight);

function resizeCanvas() {
    const canvasElement = document.getElementById("canvas") as HTMLCanvasElement;

    canvasElement.width = window.innerWidth;
    canvasElement.height = window.innerHeight;

    canvasView.resizeView(window.innerWidth, window.innerHeight);

}

resizeCanvas();

window.onresize = resizeCanvas;

document.body.onkeydown = (e: KeyboardEvent) => {
    const TRANSLATE_DELTA = 1;
    switch (e.code) {
            case "ArrowDown": {
                canvasView.translate(BigInt(0), BigInt(-TRANSLATE_DELTA));
                canvasView.pinScaleCanvas(0.9, 0.9);
                break;
            }
            case "ArrowUp": {
                canvasView.translate(BigInt(0), BigInt(TRANSLATE_DELTA));
                break;
            }
            case "ArrowLeft":
                canvasView.translate(BigInt(TRANSLATE_DELTA), BigInt(0));
                break;
            case "ArrowRight": {
                canvasView.translate(BigInt(-TRANSLATE_DELTA), BigInt(0));
                break;
            }
    }

    renderCanvas(canvas, canvasView);
};

canvas.addRasterLayer();

const canvasRect = new CanvasRect(BigInt(5), BigInt(5), 500, 500);
const pixel = Pixel.newRgb(255, 0, 0);

const fillRedOval = RasterLayerAction.fillOval(canvasRect, pixel);

canvas.performRasterAction(0, fillRedOval);

renderCanvas(canvas, canvasView);
