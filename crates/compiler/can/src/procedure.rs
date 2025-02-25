use crate::expr::Expr;
use crate::pattern::Pattern;
use broc_module::symbol::Symbol;
use broc_region::all::{Loc, Region};
use broc_types::subs::Variable;

#[derive(Clone, Debug)]
pub struct Pbrocedure {
    pub name: Option<Box<str>>,
    pub is_self_tail_recursive: bool,
    pub definition: Region,
    pub args: Vec<Loc<Pattern>>,
    pub body: Loc<Expr>,
    pub references: References,
    pub var: Variable,
    pub ret_var: Variable,
}

impl Pbrocedure {
    pub fn new(
        definition: Region,
        args: Vec<Loc<Pattern>>,
        body: Loc<Expr>,
        references: References,
        var: Variable,
        ret_var: Variable,
    ) -> Pbrocedure {
        Pbrocedure {
            name: None,
            is_self_tail_recursive: false,
            definition,
            args,
            body,
            references,
            var,
            ret_var,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
struct ReferencesBitflags(u8);

impl ReferencesBitflags {
    const VALUE_LOOKUP: Self = ReferencesBitflags(1);
    const TYPE_LOOKUP: Self = ReferencesBitflags(2);
    const CALL: Self = ReferencesBitflags(4);
    const BOUND: Self = ReferencesBitflags(8);
}

#[derive(Clone, Debug, Default)]
pub struct References {
    symbols: Vec<Symbol>,
    bitflags: Vec<ReferencesBitflags>,
}

impl References {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn union_mut(&mut self, other: &Self) {
        for (k, v) in other.symbols.iter().zip(other.bitflags.iter()) {
            self.insert(*k, *v);
        }
    }

    // iterators

    fn retain<'a, P: Fn(&'a ReferencesBitflags) -> bool>(
        &'a self,
        pred: P,
    ) -> impl Iterator<Item = &'a Symbol> {
        self.symbols
            .iter()
            .zip(self.bitflags.iter())
            .filter_map(move |(a, b)| if pred(b) { Some(a) } else { None })
    }

    pub fn value_lookups(&self) -> impl Iterator<Item = &Symbol> {
        self.retain(|b| b.0 & ReferencesBitflags::VALUE_LOOKUP.0 > 0)
    }

    pub fn type_lookups(&self) -> impl Iterator<Item = &Symbol> {
        self.retain(|b| b.0 & ReferencesBitflags::TYPE_LOOKUP.0 > 0)
    }

    pub fn bound_symbols(&self) -> impl Iterator<Item = &Symbol> {
        self.retain(|b| b.0 & ReferencesBitflags::BOUND.0 > 0)
    }

    pub fn calls(&self) -> impl Iterator<Item = &Symbol> {
        self.retain(|b| b.0 & ReferencesBitflags::CALL.0 > 0)
    }

    // insert

    fn insert(&mut self, symbol: Symbol, flags: ReferencesBitflags) {
        match self.symbols.iter().position(|x| *x == symbol) {
            None => {
                self.symbols.push(symbol);
                self.bitflags.push(flags);
            }
            Some(index) => {
                // idea: put some debug_asserts in here?
                self.bitflags[index].0 |= flags.0;
            }
        }
    }

    pub fn insert_value_lookup(&mut self, symbol: Symbol) {
        self.insert(symbol, ReferencesBitflags::VALUE_LOOKUP);
    }

    pub fn insert_type_lookup(&mut self, symbol: Symbol) {
        self.insert(symbol, ReferencesBitflags::TYPE_LOOKUP);
    }

    pub fn insert_bound(&mut self, symbol: Symbol) {
        self.insert(symbol, ReferencesBitflags::BOUND);
    }

    pub fn insert_call(&mut self, symbol: Symbol) {
        self.insert(symbol, ReferencesBitflags::CALL);
    }

    // remove

    pub fn remove_value_lookup(&mut self, symbol: &Symbol) {
        match self.symbols.iter().position(|x| x == symbol) {
            None => {
                // it's not in there; do nothing
            }
            Some(index) => {
                // idea: put some debug_asserts in here?
                self.bitflags[index].0 ^= ReferencesBitflags::VALUE_LOOKUP.0;
            }
        }
    }

    // contains

    pub fn has_value_lookup(&self, symbol: Symbol) -> bool {
        // println!("has a value lookup? {} {:?}", self.symbols.len(), symbol);
        let it = self.symbols.iter().zip(self.bitflags.iter());

        for (a, b) in it {
            if *a == symbol && b.0 & ReferencesBitflags::VALUE_LOOKUP.0 > 0 {
                return true;
            }
        }

        false
    }

    fn has_type_lookup(&self, symbol: Symbol) -> bool {
        let it = self.symbols.iter().zip(self.bitflags.iter());

        for (a, b) in it {
            if *a == symbol && b.0 & ReferencesBitflags::TYPE_LOOKUP.0 > 0 {
                return true;
            }
        }

        false
    }

    pub fn has_type_or_value_lookup(&self, symbol: Symbol) -> bool {
        let mask = ReferencesBitflags::VALUE_LOOKUP.0 | ReferencesBitflags::TYPE_LOOKUP.0;
        let it = self.symbols.iter().zip(self.bitflags.iter());

        for (a, b) in it {
            if *a == symbol && b.0 & mask > 0 {
                return true;
            }
        }

        false
    }

    pub fn references_type_def(&self, symbol: Symbol) -> bool {
        self.has_type_lookup(symbol)
    }
}
