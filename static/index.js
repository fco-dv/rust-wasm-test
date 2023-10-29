import init, { Universe, Cell } from "../pkg/rust_webpack_template.js";

let wasm;
let memory;

async function run() {
    wasm = await init();
    memory = wasm.memory;

    const CELL_SIZE = 12; // px
    const GRID_COLOR = "#CCCCCC";
    const DEAD_COLOR = "#FFFFFF";
    const ALIVE_COLOR = "#000000";
    
    
    const universe = Universe.new();
    const width = universe.width();
    const height = universe.height();
    
    
    // Give the canvas room for all of our cells and a 1px border
    // around each of them.
    const canvas = document.getElementById("game-of-life-canvas");
    canvas.height = (CELL_SIZE + 1) * height + 1;
    canvas.width = (CELL_SIZE + 1) * width + 1;
    
    const ctx = canvas.getContext('2d');
    
    canvas.addEventListener("click", event => {
      const boundingRect = canvas.getBoundingClientRect();
      const scaleX = canvas.width/boundingRect.width;
      const scaleY = canvas.height/boundingRect.height;
    
      const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
      const canvasTop = (event.clientY - boundingRect.top) * scaleY;
    
      const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
      const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);
    
      universe.toggle_cell(row, col);
    
      drawGrid();
      drawCells();
    });
    
    let animationId = null;
    const renderLoop = () => {
      drawGrid();
      drawCells();
    
      universe.tick();
    
      animationId = requestAnimationFrame(renderLoop);
    };
    
    const playPauseButton = document.getElementById("play-pause");
    
    const isPaused = () => {
      return animationId === null;
    };
}
run();
