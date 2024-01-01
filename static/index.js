import "./css/normalize.css";
import "./css/skeleton.css";
import "./css/style.css";

import("../pkg")
    .then((wasm) => {
        const canvas = document.getElementById("drawing");
        const ctx = canvas.getContext("2d");

        const populationInput = document.getElementById("population");
        const colorInput = document.getElementById("colors");
        const mutationInput = document.getElementById("mutation_rate");
        const resetBtn = document.getElementById("reset");

        let numColors = parseInt(colorInput.value);

        let algo = wasm.initiate_algorithm(numColors);
        algo.set_population_size(parseInt(populationInput.value));

        function reset() {
            algo = wasm.initiate_algorithm(numColors);
            algo.set_population_size(parseInt(populationInput.value));
        }

        const render = () => {
            wasm.render_best(ctx, algo, canvas.clientWidth, canvas.clientHeight);
            requestAnimationFrame(render);
        };

        function setMutationRate() {
            const probability = parseFloat(mutationInput.value);
            algo.mutation_rate = Math.max(Math.min(probability, 1.0), 0.0);
        }

        resetBtn.addEventListener("click", reset);

        colorInput.addEventListener("change", () => (numColors = parseInt(colorInput.value)));
        populationInput.addEventListener("change", () => algo.set_population_size(parseInt(populationInput.value)));
        mutationInput.addEventListener("change", () => setMutationRate());

        document.addEventListener("keydown", (event) => {});

        function resizeCanvas() {
            canvas.width = canvas.clientWidth;
            canvas.height = canvas.clientHeight;
        }

        render();
        setMutationRate();
        setInterval(() => {
            algo.step();
        }, 0);

        resizeCanvas();

        window.addEventListener("resize", resizeCanvas);
    })
    .catch(console.error);
