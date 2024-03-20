
export function assert(paramOne, paramTwo) {
  if (paramOne === paramTwo) {
    return true;
  }
  // throw error values are not equal
  throw Error('Values are not equal param one: ' + paramOne + ', paramTwo: ' + paramTwo);
}
