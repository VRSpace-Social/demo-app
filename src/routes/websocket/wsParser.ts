function parseJSONWS(message: string): JSON {
  try {
    return JSON.parse(message);
  } catch (error) {
    throw new Error('Invalid message');
  }
}