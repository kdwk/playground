use bon::Builder;

struct Context {
    context: Vec<Context>,
}

impl Context {}

trait Compose: PartialEq<Self> {
    fn build(self, context: ()) -> impl Compose;
    fn needsRebuild(&self, old_widget: &Self) -> bool {
        false
    }
}

#[derive(Builder, PartialEq)]
struct Box<C>
where
    C: Compose,
{
    child: C,
}

impl<C: Compose> Compose for Box<C> {
    fn build(self, context: ()) -> impl Compose {
        self.child
    }
    fn needsRebuild(&self, old_widget: &Self) -> bool {
        self != old_widget
    }
}

#[derive(Debug, PartialEq)]
struct Rect;

impl Compose for Rect {
    fn build(self, context: ()) -> impl Compose {
        Rect
    }
}
