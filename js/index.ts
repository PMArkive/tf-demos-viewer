import {parseDemo} from "./parser";

window.addEventListener('DOMContentLoaded', () => {
    document.getElementById('file').onchange = e => {
        let file = (e.target as HTMLInputElement).files[0];
        let reader = new FileReader();
        reader.readAsArrayBuffer(file);

        reader.onload = async function () {
            let bytes = new Uint8Array(reader.result as ArrayBuffer);

            console.time('demo_parse');
            let parsed = await parseDemo(bytes);
            console.timeEnd('demo_parse');
            console.log(parsed.getPlayer(150, 2));
        };

        reader.onerror = function () {
            console.log(reader.error);
        };
    };
});