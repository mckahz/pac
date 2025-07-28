
export const toString = ((x) =>
    (typeof x === 'number') ? '' + x
    : (typeof x === 'string') ? '"' + x + '"'
    : ('tag' in x) ? 'pack_' + x.tag + '_' + x.arity + '(' + x.args.map(toString).join(', ') + ')'
    : ''
);

