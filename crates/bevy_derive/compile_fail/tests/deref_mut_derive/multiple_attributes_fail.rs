use bevy_derive::DerefMut;
use core::ops::Deref;

#[derive(DerefMut)]
struct TupleStruct(
    #[deref] usize,
    #[deref] String
    //~^ ERROR: can only be used on a single field
);

impl Deref for TupleStruct {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.1
    }
}

#[derive(DerefMut)]
struct Struct {
    #[deref]
    foo: usize,
    #[deref]
    //~^ ERROR: can only be used on a single field
    bar: String,
}

impl Deref for Struct {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.bar
    }
}
