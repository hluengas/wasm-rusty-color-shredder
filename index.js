const color_shredder = import('./pkg');
const js_canvas = document.getElementById('color_shredder_canvas');
const canvas_context = js_canvas.getContext('2d');


color_shredder.then(wasm => {
    var canvas = wasm.Canvas.new(512, 512);
    // wasm.populate_test_image(canvas);
    wasm.render_canvas(canvas,canvas_context);
}).catch(console.error);
