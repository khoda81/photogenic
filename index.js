import("./pkg")
    .then((wasm) => {
        wasm.greet();
        const canvas = document.getElementById("drawing");
        const ctx = canvas.getContext("2d");

        const realInput = document.getElementById("real");
        const imaginaryInput = document.getElementById("imaginary");
        const renderBtn = document.getElementById("render");

        const render = () => {
            // const real = parseFloat(realInput.value) || 0;
            // const imaginary = parseFloat(imaginaryInput.value) || 0;
            // wasm.draw(ctx, 600, 600, real, imaginary);
            wasm.draw_colors();
        };

        realInput.addEventListener("change", render);
        imaginaryInput.addEventListener("change", render);
        renderBtn.addEventListener("click", render);

        render();
    })
    .catch(console.error);
