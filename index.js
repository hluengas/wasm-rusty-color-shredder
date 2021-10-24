const { Canvas } = require('./pkg');

const color_shredder = import('./pkg');
const html_canvas = document.getElementById('html_canvas');
const gl_canvas = html_canvas.getContext('webgl');


color_shredder.then(wasm => {

    // Check for valid gl_canvas_context
    if (!gl_canvas) {
        alert("Failed it initialize WebGL.");
        return;
    }

    // inittialize framerate values
    const FRAMETIME_LIMIT = 1000.0 / 30.0;
    var previous_time = -1;

    // initialize working canvas
    const color_shredder_canvas = new wasm.Canvas();
    const initial_time = Date.now();

    // render loop
    function render() {
        window.requestAnimationFrame(render);
        const current_time = Date.now();

        if (previous_time + FRAMETIME_LIMIT <= current_time) {
            previous_time = current_time;

            // handle window resize
            if (window.innerHeight != html_canvas.height || window.innerWidth != html_canvas.width) {
                html_canvas.height = window.innerHeight;
                html_canvas.clientHeight = window.innerHeight;
                html_canvas.style.height = window.innerHeight;

                html_canvas.width = window.innerWidth;
                html_canvas.clientWidth = window.innerWidth;
                html_canvas.style.width = window.innerWidth;

                gl_canvas.viewport(0, 0, window.innerWidth, window.innerHeight);
            }

            let elapsed_time = current_time - initial_time;
            // Rust Update Call
            color_shredder_canvas.update(elapsed_time, window.innerHeight, window.innerWidth);
            // Rust Render Call
            color_shredder_canvas.render();
        }
    }

    render();

}).catch(console.error);
