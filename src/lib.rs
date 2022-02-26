use std::{collections::HashMap, marker::PhantomData};

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

struct VDom {
    tag: String,
    attrs: HashMap<String, String>,
    children: Vec<VDom>,
}

struct UI {
    dom: VDom,
}

pub trait ImpureOperation<State> {
    fn get_state(&self) -> State;
    fn reform<Next,Func : Fn(State) -> Next>(&self,func: Func) -> Reformed<State,Next,Func,Self> where Self: Sized;
}

pub struct Reformed<P,C,Func : Fn(P) -> C,PrevOp:ImpureOperation<P>> {
    op: PrevOp,
    func: Func,
    __mark: PhantomData<(P,C)>,
}

impl <P,C,Func : Fn(P) -> C,PrevOp:ImpureOperation<P>> ImpureOperation<C> for Reformed<P,C,Func,PrevOp> {
    fn get_state(&self) -> C {
        todo!()
    }
    fn reform<Next,Func2 : Fn(C) -> Next>(&self,func: Func2) -> Reformed<C,Next,Func2,Self> where Self: Sized {
        todo!()
    }
}
    

struct State<S,M,U: Fn(S,M) -> S> {
    state: S,
    update: U,
    __mark: PhantomData<M>
}

impl <S,M,U: Fn(S,M)  -> S> State<S,M,U> {
    fn new(state: S, update: U) -> Self {
        Self {
            state,
            update,
            __mark: PhantomData
        }
    }
}

type StateWithUpdate<S> = (S,Box<dyn FnMut(S)>);

impl <S,M,U: Fn(S,M) -> S>ImpureOperation<StateWithUpdate<S>> for State<S,M,U> {
    fn get_state(&self) -> (S, Box<(dyn FnMut(S) + 'static)>) {
        todo!()
    }

    fn reform<Next,Func : Fn(StateWithUpdate<S>) -> Next>(&self,func: Func) -> Reformed<StateWithUpdate<S>,Next,Func,Self> {
        todo!()
    }
}

pub struct SampleContainerProps {
    pub name: String,
}


pub fn sample_effect(props: SampleContainerProps) -> impl ImpureOperation<i32> {
    let state = State::new(0, |cur,plus: i32| { cur + plus });
    state.reform(|(state,update)| state)
}

pub fn connect<R: Fn() -> UI>(render: R){

}

