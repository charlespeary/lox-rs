export function range<T>(start: number, end: number, fill: T): T[] {
  if (start === end) return [fill];
  return [fill, ...range(start + 1, end, fill)];
}

export async function runCode(code: string) {
  const { execute } = await import("../../pkg");
  execute(code);
}
