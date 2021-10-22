import('./pkg')
    .then(wasm_rusty_color_shredder => {
        const color_shredder_canvas = document.getElementById('color_shredder_canvas');
        const canvas_context = color_shredder_canvas.getContext('2d');

        wasm_rusty_color_shredder.main(canvas_context, 512, 512);
    })
    .catch(console.error);
