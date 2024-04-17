
import React, { useRef, useState } from 'react';
import { render } from 'react-dom';


import example from './example.jpg';


const App = () => {

  const [w, setW] = useState(50);
  const [h, setH] = useState(40);
  const [s, setS] = useState(30);

  const ref = useRef<HTMLCanvasElement>(null);

  return <div>
    <h3>Perform Ray Tracing using WebAssembly</h3>
    <div>
      <span>width:</span>
      <input
        type='number'
        value={w}
        onChange={e => {
          const v = e.target.value;
          const n = parseInt(v);
          if (n.toString() === v) { setW(n); setH(Math.floor(n * 0.8)); }
        }}
      />
    </div>

    <div >
      <span>height:</span>
      <input
        type='number'
        value={h.toString()}
        onChange={e => {
          const v = e.target.value;
          const n = parseInt(v);
          if (n.toString() === v) { setH(h); setW(Math.floor(n / 0.8)); }
        }}
      />
    </div>

    <div>
      <span>samps:</span>
      <input
        type='number'
        value={s.toString()}
        onChange={e => {
          const v = e.target.value;
          const n = parseInt(v);
          if (n.toString() === v) { setS(n) }
        }}
      />
    </div>

    <div>
      <button onClick={() => {

        const worker = new Worker(new URL('./worker.ts', import.meta.url));

        worker.onmessage = (event) => {
          const data = event.data as unknown;

          if (
            typeof data === 'object' && data !== null &&
            'row' in data && typeof data.row === 'number' &&
            'pixels' in data && data.pixels instanceof Uint8ClampedArray
          ) {
            if (ref.current) {
              const ctx = ref.current.getContext('2d');
              if (ctx) {
                const imageData = new ImageData(data.pixels, w, 1);
                ctx.putImageData(imageData, 0, h - 1 - data.row);
              }
            }
          }
        };

        worker.postMessage({ type: "start", width: w, height: h });
      }}>run</button>
    </div>

    <div style={{ height: 20 }}></div>

    <div style={{
      display: 'flex'
    }}>
      <div>
        <div>Canvas</div>
        <canvas ref={ref} style={{ border: '1px solid #ccc', width: w * 4, height: h * 4 }} width={w} height={h}></canvas>
      </div>
      <div style={{ marginLeft: 20 }}>
        <div>Image</div>
        <img style={{ width: w * 4, height: h * 4 }} src={example as string}></img>
      </div>

    </div>

  </div>;
};

const root = document.createElement('div');

document.body.appendChild(root);

render(<App />, root);
