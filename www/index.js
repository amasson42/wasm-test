import { Universe } from "wasm-game-of-life";

const pre = document.getElementById("game-of-life-canvas");
const universe = Universe.new();

var ignoring = 3;
const renderLoop = () => {
    pre.textContent = universe.render();

    ignoring -= 1;
    if (ignoring < 0) {
        ignoring = 3;
        universe.tick();
    }

    requestAnimationFrame(renderLoop);
};

requestAnimationFrame(renderLoop);