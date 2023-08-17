window.b64_bench = function (items, itemsBytes) {
  let result = new Array(items);

  for (let i = 0; i < items; i++) {
    let item = new Uint8Array(itemsBytes);
    for (let j = 0; j < itemsBytes; j++) {
      item[j] = Math.round(Math.random() * 255);
    }

    result.push(window.base64js.fromByteArray(item));
  }

  return result.reduce((acc, item) => acc + item.length, 0);
};
