use std::iter::Peekable;
use yggdrasil_error::YggdrasilError;

pub type PrecedenceNumber = u16;

#[derive(Copy, Clone)]
pub enum Associativity {
    Left,
    Right,
    Neither,
}

#[derive(PartialEq, PartialOrd, Copy, Clone)]
pub struct Precedence(PrecedenceNumber);

impl Precedence {
    pub const fn new(i: PrecedenceNumber) -> Self {
        Self(i)
    }

    const fn raise(mut self) -> Precedence {
        self.0 += 1;
        self
    }
    const fn lower(mut self) -> Precedence {
        self.0 -= 1;
        self
    }
    const fn normalize(mut self) -> Precedence {
        self.0 *= 10;
        self
    }
    const fn min() -> Precedence {
        Precedence(PrecedenceNumber::MIN)
    }
    const fn max() -> Precedence {
        Precedence(PrecedenceNumber::MAX)
    }
}

#[derive(Copy, Clone)]
pub enum Affix {
    Infix(Precedence, Associativity),
    Prefix(Precedence),
    Suffix(Precedence),
    None,
}

impl Affix {
    pub fn infix(p: PrecedenceNumber, a: Associativity) -> Self {
        Self::Infix(Precedence(p), a)
    }
    pub fn suffix(p: PrecedenceNumber) -> Self {
        Self::Prefix(Precedence(p))
    }
    pub fn prefix(p: PrecedenceNumber) -> Self {
        Self::Prefix(Precedence(p))
    }
}

pub trait PrattParser<Inputs>
where
    Inputs: Iterator<Item = Self::Input>,
{
    type Input;
    type Output: Sized;

    fn query(&mut self, input: &Self::Input) -> Result<Affix, YggdrasilError>;

    fn primary(&mut self, input: Self::Input) -> Result<Self::Output, YggdrasilError>;

    fn infix(&mut self, lhs: Self::Output, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, YggdrasilError>;

    fn prefix(&mut self, op: Self::Input, rhs: Self::Output) -> Result<Self::Output, YggdrasilError>;

    fn suffix(&mut self, lhs: Self::Output, op: Self::Input) -> Result<Self::Output, YggdrasilError>;

    fn parse(&mut self, inputs: &mut Inputs) -> Result<Self::Output, YggdrasilError> {
        self.parse_input(&mut inputs.peekable(), Precedence(0))
    }

    fn parse_input(&mut self, tail: &mut Peekable<&mut Inputs>, rbp: Precedence) -> Result<Self::Output, YggdrasilError> {
        if let Some(head) = tail.next() {
            let info = self.query(&head)?;
            let mut nbp = self.nbp(info);
            let mut node = self.nud(head, tail, info);
            while let Some(head) = tail.peek() {
                let info = self.query(head)?;
                let lbp = self.lbp(info);
                if rbp < lbp && lbp < nbp {
                    let head = tail.next().unwrap();
                    nbp = self.nbp(info);
                    node = self.led(head, tail, info, node?);
                }
                else {
                    break;
                }
            }
            node
        }
        else {
            Err(YggdrasilError::unexpected_token("EmptyInput"))
        }
    }

    /// Null-Denotation
    fn nud(
        &mut self,
        head: Self::Input,
        tail: &mut Peekable<&mut Inputs>,
        info: Affix,
    ) -> Result<Self::Output, YggdrasilError> {
        match info {
            Affix::Prefix(precedence) => {
                let rhs = self.parse_input(tail, precedence.normalize().lower());
                self.prefix(head, rhs?)
            }
            Affix::None => self.primary(head),
            Affix::Suffix(_) => Err(YggdrasilError::unexpected_token("Unexpected Postfix")),
            Affix::Infix(_, _) => Err(YggdrasilError::unexpected_token("Unexpected Infix")),
        }
    }

    /// Left-Denotation
    fn led(
        &mut self,
        head: Self::Input,
        tail: &mut Peekable<&mut Inputs>,
        info: Affix,
        lhs: Self::Output,
    ) -> Result<Self::Output, YggdrasilError> {
        match info {
            Affix::Infix(precedence, associativity) => {
                let precedence = precedence.normalize();
                let rhs = match associativity {
                    Associativity::Left => self.parse_input(tail, precedence),
                    Associativity::Right => self.parse_input(tail, precedence.lower()),
                    Associativity::Neither => self.parse_input(tail, precedence.raise()),
                };
                self.infix(lhs, head, rhs?)
            }
            Affix::Suffix(_) => self.suffix(lhs, head),
            Affix::None => Err(YggdrasilError::unexpected_token("Unexpected NilFix")),
            Affix::Prefix(_) => Err(YggdrasilError::unexpected_token("Unexpected Prefix")),
        }
    }

    //         <lbp>  <rbp>  <nbp> <kind>
    // Nilfix:  MIN |  MIN |  MAX | nud
    // Prefix:  MIN |   bp |  MAX | nud
    // Postfix:  bp |  MIN |  MAX | led
    // InfixL:   bp |   bp | bp+1 | led
    // InfixR:   bp | bp-1 | bp+1 | led
    // InfixN:   bp |   bp |   bp | led

    /// Left-Binding-Power
    fn lbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::None => Precedence::min(),
            Affix::Prefix(_) => Precedence::min(),
            Affix::Suffix(precedence) => precedence.normalize(),
            Affix::Infix(precedence, _) => precedence.normalize(),
        }
    }

    /// Next-Binding-Power
    fn nbp(&mut self, info: Affix) -> Precedence {
        match info {
            Affix::None => Precedence::max(),
            Affix::Prefix(_) => Precedence::max(),
            Affix::Suffix(_) => Precedence::max(),
            Affix::Infix(precedence, Associativity::Left) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Right) => precedence.normalize().raise(),
            Affix::Infix(precedence, Associativity::Neither) => precedence.normalize(),
        }
    }
}
