export function rand_arr (count) {

  const arr = new Float64Array(count);
  for (let i = 0; i < arr.length; i++) {
    arr[i] = Math.random();
  }
  return arr;
};