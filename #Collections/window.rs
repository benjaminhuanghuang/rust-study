// windows(n) This method returns an iterator over a slice. 
// The iterator length can be specified.
let slice = ['w', 'i', 'n', 'd', 'o', 'w', 's'];
for window in slice.windows(2) {
  &println!{"[{}, {}]", window[0], window[1]};
}
// prints: [w, i] -> [i, n] -> [n, d] -> [d, o] -> [o, w] -> [w, s]