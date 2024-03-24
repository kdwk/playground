use std::{any::Any, fmt::Debug, marker::PhantomData, ops::Index, slice::Iter};

use crate::whoops::attempt;

pub struct Step<'a, Input, Output>(String, Box<dyn Runnable<Input, Output> + 'a>);

impl<'a, Input, Output> Runnable<Input, Output> for Step<'a, Input, Output> {
    fn run(&self, arg: Input) -> Output {
        self.1.run(arg)
    }
}

impl<'a, Input, Output> Step<'a, Input, Output> {
    pub fn action<Closure>(alias: impl ToString, closure: Closure) -> Self
    where
        Closure: Runnable<Input, Output> + 'a,
    {
        Self(alias.to_string(), Box::new(closure))
    }
}

pub struct Recipe<'a, Ingredients, Outcome> {
    pub initial_step: Step<'a, Ingredients, Outcome>,
    pub steps: Vec<Step<'a, Outcome, Outcome>>,
}

impl<'a, Ingredients, Outcome> Recipe<'a, Ingredients, Outcome> {
    pub fn initially<Closure: Runnable<Ingredients, Outcome> + 'a>(
        alias: impl ToString,
        action: Closure,
    ) -> Self {
        Self {
            initial_step: Step::action(alias.to_string(), action),
            steps: vec![],
        }
    }
    pub fn then<Closure: Runnable<Outcome, Outcome> + 'a>(
        mut self,
        alias: impl ToString,
        action: Closure,
    ) -> Self {
        self.steps.push(Step::action(alias.to_string(), action));
        self
    }
    fn replace_one(&mut self, index: usize, step: Step<'a, Outcome, Outcome>) {
        self.steps[index] = step;
    }
    pub fn replace<Closure: Runnable<Outcome, Outcome> + 'a>(
        &mut self,
        alias: impl ToString,
        action: Closure,
    ) -> &mut Self {
        let mut index = None;
        for (i, step) in &mut self.steps.iter().enumerate() {
            if step.0 == alias.to_string() {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            self.replace_one(index, Step::action(alias, action));
        }
        self
    }
    pub fn replace_all<Closure: Runnable<Outcome, Outcome> + 'a + Clone>(
        &mut self,
        alias: impl ToString,
        action: Closure,
    ) -> &mut Self {
        let mut indices = vec![];
        for (i, step) in &mut self.steps.iter().enumerate() {
            if step.0 == alias.to_string() {
                indices.push(i);
            }
        }
        for index in indices {
            self.replace_one(
                index,
                Step::action(alias.to_string().clone(), action.clone()),
            );
        }
        self
    }
    pub fn replace_initial<Closure: Runnable<Ingredients, Outcome> + 'a>(
        &mut self,
        action: Closure,
    ) -> &mut Self {
        self.initial_step = Step::action(self.initial_step.0.clone(), action);
        self
    }
    pub fn remove(&mut self, alias: impl ToString) -> &mut Self {
        let mut index = None;
        for (i, step) in &mut self.steps.iter().enumerate() {
            if step.0 == alias.to_string() {
                index = Some(i);
                break;
            }
        }
        if let Some(index) = index {
            self.steps.remove(index);
        }
        self
    }
    pub fn remove_all(&mut self, alias: impl ToString) -> &mut Self {
        let mut indices = vec![];
        for (i, step) in &mut self.steps.iter().enumerate() {
            if step.0 == alias.to_string() {
                indices.push(i);
            }
        }
        for index in indices {
            self.steps.remove(index);
        }
        self
    }
    // TODO: requires Step to be Clone
    // pub fn get(&mut self, alias: impl ToString) -> Step<'a, Outcome, Outcome> {
    //     let mut index = None;
    //     for (i, step) in &mut self.steps.iter().enumerate() {
    //         if step.0 == alias.to_string() {
    //             index = Some(i);
    //             break;
    //         }
    //     }
    //     if let Some(index) = index {
    //         self.steps[index]
    //     } else {
    //         Step::action("identity", |input: Outcome| input)
    //     }
    // }
}

impl<'a, Ingredients, Outcome> Runnable<Ingredients, Outcome> for Recipe<'a, Ingredients, Outcome> {
    fn run(&self, ingredients: Ingredients) -> Outcome {
        let mut intermediate = self.initial_step.run(ingredients);
        for step in &self.steps {
            intermediate = step.run(intermediate);
        }
        intermediate
    }
}

// impl<'a, Str: ToString, Ingredients, Outcome> Index<Str> for Recipe<'a, Ingredients, Outcome> {
//     type Output = Step<'a, Outcome, Outcome>;
//     fn index(&self, index: Str) -> &Self::Output {
//         let mut i = None;
//         let step = Step::action(index.to_string().clone(), |input: Outcome| input);
//         for (j, step) in self.steps.iter().enumerate() {
//             if step.0 == index.to_string() {
//                 i = Some(j);
//                 break;
//             }
//         }
//         if let Some(i) = i {
//             &self.steps[i]
//         } else {
//             &step
//         }
//     }
// }

pub trait Pipe<Closure, Return>
where
    Self: Sized,
    Closure: FnOnce(Self) -> Return,
{
    fn pipe(self, closure: Closure) -> Return;
}

impl<T, Closure, Return> Pipe<Closure, Return> for T
where
    Self: Sized,
    Closure: FnOnce(Self) -> Return,
{
    fn pipe(self, closure: Closure) -> Return {
        closure(self)
    }
}

pub trait Log {
    fn log(self) -> Self;
}

impl<T> Log for T
where
    T: Debug,
{
    fn log(self) -> Self {
        println!("{self:?}");
        self
    }
}

pub trait Runnable<Arg, Return> {
    fn run(&self, arg: Arg) -> Return;
}

impl<Closure, Arg, Return> Runnable<Arg, Return> for Closure
where
    Closure: Fn(Arg) -> Return,
{
    fn run(&self, arg: Arg) -> Return {
        self(arg)
    }
}

pub trait Pass<Arg: Clone> {
    fn pass(self, arg: Arg) -> impl Fn();
}

impl<Closure, Arg: Clone, Return> Pass<Arg> for Closure
where
    Closure: Fn(Arg) -> Return + Clone + 'static,
{
    fn pass(self, arg: Arg) -> impl Fn() {
        move || {
            _ = self.clone().run(arg.clone());
        }
    }
}

pub trait Discard {
    fn discard(self);
}

impl<T> Discard for T {
    fn discard(self) {
        _ = self;
    }
}

pub mod example {
    use std::fmt::{Debug, Display};

    use super::{Log, Pipe, Recipe, Runnable};
    #[derive(Debug)]
    pub struct BoxInternal(i32, i32, f32);
    pub struct Box<'a>(Recipe<'a, (i32, i32, f32), BoxInternal>);
    impl<'a> Box<'a> {
        fn new() -> Self {
            Self(Recipe::initially(
                "new",
                |(width, height, rotation): (i32, i32, f32)| BoxInternal(width, height, rotation),
            ))
        }
        fn width(self, value: i32) -> Self {
            Self(self.0.then("setWidth", move |mut b: BoxInternal| {
                b.0 = value;
                b
            }))
        }
        fn height(self, value: i32) -> Self {
            Self(self.0.then("setHeight", move |mut b: BoxInternal| {
                b.1 = value;
                b
            }))
        }
        fn rotate(self, degrees: f32) -> Self {
            Self(self.0.then("setRotation", move |mut b: BoxInternal| {
                b.2 = degrees;
                b
            }))
        }
    }
    impl<'a> Debug for Box<'a> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let b = self.0.run((0, 0, 0.0));
            f.pad(format!("{:?}", b).as_str())
        }
    }
    pub fn test() {
        Box::new().width(6).height(7).rotate(45.0).log();
        Box::new().width(6).height(7).log();
    }
}