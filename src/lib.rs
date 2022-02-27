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
    fn call_effect(&mut self);
    fn reform<Next,Func : Fn(State) -> Next>(self,func: Func) -> Reformed<State,Next,Func,Self> where Self: Sized {
        todo!()
    }
    fn merge<OS,OO: ImpureOperation<OS>,Next,Func: Fn(State,OS) -> Next>(self,other: OO,func: Func) -> Merged<State,OS,Next,Func,Self,OO> where Self: Sized {
        todo!()
    }
}

pub struct Merged<S1,S2,S3,Func: Fn(S1,S2) -> S3, Op1: ImpureOperation<S1>,Op2: ImpureOperation<S2>> {
    op1: Op1,
    op2: Op2,
    func: Func,
    __marker: PhantomData<(S1,S2)>,
}

impl <S1,S2,S3,Func: Fn(S1,S2) -> S3, Op1: ImpureOperation<S1>,Op2: ImpureOperation<S2>> ImpureOperation<S3> for Merged<S1,S2,S3,Func,Op1,Op2> {
    fn get_state(&self) -> S3 {
        (self.func)(self.op1.get_state(),self.op2.get_state())
    }
    fn call_effect(&mut self) {
        self.op1.call_effect();
        self.op2.call_effect();
    }
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

    fn call_effect(&mut self) {
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
    fn get_state(&self) -> (S, Box<(dyn FnMut(S))>) {
        todo!()
    }

    fn call_effect(&mut self) {
        todo!()
    }

}

pub struct Effect<S, Op: ImpureOperation<S>,C: FnMut(S)> {
    callback: C,
    op: Op,
    __mark: PhantomData<S>,
}

struct Empty {

}

impl ImpureOperation<()> for Empty {
    fn get_state(&self) -> () {
        todo!()
    }

    fn call_effect(&mut self) {
        todo!()
    }
}

struct OperationTodo;
impl<T> ImpureOperation<T> for OperationTodo {
    fn get_state(&self) -> T {
        todo!()
    }

    fn call_effect(&mut self) {
        todo!()
    }
}


impl <C: FnMut(())>Effect<(),Empty,C> {
    pub fn new(callback: C) -> Self {
        Self {
            callback,
            op: Empty {},
            __mark: PhantomData
        }
    }
}

impl <S, Op: ImpureOperation<S>,C:FnMut(S)>Effect<S,Op,C> {
    pub fn with(callback: C,op: Op) -> Effect<S,Op,C> {
        Effect {
            callback,
            op,
            __mark: PhantomData
        }
    }
}

impl <S, Op: ImpureOperation<S>,C:FnMut(S)>ImpureOperation<S> for Effect<S,Op,C> {
    fn get_state(&self) -> S {
        todo!()
    }

    fn call_effect(&mut self) {
        todo!()
    }
}

pub struct SampleContainerProps {
    pub name: String,
}

fn set_interval_mock(callback: Box<dyn FnMut()>, delay: u32) {
    todo!()
}

pub fn sample_effect() -> impl ImpureOperation<i32> {
    let state = State::new(0, |cur,plus: i32| { cur + plus });
    let effect = Effect::with(|(_,mut update)| {
        set_interval_mock(Box::new(move || {
            update(1);
        }), 1000);
    }, state);
    effect.reform(|(state,_)| state)
}

pub fn sample_effect2() -> impl ImpureOperation<i32> {
    let state = State::new(0, |cur,plus: i32| { cur + plus });
    let effect = Effect::with(|(_,mut update)| {
        set_interval_mock(Box::new(move || {
            update(1);
        }), 100);
    }, state);
    effect.reform(|(state,_)| state)
}

pub fn sample_effect_merge(props: SampleContainerProps) -> impl ImpureOperation<i32> {
    let op1 = sample_effect();
    let op2 = sample_effect2();
    op1.merge(op2, |s1,s2| s1 + s2)
}

pub fn connect<R: Fn() -> UI>(render: R){

}

