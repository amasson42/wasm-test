import { Universe, Cell, CellMotif } from "wasm-game-of-life";
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg";

const CELL_SIZE = 5;
const GRID_COLOR = "#CCCCCC";
const GRID_DEAD_COLOR = "#FFFFFF";
const GRID_ALIVE_COLOR = "#000000";

const canvas = document.getElementById("game-of-life-canvas");

var universe = Universe.new();
const width = universe.width();
const height = universe.height();

canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;

const ctx = canvas.getContext('2d');

function drawGrid() {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    for (let i = 0; i < width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    for (let j = 0; j < height; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, j * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
}

function getIndex(row, column) {
    return row * width + column;
}

function drawCells() {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();

    for (let row = 0; row < height; row++) {
        for (let column = 0; column < width; column++) {
            const idx = getIndex(row, column);

            ctx.fillStyle = cells[idx] === Cell.Dead ? GRID_DEAD_COLOR : GRID_ALIVE_COLOR;

            ctx.fillRect(
                column * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE
            );
        }
    }
    ctx.stroke();
}

let animationId = null;

var ignoring = 3;
const renderLoop = () => {
    drawGrid();
    drawCells();

    ignoring -= 1;
    if (ignoring < 0) {
        ignoring = 3;
        universe.tick();
        debugger;
    }

    animationId = requestAnimationFrame(renderLoop);
};

const isPaused = () => {
    return animationId === null;
};

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";
    renderLoop();
};

const pause = () => {
    playPauseButton.textContent = "▶";
    cancelAnimationFrame(animationId);
    animationId = null;
}

playPauseButton.addEventListener("click", event => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
});

play();

const newUniverse_lines = document.getElementById("new-universe-lines");
newUniverse_lines.addEventListener("click", event => {
    universe = Universe.new(CellMotif.Lines);
    drawGrid();
    drawCells();
});

const newUniverse_spaceship = document.getElementById("new-universe-spaceship");
newUniverse_spaceship.addEventListener("click", event => {
    universe = Universe.new(CellMotif.Spaceship);
    drawGrid();
    drawCells();
});

const newUniverse_random = document.getElementById("new-universe-random");
newUniverse_random.addEventListener("click", event => {
    universe = Universe.new(CellMotif.Random);
    drawGrid();
    drawCells();
});

canvas.addEventListener("click", event => {
    const boundingRect = canvas.getBoundingClientRect();

    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    const canvasLeft = (event.clientX - boundingRect.left) * scaleX;
    const canvasTop = (event.clientY - boundingRect.top) * scaleY;

    const row = Math.min(Math.floor(canvasTop / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(canvasLeft / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawGrid();
    drawCells();
});

