import("./pkg")
    .then((wasm) => {
        const canvas = document.getElementById("drawing");
        const ctx = canvas.getContext("2d");

        const populationInput = document.getElementById("population");
        const colorInput = document.getElementById("colors");
        const resetBtn = document.getElementById("reset");

        let numColors = parseInt(colorInput.value);

        let algo;
        function reset() {
            algo = wasm.initiate_algorithm(numColors);
            algo.set_population_size(parseInt(populationInput.value));
            return algo;
        }

        const render = () => {
            wasm.render_best(ctx, algo, canvas.clientWidth, canvas.clientHeight);
            requestAnimationFrame(render);
        };

        resetBtn.addEventListener("click", reset);

        colorInput.addEventListener("change", () => (numColors = parseInt(colorInput.value)));
        populationInput.addEventListener("change", () => algo.set_population_size(parseInt(populationInput.value)));

        document.addEventListener("keydown", (event) => {});

        function resizeCanvas() {
            canvas.width = canvas.clientWidth;
            canvas.height = canvas.clientHeight;
        }

        reset();
        render();

        setInterval(() => {
            algo.step();
        }, 0);

        resizeCanvas();

        window.addEventListener("resize", resizeCanvas);
    })
    .catch(console.error);
