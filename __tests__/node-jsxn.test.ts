import { readFileSync } from 'fs';
import { resolve as resolvePath } from 'path';
import { parseString, parseFile } from '../pkg';

describe('node-jsxn', () => {
  it('can parse strings', () => {
    const jsxPath = resolvePath(__dirname, 'jsx_root.jsxn');

    const jsonPath = resolvePath(__dirname, 'json_root.jsxn');

    const jsxRoot = readFileSync(jsxPath, { encoding: 'utf8' });

    const jsonRoot = readFileSync(jsonPath, { encoding: 'utf8' });

    const parsedJSXRoot = {
      type: 'Hello',
      props: { count: 1, friend: 'World' },
      children: [
        { type: 'Goodbye', props: { signOff: true }, children: [] },
        'You can put text here too.',
        { 'Is it okay to put JSON values here?': true },
        {
          type: 'ExpressionInception',
          props: {
            arrayProp: [0, '1', true, null],
            nullProp: null,
            objectProp: { cool: true },
          },
          children: [],
        },
        {
          type: 'Fragment',
          children: [{ type: 'Fragment', children: ['F R A G M E N T S'] }],
        },
        'Text is fine here too.',
      ],
    };

    const parsedJSONRoot = {
      a: 42,
      b: ['this is a string', 'this is too 👍 \\u2605', 12],
      c: { hello: 'world' },
      d: null,
      e: { type: 'Element', props: { prop: 'value' }, children: [] },
    };

    expect(parseString(jsxRoot)).toEqual(parsedJSXRoot);

    expect(parseString(jsonRoot)).toEqual(parsedJSONRoot);

    expect(parseFile(jsxPath)).toEqual(parsedJSXRoot);

    expect(parseFile(jsonPath)).toEqual(parsedJSONRoot);
  });
});
