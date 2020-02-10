export function parseResponse(response) {
  const object = typeof response === 'string' ? JSON.parse(response) : response;

  return object.hasOwnProperty('Ok') ? object.Ok : object;
}

export function parseEntry(entry) {
  return JSON.parse(parseResponse(entry).App[1]);
}
