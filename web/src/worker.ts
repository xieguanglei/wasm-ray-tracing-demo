self.onmessage = (event) => {

  const { data } = event;

  if (typeof data === 'object' && data !== null) {

    if (
      'type' in data && data.type === 'start' &&
      'width' in data && typeof data.width === 'number' &&
      'height' in data && typeof data.height === 'number'
    ) {

      import('../../wasm/pkg/ray_tracing')
        .then(wasm => {
          wasm.run(
            data.width, 
            data.height, 
            30,
            function (...args: any[]) {
              if (args.length === 2 && typeof args[0] === 'number' && args[1] instanceof Uint8ClampedArray) {
                self.postMessage({
                  row: args[0],
                  pixels: args[1]
                })
              }
            }
          );

          self.postMessage("end");
        });
    }
  }
};