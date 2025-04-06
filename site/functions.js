register_plugin = function(importObject) {
    // make perform_demo() function available to call from rust
    importObject.env.open_file_js = open_file;
}
miniquad_add_plugin({ register_plugin })


async function open_file() {
    const input = document.createElement('input');
    input.type = 'file';
    const data = await new Promise((resolve) => {
        input.onchange = async e => {
            const target = e.target;

            if (!target.files || target.files.length === 0) {
                return;
            }
            const data = new Uint8Array(await (await fileToBlob(target.files[0])).arrayBuffer());
            resolve(data);
        }
        input.click();
    });
    const decoder = new TextDecoder("utf-8");
    const text = decoder.decode(data);
    return js_object(text);
}
