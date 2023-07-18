use crate::parse::{MatchResult, Parser, Stateful};

pub enum NEnum<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z> {
    Nil,
    A(Option<A>),
    B(Option<B>),
    C(Option<C>),
    D(Option<D>),
    E(Option<E>),
    F(Option<F>),
    G(Option<G>),
    H(Option<H>),
    I(Option<I>),
    J(Option<J>),
    K(Option<K>),
    L(Option<L>),
    M(Option<M>),
    N(Option<N>),
    O(Option<O>),
    P(Option<P>),
    Q(Option<Q>),
    R(Option<R>),
    S(Option<S>),
    T(Option<T>),
    U(Option<U>),
    V(Option<V>),
    W(Option<W>),
    X(Option<X>),
    Y(Option<Y>),
    Z(Option<Z>),
}

struct ParserState<const SIZE: usize, T> {
    state: usize,
    parsed_elements: [T; SIZE],
}

pub struct Sequence<
    const SIZE: usize,
    A: Parser = (),
    B: Parser = (),
    C: Parser = (),
    D: Parser = (),
    E: Parser = (),
    F: Parser = (),
    G: Parser = (),
    H: Parser = (),
    I: Parser = (),
    J: Parser = (),
    K: Parser = (),
    L: Parser = (),
    M: Parser = (),
    N: Parser = (),
    O: Parser = (),
    P: Parser = (),
    Q: Parser = (),
    R: Parser = (),
    S: Parser = (),
    T: Parser = (),
    U: Parser = (),
    V: Parser = (),
    W: Parser = (),
    X: Parser = (),
    Y: Parser = (),
    Z: Parser = (),
> {
    seq:
        [NEnum<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>; SIZE],
}

impl<
        const SIZE: usize,
        A,
        B,
        C,
        D,
        E,
        F,
        G,
        H,
        I,
        J,
        K,
        L,
        M,
        N,
        O,
        P,
        Q,
        R,
        S,
        T,
        U,
        V,
        W,
        X,
        Y,
        Z,
    > Stateful<Sequence<SIZE>>
    for ParserState<
        SIZE,
        NEnum<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z>,
    >
{
    fn new() -> Self {
        ParserState {
            state: 0,
            parsed_elements: [NEnum::Nil; SIZE],
        }
    }

    fn parse(
        &mut self,
        byte: u8,
    ) -> MatchResult<
        Sequence<
            SIZE,
            A,
            B,
            C,
            D,
            E,
            F,
            G,
            H,
            I,
            J,
            K,
            L,
            M,
            N,
            O,
            P,
            Q,
            R,
            S,
            T,
            U,
            V,
            W,
            X,
            Y,
            Z,
        >,
    > {
    }
}
