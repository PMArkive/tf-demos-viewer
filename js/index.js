import("../pkg/index.js")
    .then(m => {
        document.getElementById('file').onchange = e => {
            let file = e.target.files[0];

            let reader = new FileReader();

            reader.readAsArrayBuffer(file);

            reader.onload = function() {
                console.log(reader.result);
                let bytes = new Uint8Array(reader.result);

                console.time('demo_parse');
                m.parse_demo(bytes);
                console.timeEnd('demo_parse');
            };

            reader.onerror = function() {
                console.log(reader.error);
            };
        };
    })
    .catch(console.error);
