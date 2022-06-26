export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'mul' : IDL.Func([IDL.Nat], [IDL.Nat], []),
    'read' : IDL.Func([], [IDL.Nat], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
