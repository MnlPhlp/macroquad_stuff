register_plugin = function(importObject) {
    // make perform_demo() function available to call from rust
    importObject.env.open_file_js = open_file;
}
miniquad_add_plugin({ register_plugin })

async function fileToText(file) {
    const reader = new FileReader();
    return new Promise((resolve, reject) => {
        reader.onload = () => resolve(reader.result);
        reader.onerror = reject;
        reader.readAsText(file);
    });
}


async function open_file() {
    const input = document.createElement('input');
    input.type = 'file';
    const text = await new Promise((resolve) => {
        input.onchange = async e => {
            const target = e.target;

            if (!target.files || target.files.length === 0) {
                return;
            }
            const text = await fileToText(target.files[0]);
            resolve(text);
        }
        input.click();
    });
    wasm_exports.string_response(js_object(text));
}
