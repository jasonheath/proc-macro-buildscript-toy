use strum::{EnumCount, IntoEnumIterator};
use strum_macros::{EnumCount as EnumCountMacro, EnumIter};

#[derive(Debug, EnumCountMacro, EnumIter)]
enum MetaSyntaticVariables {
    Foo,
    Bar,
    Baz,
    Qux,
    Quux,
    Quuux
}

fn main() {
    println!("There are {} metasyntatic variables.", MetaSyntaticVariables::COUNT);
    MetaSyntaticVariables::iter().count();
}