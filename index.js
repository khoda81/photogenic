import("./pkg")
    .then((wasm) => {
        const numColors = 64;

        const canvas = document.getElementById("drawing");
        const ctx = canvas.getContext("2d");

        const stepsInput = document.getElementById("steps");
        const populationInput = document.getElementById("population");
        const renderBtn = document.getElementById("render");
        const stepBtn = document.getElementById("step");

        function initiate() {
            let algo = wasm.initiate_algorithm(numColors);
            algo.populate(parseInt(populationInput.value));
            return algo;
        }

        const render = () => {
            wasm.render_best(ctx, algo, canvas.clientWidth, canvas.clientHeight);
            requestAnimationFrame(render);
        };

        const step = () => {
            const numSteps = parseInt(stepsInput.value);
            for (let index = 0; index < numSteps; index++) {
                algo.step();
            }
        };

        let algo = initiate();

        render();

        stepBtn.addEventListener("click", step);

        document.addEventListener("keydown", (event) => {
            if (event.code === "Space") {
                step();
                event.preventDefault();
            }
        });

        populationInput.addEventListener("change", () => (algo = initiate()));

        function resizeCanvas() {
            canvas.width = canvas.clientWidth;
            canvas.height = canvas.clientHeight;
        }

        resizeCanvas();

        window.addEventListener("resize", resizeCanvas);
    })
    .catch(console.error);
